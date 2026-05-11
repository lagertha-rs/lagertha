use crate::instruction::INSTRUCTION_SPECS;

fn levenshtein_distance(s1: &str, s2: &str) -> usize {
    let s1_len = s1.chars().count();
    let s2_len = s2.chars().count();

    if s1_len == 0 {
        return s2_len;
    }
    if s2_len == 0 {
        return s1_len;
    }

    let mut prev = (0..=s2_len).collect::<Vec<_>>();
    let mut curr = vec![0; s2_len + 1];

    for (i, c1) in s1.chars().enumerate() {
        curr[0] = i + 1;
        for (j, c2) in s2.chars().enumerate() {
            let cost = if c1 == c2 { 0 } else { 1 };
            curr[j + 1] =
                std::cmp::min(std::cmp::min(curr[j] + 1, prev[j + 1] + 1), prev[j] + cost);
        }
        prev.copy_from_slice(&curr);
    }

    prev[s2_len]
}

const DIRECTIVES: [&str; 8] = [
    ".class",
    ".class_end",
    ".super",
    ".method",
    ".method_end",
    ".code",
    ".code_end",
    ".annotation",
];

const TYPE_HINTS: [&str; 18] = [
    "cp_idx",
    "utf8",
    "int",
    "string",
    "class",
    "methodref",
    "fieldref",
    "interfaceMethodref",
    "float",
    "long",
    "double",
    "nameAndType",
    "methodHandle",
    "methodType",
    "dynamic",
    "invokeDynamic",
    "module",
    "package",
];

pub fn closest_match<'a>(
    input: &str,
    candidates: impl IntoIterator<Item = &'a str>,
    max_distance: usize,
) -> Option<&'a str> {
    let mut best: Option<&str> = None;
    let mut best_dist = usize::MAX;
    for candidate in candidates {
        let dist = levenshtein_distance(input, candidate);
        if dist < best_dist && dist <= max_distance {
            best_dist = dist;
            best = Some(candidate);
        }
    }
    best
}

pub fn closest_directive(input: &str) -> Option<&'static str> {
    closest_match(input, DIRECTIVES, 2)
}

pub fn closest_type_hint(input: &str) -> Option<&'static str> {
    closest_match(input, TYPE_HINTS, 2)
}

pub fn closest_instruction(input: &str) -> Option<&'static str> {
    closest_match(input, INSTRUCTION_SPECS.keys().copied(), 2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exact_match_directive() {
        assert_eq!(closest_directive(".class"), Some(".class"));
    }

    #[test]
    fn close_directive_typo() {
        assert_eq!(closest_directive(".clas"), Some(".class"));
        assert_eq!(closest_directive(".supr"), Some(".super"));
    }

    #[test]
    fn far_directive_returns_none() {
        assert_eq!(closest_directive(".zzzzz"), None);
    }

    #[test]
    fn exact_match_type_hint() {
        assert_eq!(closest_type_hint("cp_index"), Some("cp_idx"));
        assert_eq!(closest_type_hint("cp_idx"), Some("cp_idx"));
        assert_eq!(closest_type_hint("utf8"), Some("utf8"));
        assert_eq!(
            closest_type_hint("interfaceMethodref"),
            Some("interfaceMethodref")
        );
    }

    #[test]
    fn close_type_hint_typo() {
        assert_eq!(closest_type_hint("cp_ix"), Some("cp_idx"));
        assert_eq!(closest_type_hint("strig"), Some("string"));
        assert_eq!(closest_type_hint("flot"), Some("float"));
        assert_eq!(closest_type_hint("dubble"), Some("double"));
    }

    #[test]
    fn far_type_hint_returns_none() {
        assert_eq!(closest_type_hint("foobar"), None);
    }

    #[test]
    fn generic_closest_match_empty_candidates() {
        let empty: Vec<&str> = vec![];
        assert_eq!(closest_match("hello", empty, 2), None);
    }

    #[test]
    fn generic_closest_match_picks_best() {
        let candidates = vec!["cat", "car", "bat"];
        assert_eq!(closest_match("cap", candidates, 2), Some("cat"));
    }
}
