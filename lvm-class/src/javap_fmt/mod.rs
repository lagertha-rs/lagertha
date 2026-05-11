mod attribute;
mod class;
mod constant_pool;
mod field;
mod flags;
mod instruction;
mod method;

/// Transforms a class name from internal JVM format to Java-like format.
/// Trims leading 'L' and trailing ';', replaces '/' with '.'.
fn fmt_class_name(name: &str) -> String {
    name.trim_start_matches('L')
        .trim_end_matches(';')
        .replace('/', ".")
}

/// Formats a method name for javap output.
/// Wraps `<init>` in quotes.
fn fmt_method_name(name: &str) -> String {
    match name {
        "<init>" => format!("\"{}\"", name),
        _ => name.to_string(),
    }
}
