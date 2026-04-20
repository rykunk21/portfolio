//! Collect actual test and coverage data from cargo

use anyhow::{Context, Result};
use serde::Deserialize;
use std::collections::HashMap;
use std::path::Path;
use tokio::process::Command as TokioCommand;

/// Actual test results from cargo test
#[derive(Debug, Clone, Default)]
pub struct ActualResults {
    pub tests: Vec<ActualTest>,
    pub coverage: HashMap<String, f64>,
}

#[derive(Debug, Clone)]
pub struct ActualTest {
    pub name: String,
    pub passed: bool,
    pub stdout: Option<String>,
}

/// Run cargo test and collect results
pub async fn collect_tests(
    manifest_path: Option<&Path>,
) -> Result<ActualResults> {
    let mut cmd = TokioCommand::new("cargo");
    cmd.arg("test")
        .arg("--no-run")  // Just compile first
        .arg("--message-format=json");
    
    if let Some(path) = manifest_path {
        cmd.arg("--manifest-path").arg(path);
    }
    
    // First, get test binaries
    let output = cmd
        .output()
        .await
        .context("Failed to run cargo test --no-run")?;
    
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow::anyhow!(
            "cargo test failed: {}",
            stderr
        ));
    }
    
    // Parse test binaries from JSON output
    let stdout = String::from_utf8_lossy(&output.stdout);
    let test_bins = parse_test_binaries(&stdout)?;
    
    // Now run the actual tests
    let mut results = ActualResults::default();
    
    for bin in test_bins {
        let bin_results = run_test_binary(&bin).await?;
        results.tests.extend(bin_results);
    }
    
    Ok(results)
}

/// Collect coverage using cargo-llvm-cov
pub async fn collect_coverage(
    _manifest_path: Option<&Path>,
) -> Result<HashMap<String, f64>> {
    // Check if cargo-llvm-cov is installed
    let check = TokioCommand::new("cargo")
        .args(&["llvm-cov", "--version"])
        .output()
        .await?;
    
    if !check.status.success() {
        return Err(anyhow::anyhow!(
            "cargo-llvm-cov not installed. Install with: cargo install cargo-llvm-cov"
        ));
    }
    
    let mut cmd = TokioCommand::new("cargo");
    cmd.args(&[
        "llvm-cov",
        "--json",
        "--summary-only",
    ]);
    
    let output = cmd
        .output()
        .await
        .context("Failed to run cargo llvm-cov")?;
    
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow::anyhow!(
            "cargo llvm-cov failed: {}",
            stderr
        ));
    }
    
    let json_str = String::from_utf8_lossy(&output.stdout);
    parse_coverage_json(&json_str)
}

#[derive(Deserialize)]
struct CargoArtifact {
    #[serde(rename = "reason")]
    _reason: String,
    executable: Option<String>,
    target: CargoTarget,
}

#[derive(Deserialize)]
struct CargoTarget {
    kind: Vec<String>,
    name: String,
}

fn parse_test_binaries(output: &str) -> Result<Vec<String>> {
    let mut binaries = Vec::new();
    
    for line in output.lines() {
        // Parse JSON messages
        if let Ok(artifact) = serde_json::from_str::<CargoArtifact>(line) {
            if artifact.target.kind.contains(&"test".to_string()) {
                if let Some(exec) = artifact.executable {
                    binaries.push(exec);
                }
            }
        }
    }
    
    Ok(binaries)
}

async fn run_test_binary(
    binary: &str,
) -> Result<Vec<ActualTest>> {
    let output = TokioCommand::new(binary)
        .arg("--nocapture")
        .arg("--test-threads=1")
        .output()
        .await
        .context(format!("Failed to run test binary: {}", binary))?;
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    
    // Parse test output
    // Format: test test_name ... ok|FAILED
    let mut tests = Vec::new();
    let combined = format!("{}\n{}", stdout, stderr);
    
    for line in combined.lines() {
        if let Some(test_info) = parse_test_line(line) {
            tests.push(test_info);
        }
    }
    
    Ok(tests)
}

fn parse_test_line(line: &str) -> Option<ActualTest> {
    // Look for: "test test_name ... ok" or "test test_name ... FAILED"
    let re = regex::Regex::new(r"test\s+(\S+)\s+\.\.\.\s+(ok|FAILED)").ok()?;
    
    let cap = re.captures(line)?;
    let name = cap.get(1)?.as_str().to_string();
    let result = cap.get(2)?.as_str();
    let passed = result == "ok";
    
    Some(ActualTest {
        name,
        passed,
        stdout: None,
    })
}

fn parse_coverage_json(
    json: &str
) -> Result<HashMap<String, f64>> {
    // cargo-llvm-cov JSON format:
    // [{"file": "src/lib.rs", "functions": [...], "percent": 95.5}, ...]
    
    let mut coverage = HashMap::new();
    
    // For now, just parse as generic JSON
    // Full implementation would map specific functions to coverage
    
    // If it's a simple percentage
    if let Ok(percent) = json.trim().parse::<f64>() {
        coverage.insert("_total".to_string(), percent);
    }
    
    Ok(coverage)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_test_line_ok() {
        let line = "test my_test ... ok";
        let result = parse_test_line(line).unwrap();
        assert_eq!(result.name, "my_test");
        assert!(result.passed);
    }
    
    #[test]
    fn test_parse_test_line_failed() {
        let line = "test my_test ... FAILED";
        let result = parse_test_line(line).unwrap();
        assert_eq!(result.name, "my_test");
        assert!(!result.passed);
    }
}