use std::path::Path;

pub fn extract_imports(content: &str, file_path: &str) -> Vec<String> {
    let ext = Path::new(file_path)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("");

    match ext {
        "rs" => extract_rust_imports(content),
        "py" => extract_python_imports(content),
        "ts" | "tsx" | "js" | "jsx" => extract_ts_imports(content),
        "go" => extract_go_imports(content),
        _ => vec![],
    }
}

fn extract_rust_imports(content: &str) -> Vec<String> {
    content
        .lines()
        .filter_map(|line| {
            let trimmed = line.trim();
            if trimmed.starts_with("use ") {
                let import = trimmed
                    .trim_start_matches("use ")
                    .trim_end_matches(';')
                    .split("::")
                    .next()
                    .unwrap_or("")
                    .to_string();
                if !import.is_empty() {
                    Some(import)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect()
}

fn extract_python_imports(content: &str) -> Vec<String> {
    content
        .lines()
        .filter_map(|line| {
            let trimmed = line.trim();
            if trimmed.starts_with("import ") {
                Some(
                    trimmed
                        .trim_start_matches("import ")
                        .split_whitespace()
                        .next()
                        .unwrap_or("")
                        .to_string(),
                )
            } else if trimmed.starts_with("from ") {
                Some(
                    trimmed
                        .trim_start_matches("from ")
                        .split_whitespace()
                        .next()
                        .unwrap_or("")
                        .to_string(),
                )
            } else {
                None
            }
        })
        .filter(|s| !s.is_empty())
        .collect()
}

fn extract_ts_imports(content: &str) -> Vec<String> {
    content
        .lines()
        .filter_map(|line| {
            let trimmed = line.trim();
            if trimmed.contains("import ") && trimmed.contains("from ") {
                let from_part = trimmed.split("from ").nth(1)?;
                let module = from_part
                    .trim()
                    .trim_matches(|c| c == '\'' || c == '"' || c == ';');
                Some(module.to_string())
            } else if trimmed.starts_with("import ") {
                let module = trimmed
                    .trim_start_matches("import ")
                    .trim_matches(|c: char| c == '\'' || c == '"' || c == ';');
                Some(module.to_string())
            } else {
                None
            }
        })
        .filter(|s| !s.is_empty())
        .collect()
}

fn extract_go_imports(content: &str) -> Vec<String> {
    let mut imports = Vec::new();
    let mut in_import_block = false;

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed == "import (" {
            in_import_block = true;
            continue;
        }
        if in_import_block && trimmed == ")" {
            in_import_block = false;
            continue;
        }
        if in_import_block {
            let import = trimmed.trim_matches('"');
            if !import.is_empty() {
                imports.push(import.to_string());
            }
        }
        if trimmed.starts_with("import \"") {
            let import = trimmed.trim_start_matches("import ").trim_matches('"');
            imports.push(import.to_string());
        }
    }

    imports
}
