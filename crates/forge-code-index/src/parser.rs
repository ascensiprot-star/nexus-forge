use forge_core::traits::{Symbol, SymbolKind};

pub fn extract_symbols(content: &str, language: &str, file_path: &str) -> Vec<Symbol> {
    let mut symbols = Vec::new();

    for (line_num, line) in content.lines().enumerate() {
        let trimmed = line.trim();
        let line_1based = (line_num + 1) as u32;

        match language {
            "rs" => extract_rust_symbols(trimmed, file_path, line_1based, &mut symbols),
            "py" => extract_python_symbols(trimmed, file_path, line_1based, &mut symbols),
            "ts" | "tsx" | "js" | "jsx" => {
                extract_typescript_symbols(trimmed, file_path, line_1based, &mut symbols)
            }
            "go" => extract_go_symbols(trimmed, file_path, line_1based, &mut symbols),
            _ => {}
        }
    }

    symbols
}

fn extract_rust_symbols(line: &str, file_path: &str, line_num: u32, symbols: &mut Vec<Symbol>) {
    if let Some(name) = extract_after_keyword(line, "fn ") {
        symbols.push(Symbol {
            name,
            kind: SymbolKind::Function,
            file_path: file_path.to_string(),
            start_line: line_num,
            end_line: line_num,
            signature: Some(line.to_string()),
        });
    }
    if let Some(name) = extract_after_keyword(line, "struct ") {
        symbols.push(Symbol {
            name,
            kind: SymbolKind::Class,
            file_path: file_path.to_string(),
            start_line: line_num,
            end_line: line_num,
            signature: Some(line.to_string()),
        });
    }
    if let Some(name) = extract_after_keyword(line, "enum ") {
        symbols.push(Symbol {
            name,
            kind: SymbolKind::Enum,
            file_path: file_path.to_string(),
            start_line: line_num,
            end_line: line_num,
            signature: Some(line.to_string()),
        });
    }
    if let Some(name) = extract_after_keyword(line, "trait ") {
        symbols.push(Symbol {
            name,
            kind: SymbolKind::Interface,
            file_path: file_path.to_string(),
            start_line: line_num,
            end_line: line_num,
            signature: Some(line.to_string()),
        });
    }
    if let Some(name) = extract_after_keyword(line, "mod ") {
        let name = name.trim_end_matches(';');
        symbols.push(Symbol {
            name: name.to_string(),
            kind: SymbolKind::Module,
            file_path: file_path.to_string(),
            start_line: line_num,
            end_line: line_num,
            signature: None,
        });
    }
}

fn extract_python_symbols(line: &str, file_path: &str, line_num: u32, symbols: &mut Vec<Symbol>) {
    if let Some(name) = extract_after_keyword(line, "def ") {
        symbols.push(Symbol {
            name,
            kind: SymbolKind::Function,
            file_path: file_path.to_string(),
            start_line: line_num,
            end_line: line_num,
            signature: Some(line.to_string()),
        });
    }
    if let Some(name) = extract_after_keyword(line, "class ") {
        symbols.push(Symbol {
            name,
            kind: SymbolKind::Class,
            file_path: file_path.to_string(),
            start_line: line_num,
            end_line: line_num,
            signature: Some(line.to_string()),
        });
    }
}

fn extract_typescript_symbols(
    line: &str,
    file_path: &str,
    line_num: u32,
    symbols: &mut Vec<Symbol>,
) {
    if let Some(name) = extract_after_keyword(line, "function ") {
        symbols.push(Symbol {
            name,
            kind: SymbolKind::Function,
            file_path: file_path.to_string(),
            start_line: line_num,
            end_line: line_num,
            signature: Some(line.to_string()),
        });
    }
    if let Some(name) = extract_after_keyword(line, "class ") {
        symbols.push(Symbol {
            name,
            kind: SymbolKind::Class,
            file_path: file_path.to_string(),
            start_line: line_num,
            end_line: line_num,
            signature: Some(line.to_string()),
        });
    }
    if let Some(name) = extract_after_keyword(line, "interface ") {
        symbols.push(Symbol {
            name,
            kind: SymbolKind::Interface,
            file_path: file_path.to_string(),
            start_line: line_num,
            end_line: line_num,
            signature: Some(line.to_string()),
        });
    }
    if let Some(name) = extract_after_keyword(line, "enum ") {
        symbols.push(Symbol {
            name,
            kind: SymbolKind::Enum,
            file_path: file_path.to_string(),
            start_line: line_num,
            end_line: line_num,
            signature: Some(line.to_string()),
        });
    }
}

fn extract_go_symbols(line: &str, file_path: &str, line_num: u32, symbols: &mut Vec<Symbol>) {
    if let Some(name) = extract_after_keyword(line, "func ") {
        symbols.push(Symbol {
            name,
            kind: SymbolKind::Function,
            file_path: file_path.to_string(),
            start_line: line_num,
            end_line: line_num,
            signature: Some(line.to_string()),
        });
    }
    if let Some(name) = extract_after_keyword(line, "type ") {
        symbols.push(Symbol {
            name,
            kind: SymbolKind::Type,
            file_path: file_path.to_string(),
            start_line: line_num,
            end_line: line_num,
            signature: Some(line.to_string()),
        });
    }
}

fn extract_after_keyword(line: &str, keyword: &str) -> Option<String> {
    let stripped = line
        .trim_start_matches("pub ")
        .trim_start_matches("pub(crate) ")
        .trim_start_matches("async ")
        .trim_start_matches("export ")
        .trim_start_matches("default ")
        .trim_start_matches("const ")
        .trim_start_matches("unsafe ");

    if stripped.starts_with(keyword) {
        let rest = &stripped[keyword.len()..];
        let name = rest
            .split(|c: char| !c.is_alphanumeric() && c != '_')
            .next()
            .unwrap_or("")
            .to_string();
        if !name.is_empty() {
            return Some(name);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rust_function_extraction() {
        let symbols = extract_symbols("pub fn my_function(x: i32) -> i32 {", "rs", "test.rs");
        assert_eq!(symbols.len(), 1);
        assert_eq!(symbols[0].name, "my_function");
        assert_eq!(symbols[0].kind, SymbolKind::Function);
    }

    #[test]
    fn test_python_class_extraction() {
        let symbols = extract_symbols("class MyClass:\n    pass", "py", "test.py");
        assert_eq!(symbols.len(), 1);
        assert_eq!(symbols[0].name, "MyClass");
        assert_eq!(symbols[0].kind, SymbolKind::Class);
    }

    #[test]
    fn test_typescript_function() {
        let symbols = extract_symbols("export function handleClick(e: Event) {", "ts", "test.ts");
        assert_eq!(symbols.len(), 1);
        assert_eq!(symbols[0].name, "handleClick");
    }
}
