//! Report formatting - human-readable and JSON output

use crate::engine::VerificationResult;
use crate::extract::Declaration;
use colored::Colorize;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OutputFormat {
    Human,
    Json,
}

#[derive(Debug, Clone)]
pub struct ReportConfig {
    pub format: OutputFormat,
    pub quiet: bool,
    pub show_warnings: bool,
}

impl Default for ReportConfig {
    fn default() -> Self {
        Self {
            format: OutputFormat::Human,
            quiet: false,
            show_warnings: true,
        }
    }
}

/// Full report data structure
#[derive(Debug, Serialize)]
pub struct FullReport {
    pub summary: Summary,
    pub violations: Vec<Violation>,
    pub warnings: Vec<Warning>,
    pub successes: Vec<Success>,
}

#[derive(Debug, Serialize)]
pub struct Summary {
    pub total_tests: usize,
    pub violations_count: usize,
    pub warnings_count: usize,
    pub success_count: usize,
    pub passed: bool,
}

#[derive(Debug, Serialize)]
pub struct Violation {
    pub test_name: String,
    pub message: String,
    pub file: Option<String>,
    pub line: Option<usize>,
}

#[derive(Debug, Serialize)]
pub struct Warning {
    pub test_name: String,
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct Success {
    pub test_name: String,
    pub message: String,
}

/// Generate report from verification result
pub fn generate_report(
    result: &VerificationResult,
    declarations: &[Declaration],
    config: &ReportConfig,
) -> String {
    let report = build_full_report(result, declarations);
    
    match config.format {
        OutputFormat::Json => generate_json(report),
        OutputFormat::Human => generate_human(report, config),
    }
}

fn build_full_report(
    result: &VerificationResult,
    declarations: &[Declaration],
) -> FullReport {
    // Build lookup for file/line info
    let mut loc_map: HashMap<String, (String, usize)> = HashMap::new();
    for decl in declarations {
        // Parse the datalog to find test names
        // This is simplified - real implementation would parse facts
        for line in decl.datalog.lines() {
            if line.contains("test(") {
                if let Some(test_name) = extract_quoted_string(line) {
                    loc_map.insert(
                        test_name,
                        (decl.file.clone(), decl.line)
                    );
                }
            }
        }
    }
    
    let violations: Vec<Violation> = result
        .violations
        .iter()
        .map(|(name, msg)| {
            let (file, line) = loc_map.get(name)
                .map(|(f, l)| (Some(f.clone()), Some(*l)))
                .unwrap_or((None, None));
            
            Violation {
                test_name: name.clone(),
                message: msg.clone(),
                file,
                line,
            }
        })
        .collect();
    
    let warnings: Vec<Warning> = result
        .warnings
        .iter()
        .map(|(name, msg)| Warning {
            test_name: name.clone(),
            message: msg.clone(),
        })
        .collect();
    
    let successes: Vec<Success> = result
        .successes
        .iter()
        .map(|(name, msg)| Success {
            test_name: name.clone(),
            message: msg.clone(),
        })
        .collect();
    
    FullReport {
        summary: Summary {
            total_tests: violations.len() + warnings.len() + successes.len(),
            violations_count: violations.len(),
            warnings_count: warnings.len(),
            success_count: successes.len(),
            passed: violations.is_empty(),
        },
        violations,
        warnings,
        successes,
    }
}

fn generate_human(report: FullReport, config: &ReportConfig) -> String {
    let mut output = String::new();
    
    if config.quiet {
        // In quiet mode, only output if there are violations
        if !report.violations.is_empty() {
            for v in &report.violations {
                if let (Some(file), Some(line)) = (&v.file, v.line) {
                    output.push_str(&format!(
                        "{}:{}: {}: {}\n",
                        file, line,
                        "error".red().bold(),
                        v.message
                    ));
                } else {
                    output.push_str(&format!(
                        "{}: {}\n",
                        "error".red().bold(),
                        v.message
                    ));
                }
            }
        }
        return output;
    }
    
    // Header
    output.push_str(&sep_line('='));
    output.push_str("cargo-verify\n");
    output.push_str(&sep_line('='));
    output.push('\n');
    
    // Summary
    if report.summary.passed {
        output.push_str(&format!(
            "{} {}/{} tests verified, {} {}\n\n",
            "✓".green(),
            report.summary.success_count,
            report.summary.total_tests,
            "passed".green().bold(),
            "verification complete".green()
        ));
    } else {
        output.push_str(&format!(
            "{} {}/{} tests verified, {} {}\n\n",
            "✗".red(),
            report.summary.success_count,
            report.summary.total_tests,
            "failed".red().bold(),
            "verification failed".red()
        ));
    }
    
    // Violations
    if !report.violations.is_empty() {
        output.push_str(&format!("{} {} {}\n", &sep_line('-'), "VIOLATIONS".red().bold(), &sep_line('-')));
        for v in &report.violations {
            if let (Some(file), Some(line)) = (&v.file, v.line) {
                output.push_str(&format!(
                    "  {} {}\n     {} {}\n",
                    "→".red(),
                    v.test_name.bold(),
                    file,
                    format!("line {}", line).dimmed()
                ));
            }
            output.push_str(&format!(
                "     {} {}\n\n",
                "ERROR:".red().bold(),
                v.message
            ));
        }
    }
    
    // Warnings
    if config.show_warnings && !report.warnings.is_empty() {
        output.push_str(&format!("{} {} {}\n", &sep_line('-'), "WARNINGS".yellow().bold(), &sep_line('-')));
        for w in &report.warnings {
            output.push_str(&format!(
                "  {} {}: {}\n",
                "⚠".yellow(),
                w.test_name.bold(),
                w.message.dimmed()
            ));
        }
        output.push('\n');
    }
    
    // Successes (only if verbose)
    if !report.successes.is_empty() && !config.quiet {
        output.push_str(&format!("{} {} {}\n", &sep_line('-'), "PASSED".green().bold(), &sep_line('-')));
        for s in &report.successes {
            output.push_str(&format!(
                "  {} {}\n     {}\n",
                "✓".green(),
                s.test_name.bold(),
                s.message.dimmed()
            ));
        }
        output.push('\n');
    }
    
    output
}

fn generate_json(report: FullReport) -> String {
    serde_json::to_string_pretty(&report).unwrap_or_else(|_| {
        r#"{"error": "JSON serialization failed"}"#.to_string()
    })
}

fn sep_line(ch: char) -> String {
    ch.to_string().repeat(60) + "\n"
}

fn extract_quoted_string(line: &str) -> Option<String> {
    let re = regex::Regex::new(r#""([^"]+)""#).ok()?;
    re.captures(line)?.get(1).map(|m| m.as_str().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_json_output() {
        let result = VerificationResult {
            violations: vec![(
                "test_a".to_string(),
                "Failed".to_string(),
            )],
            warnings: vec![],
            successes: vec![],
        };
        
        let decls = vec![];
        let config = ReportConfig {
            format: OutputFormat::Json,
            ..Default::default()
        };
        
        let output = generate_report(&result, &decls, &config);
        
        assert!(output.contains("violations"));
        assert!(output.contains("test_a"));
        
        // Valid JSON?
        let _: serde_json::Value = serde_json::from_str(&output).unwrap();
    }
}