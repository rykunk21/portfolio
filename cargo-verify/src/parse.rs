//! Datalog parser for docstring declarations
//! Parses test(), coverage(), complexity() facts

use anyhow::Result;
use regex::Regex;
use std::collections::HashMap;

/// Types of declarations that can be extracted
#[derive(Debug, Clone, PartialEq)]
pub enum Fact {
    /// test(test_name, pass)
    Test { name: String, should_pass: bool },
    
    /// coverage(test_name, percentage)
    Coverage { name: String, percentage: f64 },
    
    /// complexity(function, score)
    Complexity { function: String, score: u32 },
}

/// Parsed datalog content
#[derive(Debug, Clone, Default)]
pub struct ParsedDatalog {
    pub facts: Vec<Fact>,
    pub errors: Vec<String>,
}

/// Parse a datalog string into facts
pub fn parse_datalog(content: &str) -> Result<ParsedDatalog> {
    let mut result = ParsedDatalog::default();
    
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with("%") {
            continue; // Skip empty lines and comments
        }
        
        // Remove trailing period but keep for validation
        let line = line.trim_end_matches('.');
        
        // Try to parse as test fact
        if let Some(fact) = parse_test_fact(line) {
            result.facts.push(fact);
            continue;
        }
        
        // Try to parse as coverage fact
        if let Some(fact) = parse_coverage_fact(line) {
            result.facts.push(fact);
            continue;
        }
        
        // Try to parse as complexity fact
        if let Some(fact) = parse_complexity_fact(line) {
            result.facts.push(fact);
            continue;
        }
        
        // Unknown fact type
        result.errors.push(format!("Unknown datalog statement: {}", line));
    }
    
    Ok(result)
}

fn parse_test_fact(line: &str) -> Option<Fact> {
    // Pattern: test("name", pass). or test("name", fail).
    let re = Regex::new(r#"^test\s*\(\s*"([^"]+)"\s*,\s*(pass|fail)\s*\)"#).ok()?;
    
    let cap = re.captures(line)?;
    let name = cap.get(1)?.as_str().to_string();
    let result = cap.get(2)?.as_str();
    let should_pass = result == "pass";
    
    Some(Fact::Test { name, should_pass })
}

fn parse_coverage_fact(line: &str) -> Option<Fact> {
    // Pattern: coverage("name", percentage).
    let re = Regex::new(r#"^coverage\s*\(\s*"([^"]+)"\s*,\s*(\d+(?:\.\d+)?)\s*\)"#).ok()?;
    
    let cap = re.captures(line)?;
    let name = cap.get(1)?.as_str().to_string();
    let percentage = cap.get(2)?.as_str().parse::<f64>().ok()?;
    
    Some(Fact::Coverage { name, percentage })
}

fn parse_complexity_fact(line: &str) -> Option<Fact> {
    // Pattern: complexity("function", score).
    let re = Regex::new(r#"^complexity\s*\(\s*"([^"]+)"\s*,\s*(\d+)\s*\)"#).ok()?;
    
    let cap = re.captures(line)?;
    let function = cap.get(1)?.as_str().to_string();
    let score = cap.get(2)?.as_str().parse::<u32>().ok()?;
    
    Some(Fact::Complexity { function, score })
}

/// Collect all declarations into structured data
#[derive(Debug, Clone, Default)]
pub struct Declarations {
    pub tests: HashMap<String, TestDecl>,
    pub coverage: HashMap<String, f64>,
    pub complexity: HashMap<String, u32>,
}

#[derive(Debug, Clone)]
pub struct TestDecl {
    pub name: String,
    pub should_pass: bool,
    pub declared_coverage: Option<f64>,
    pub declared_complexity: Option<u32>,
}

impl Declarations {
    pub fn from_facts(facts: &[Fact]) -> Self {
        let mut decls = Self::default();
        
        for fact in facts {
            match fact {
                Fact::Test { name, should_pass } => {
                    decls.tests.insert(
                        name.clone(),
                        TestDecl {
                            name: name.clone(),
                            should_pass: *should_pass,
                            declared_coverage: None,
                            declared_complexity: None,
                        },
                    );
                }
                Fact::Coverage { name, percentage } => {
                    decls.coverage.insert(name.clone(), *percentage);
                    // Also update test decl if exists
                    if let Some(test) = decls.tests.get_mut(name) {
                        test.declared_coverage = Some(*percentage);
                    }
                }
                Fact::Complexity { function, score } => {
                    decls.complexity.insert(function.clone(), *score);
                    // Also update test decl if function name matches
                    if let Some(test) = decls.tests.get_mut(function) {
                        test.declared_complexity = Some(*score);
                    }
                }
            }
        }
        
        decls
    }
    
    pub fn validate_consistency(&self) -> Vec<String> {
        let mut errors = Vec::new();
        
        // Check all coverage declarations have matching tests
        for (test_name, cov) in &self.coverage {
            if !self.tests.contains_key(test_name) {
                errors.push(format!(
                    "Coverage declared for '{}' but no test('{}', _) declared",
                    cov, test_name
                ));
            }
        }
        
        // Check all complexity declarations have matching tests
        for (func, _score) in &self.complexity {
            if !self.tests.contains_key(func) {
                errors.push(format!(
                    "Complexity declared for '{}' but no corresponding test",
                    func
                ));
            }
        }
        
        errors
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_test_pass() {
        let input = r#"test("add_user", pass)."#;
        let fact = parse_test_fact(input).unwrap();
        
        match fact {
            Fact::Test { name, should_pass } => {
                assert_eq!(name, "add_user");
                assert!(should_pass);
            }
            _ => panic!("Expected Test fact"),
        }
    }
    
    #[test]
    fn test_parse_test_fail() {
        let input = r#"test("invalid_email", fail)."#;
        let fact = parse_test_fact(input).unwrap();
        
        match fact {
            Fact::Test { name, should_pass } => {
                assert_eq!(name, "invalid_email");
                assert!(!should_pass);
            }
            _ => panic!("Expected Test fact"),
        }
    }
    
    #[test]
    fn test_parse_coverage() {
        let input = r#"coverage("add_user", 95.5)."#;
        let fact = parse_coverage_fact(input).unwrap();
        
        match fact {
            Fact::Coverage { name, percentage } => {
                assert_eq!(name, "add_user");
                assert!((percentage - 95.5).abs() < 0.01);
            }
            _ => panic!("Expected Coverage fact"),
        }
    }
    
    #[test]
    fn test_parse_complexity() {
        let input = r#"complexity("process_data", 15)."#;
        let fact = parse_complexity_fact(input).unwrap();
        
        match fact {
            Fact::Complexity { function, score } => {
                assert_eq!(function, "process_data");
                assert_eq!(score, 15);
            }
            _ => panic!("Expected Complexity fact"),
        }
    }
    
    #[test]
    fn test_parse_datalog_block() {
        let input = r#"
test("my_func", pass).
coverage("my_func", 100.0).
complexity("my_func", 5).
"#;
        
        let parsed = parse_datalog(input).unwrap();
        assert_eq!(parsed.facts.len(), 3);
    }
}