use std::fmt::{Display, Formatter};
use std::path::Path;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TestCategory {
    Success,
    Error,
}

impl Display for TestCategory {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(match self {
            Self::Success => "Success",
            Self::Error => "Error",
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct TestMetadata {
    pub feature: String,
    pub description: String,
    pub category: TestCategory,
}

#[derive(Debug, PartialEq, Eq)]
pub struct MetadataError {
    message: String,
}

impl MetadataError {
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl Display for MetadataError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl std::error::Error for MetadataError {}

pub fn parse_test_metadata(path: &Path, source: &str) -> Result<TestMetadata, MetadataError> {
    let comment = metadata_comment(path)?;
    let lines = source.lines().collect::<Vec<_>>();
    if lines.len() < 3 {
        return Err(error(format!(
            "{}: fixture must start with three metadata comments",
            path.display()
        )));
    }

    let feature = metadata_value(path, lines[0], comment, "feature")?;
    let description = metadata_value(path, lines[1], comment, "description")?;
    let category = metadata_value(path, lines[2], comment, "category")?;
    if lines
        .get(3)
        .is_some_and(|line| line.starts_with(&format!("{comment} @test ")))
    {
        return Err(error(format!(
            "{}: fixture must have exactly three metadata comments",
            path.display()
        )));
    }
    if feature.trim().is_empty() {
        return Err(error(format!(
            "{}: feature must not be empty",
            path.display()
        )));
    }
    validate_feature_id(&feature).map_err(|validation| {
        error(format!(
            "{}: invalid feature ID {feature:?}: {validation}",
            path.display()
        ))
    })?;
    if description.trim().is_empty() {
        return Err(error(format!(
            "{}: description must not be empty",
            path.display()
        )));
    }
    let category = match category.as_str() {
        "success" => TestCategory::Success,
        "error" => TestCategory::Error,
        value => {
            return Err(error(format!(
                "{}: unsupported test category {value:?}",
                path.display()
            )));
        }
    };

    Ok(TestMetadata {
        feature,
        description,
        category,
    })
}

pub fn validate_feature_id(id: &str) -> Result<(), MetadataError> {
    let segments = id.split('.').collect::<Vec<_>>();
    if segments.len() < 2 {
        return Err(error("feature ID must contain at least two segments"));
    }
    for segment in segments {
        let bytes = segment.as_bytes();
        if bytes.is_empty() {
            return Err(error("feature ID contains an empty segment"));
        }
        if !bytes[0].is_ascii_lowercase() {
            return Err(error(format!(
                "feature ID segment {segment:?} must start with a lowercase ASCII letter"
            )));
        }
        if !bytes.last().is_some_and(u8::is_ascii_alphanumeric) {
            return Err(error(format!(
                "feature ID segment {segment:?} must end with a lowercase ASCII letter or digit"
            )));
        }
        let mut previous_hyphen = false;
        for byte in bytes {
            let valid = byte.is_ascii_lowercase() || byte.is_ascii_digit() || *byte == b'-';
            if !valid {
                return Err(error(format!(
                    "feature ID segment {segment:?} contains an unsupported character"
                )));
            }
            if *byte == b'-' && previous_hyphen {
                return Err(error(format!(
                    "feature ID segment {segment:?} contains consecutive hyphens"
                )));
            }
            previous_hyphen = *byte == b'-';
        }
    }
    Ok(())
}

fn metadata_comment(path: &Path) -> Result<&'static str, MetadataError> {
    match path.extension().and_then(|extension| extension.to_str()) {
        Some("java") => Ok("//"),
        Some("rns") => Ok(";"),
        _ => Err(error(format!(
            "unsupported fixture extension: {}",
            path.display()
        ))),
    }
}

fn metadata_value(
    path: &Path,
    line: &str,
    comment: &str,
    field: &str,
) -> Result<String, MetadataError> {
    let prefix = format!("{comment} @test {field} = \"");
    let encoded = line
        .strip_prefix(&prefix)
        .and_then(|value| value.strip_suffix('"'))
        .ok_or_else(|| {
            error(format!(
                "{}: expected `{comment} @test {field} = \"...\"`",
                path.display()
            ))
        })?;

    let mut value = String::new();
    let mut chars = encoded.chars();
    while let Some(character) = chars.next() {
        match character {
            '\\' => match chars.next() {
                Some('"') => value.push('"'),
                Some('\\') => value.push('\\'),
                _ => {
                    return Err(error(format!(
                        "{}: {field} contains an invalid escape",
                        path.display()
                    )));
                }
            },
            '"' => {
                return Err(error(format!(
                    "{}: {field} contains an unescaped quote",
                    path.display()
                )));
            }
            character => value.push(character),
        }
    }
    Ok(value)
}

fn error(message: impl Into<String>) -> MetadataError {
    MetadataError {
        message: message.into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_java_metadata() {
        let source = r#"// @test feature = "execution.integer.arithmetic"
// @test description = "Adds two values."
// @test category = "success"

class ArithmeticTest {}
"#;

        let metadata = parse_test_metadata(Path::new("ArithmeticTest.java"), source).unwrap();

        assert_eq!(metadata.feature, "execution.integer.arithmetic");
        assert_eq!(metadata.description, "Adds two values.");
        assert_eq!(metadata.category, TestCategory::Success);
    }

    #[test]
    fn parses_rns_metadata_with_escapes() {
        let source = r#"; @test feature = "class-format.interface-flags"
; @test description = "Rejects a \"bad\" interface."
; @test category = "error"

.class_end
"#;

        let metadata = parse_test_metadata(Path::new("BadInterfaceTest.rns"), source).unwrap();

        assert_eq!(metadata.description, "Rejects a \"bad\" interface.");
        assert_eq!(metadata.category, TestCategory::Error);
    }

    #[test]
    fn rejects_noncanonical_feature_ids() {
        for id in [
            "arithmetic",
            "Execution.integer",
            "execution.integer_arithmetic",
            "execution..arithmetic",
            "execution.integer--arithmetic",
            "execution.integer-",
        ] {
            assert!(validate_feature_id(id).is_err(), "accepted {id}");
        }
    }

    #[test]
    fn accepts_canonical_feature_ids() {
        for id in [
            "execution.integer.arithmetic",
            "class-format.interface-flags",
            "natives.system.arraycopy2",
        ] {
            validate_feature_id(id).unwrap();
        }
    }
}
