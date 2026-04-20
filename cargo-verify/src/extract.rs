//! Docstring extraction using syn
//! Finds `/// ```datalog` blocks in Rust source files

use anyhow::{Context, Result};
use regex::Regex;
use std::fs;
use std::path::Path;
use syn::{visit::Visit, ItemFn, Lit, LitStr};

/// An extracted Datalog declaration
#[derive(Debug, Clone)]
pub struct Declaration {
    pub file: String,
    pub line: usize,
    pub datalog: String,
}

/// Extract datalog declarations from all .rs files in a directory
pub fn extract_declarations(manifest_path: &Path) -> Result<Vec<Declaration>> {
    let mut declarations = Vec::new();
    
    // Find all .rs files in the project
    let src_dir = manifest_path
        .parent()
        .context("manifest has no parent directory")?
        .join("src");
    
    if !src_dir.exists() {
        // Try finding src files differently
        let base_dir = manifest_path.parent().unwrap_or(Path::new("."));
        find_rust_files(base_dir, &mut declarations)?;
    } else {
        find_rust_files(&src_dir, &mut declarations)?;
    }
    
    Ok(declarations)
}

fn find_rust_files(dir: &Path, declarations: &mut Vec<Declaration>) -> Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_dir() {
            find_rust_files(&path, declarations)?;
        } else if path.extension().map(|e| e == "rs").unwrap_or(false) {
            extract_from_file(&path, declarations)?;
        }
    }
    Ok(())
}

fn extract_from_file(path: &Path, declarations: &mut Vec<Declaration>) -> Result<()> {
    let content = fs::read_to_string(path)?;
    let file_str = path.to_string_lossy().to_string();
    
    // Parse the file with syn
    let syntax = syn::parse_file(&content)
        .map_err(|e| anyhow::anyhow!("Failed to parse {}: {}", path.display(), e))?;
    
    // Use regex to find docstring datalog blocks
    // This is simpler than full syn traversal for doc comments
    let datalog_regex = Regex::new(
        r"(?m)^\s*///\s*```datalog\s*\n(.*?)///\s*```",
    )?;
    
    for cap in datalog_regex.captures_iter(&content) {
        let datalog_content = &cap[1];
        let start_pos = cap.get(0).unwrap().start();
        let line = content[..start_pos].lines().count() + 1;
        
        // Clean up the datalog content (remove /// prefixes)
        let cleaned = datalog_content
            .lines()
            .map(|line| {
                line.trim_start()
                    .strip_prefix("///")
                    .unwrap_or(line)
                    .trim_start()
            })
            .collect::<Vec<_>>()
            .join("\n");
        
        declarations.push(Declaration {
            file: file_str.clone(),
            line,
            datalog: cleaned,
        });
    }
    
    // Also extract using syn for more structured approach
    let mut extractor = DocExtractor {
        declarations: Vec::new(),
        file: file_str.clone(),
    };
    extractor.visit_file(&syntax);
    declarations.extend(extractor.declarations);
    
    Ok(())
}

/// Syn visitor to extract doc comments
struct DocExtractor {
    declarations: Vec<Declaration>,
    file: String,
}

impl Visit<'_> for DocExtractor {
    fn visit_item_fn(&mut self, node: &ItemFn) {
        for attr in &node.attrs {
            if let syn::Meta::NameValue(meta) = &attr.meta {
                if meta.path.is_ident("doc") {
                    if let syn::Expr::Lit(expr_lit) = &meta.value {
                        if let Lit::Str(lit_str) = &expr_lit.lit {
                            self.process_doc_literal(lit_str);
                        }
                    }
                }
            }
        }
        
        syn::visit::visit_item_fn(self, node);
    }
}

impl DocExtractor {
    fn process_doc_literal(&mut self, lit: &LitStr) {
        let doc = lit.value();
        
        // Look for datalog block
        if doc.contains("```datalog") {
            if let Some(start) = doc.find("```datalog") {
                if let Some(end) = doc[start + 10..].find("```") {
                    let datalog = &doc[start + 10..start + 10 + end];
                    
                    self.declarations.push(Declaration {
                        file: self.file.clone(),
                        line: 0, // Line info hard to get from syn span
                        datalog: datalog.trim().to_string(),
                    });
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;
    
    #[test]
    fn test_extract_datalog_block() {
        let content = r#"
/// Some function
/// ```datalog
/// test("my_test", pass).
/// coverage("my_test", 100.0).
/// ```
pub fn my_function() {}
"#;
        
        let mut file = NamedTempFile::new().unwrap();
        file.write_all(content.as_bytes()).unwrap();
        
        let mut decls = Vec::new();
        extract_from_file(file.path(), &mut decls).unwrap();
        
        assert_eq!(decls.len(), 2); // Both regex and syn should find it
        assert!(decls.iter().any(|d| d.datalog.contains("test(")));
    }
}