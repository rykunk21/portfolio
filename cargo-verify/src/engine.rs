//! Ascent-based verification engine

use ascent::ascent;
use crate::parse::TestDecl;
use crate::collect::ActualTest;
use std::collections::HashMap;

// Minimal ascent program to start
ascent! {
    relation declared_test(String, bool);
    relation actual_test(String, bool);
    relation violation(String, String);
    
    violation(x, y) <-- 
        declared_test(x, _),
        !actual_test(x, _),
        let y = "Test declared but not found".to_string();
}

// AscentProgram is now available in this module

/// Config and Result types...
#[derive(Debug, Clone)]
pub struct VerifyConfig {
    pub check_coverage: bool,
    pub check_complexity: bool,
    pub use_cache: bool,
}

impl Default for VerifyConfig {
    fn default() -> Self {
        Self {
            check_coverage: true,
            check_complexity: true,
            use_cache: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct VerificationResult {
    pub violations: Vec<(String, String)>,
    pub warnings: Vec<(String, String)>,
    pub successes: Vec<(String, String)>,
}

impl VerificationResult {
    pub fn exit_code(&self) -> i32 {
        if self.violations.is_empty() { 0 } else { 1 }
    }
    pub fn is_clean(&self) -> bool { self.violations.is_empty() }
}

pub fn verify(
    declared: &HashMap<String, TestDecl>,
    actual: &[ActualTest],
    _coverage: &HashMap<String, f64>,
    _complexity: &HashMap<String, u32>,
    _config: &VerifyConfig,
) -> VerificationResult {
    let mut prog: AscentProgram = Default::default();
    
    for (name, test) in declared {
        prog.declared_test.push((name.clone(), test.should_pass));
    }
    
    for test in actual {
        prog.actual_test.push((test.name.clone(), test.passed));
    }
    
    prog.run();
    
    VerificationResult {
        violations: prog.violation,
        warnings: Vec::new(),
        successes: Vec::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_basic_violation() {
        let mut declared = HashMap::new();
        declared.insert(
            "test_a".to_string(),
            TestDecl { name: "test_a".to_string(), should_pass: true, declared_coverage: None, declared_complexity: None }
        );
        
        let actual: Vec<ActualTest> = vec![];
        let result = verify(&declared, &actual, &HashMap::new(), &HashMap::new(), &VerifyConfig::default()
        );
        
        assert!(!result.is_clean());
    }
}