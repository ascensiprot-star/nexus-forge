pub fn supported_extensions() -> &'static [&'static str] {
    &[
        "rs", "py", "ts", "tsx", "js", "jsx", "go", "java", "cs", "rb", "php", "swift", "kt",
    ]
}

pub fn language_name(extension: &str) -> &'static str {
    match extension {
        "rs" => "rust",
        "py" => "python",
        "ts" | "tsx" => "typescript",
        "js" | "jsx" => "javascript",
        "go" => "go",
        "java" => "java",
        "cs" => "csharp",
        "rb" => "ruby",
        "php" => "php",
        "swift" => "swift",
        "kt" => "kotlin",
        _ => "unknown",
    }
}
