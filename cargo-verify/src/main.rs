//! cargo-verify: Datalog-Gated Promotion System
//! 
//! Extracts Datalog facts from Rust docstrings and gates CI promotion
//! on declared-vs-actual compliance.

use anyhow::{Context, Result};
use clap::Parser;
use std::path::PathBuf;

mod collect;
mod engine;
mod extract;
mod parse;
mod report;

use engine::{VerifyConfig, verify};
use extract::extract_declarations;
use parse::{parse_datalog, Declarations};
use report::{generate_report, OutputFormat, ReportConfig};

#[derive(Parser, Debug)]
#[command(name = "cargo-verify")]
#[command(about = "Verify Rust code against Datalog declarations in docstrings")]
#[command(version)]
struct Args {
    /// Verify target (defaults to all in workspace)
    #[arg(value_name = "TARGET")]
    target: Option<String>,
    
    /// Path to Cargo.toml
    #[arg(long, value_name = "PATH")]
    manifest_path: Option<PathBuf>,
    
    /// Skip tests, use cached results
    #[arg(long)]
    check: bool,
    
    /// Output format
    #[arg(short, long, value_enum, default_value = "human")]
    format: FormatArg,
    
    /// Skip coverage verification
    #[arg(long)]
    no_coverage: bool,
    
    /// Skip complexity verification
    #[arg(long)]
    no_complexity: bool,
    
    /// Quiet output (errors only)
    #[arg(short)]
    quiet: bool,
}

#[derive(Debug, Clone, clap::ValueEnum)]
enum FormatArg {
    Human,
    Json,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    
    // Determine manifest path
    let manifest_path = if let Some(path) = &args.manifest_path {
        path.clone()
    } else {
        std::env::current_dir()?.join("Cargo.toml")
    };
    
    // Step 1: Extract declarations from docstrings
    if !args.quiet {
        eprintln!("Collecting declarations from docstrings...");
    }
    
    let declarations = extract_declarations(&manifest_path
    ).context("Failed to extract declarations")?;
    
    if declarations.is_empty() {
        eprintln!("Warning: No Datalog declarations found in docstrings");
        return Ok(());
    }
    
    if !args.quiet {
        eprintln!("Found {} declaration blocks", declarations.len());
    }
    
    // Step 2: Parse the datalog facts
    let mut all_facts = Vec::new();
    for decl in &declarations {
        match parse_datalog(&decl.datalog) {
            Ok(parsed) => {
                if !parsed.errors.is_empty() {
                    for err in &parsed.errors {
                        eprintln!(
                            "Parse error in {} at line {}: {}",
                            decl.file, decl.line, err
                        );
                    }
                }
                all_facts.extend(parsed.facts);
            }
            Err(e) => {
                eprintln!(
                    "Failed to parse datalog from {} at line {}: {}",
                    decl.file, decl.line, e
                );
            }
        }
    }
    
    if all_facts.is_empty() {
        eprintln!("No valid Datalog facts found");
        return Ok(());
    }
    
    // Build declaration structure
    let declarations_map = Declarations::from_facts(&all_facts);
    
    // Validate consistency
    let consistency_errors = declarations_map.validate_consistency();
    for err in &consistency_errors {
        eprintln!("Declaration error: {}", err);
    }
    
    // Step 3: Collect actual results (unless --check)
    let (actual_tests, actual_coverage) = if args.check {
        // TODO: Load from cache
        (Vec::new(), std::collections::HashMap::new())
    } else {
        if !args.quiet {
            eprintln!("Running cargo tests...");
        }
        
        let tests = collect::collect_tests(Some(&manifest_path))
            .await
            .context("Failed to collect test results")?;
        
        let coverage = if args.no_coverage {
            std::collections::HashMap::new()
        } else {
            if !args.quiet {
                eprintln!("Collecting coverage data...");
            }
            
            match collect::collect_coverage(Some(&manifest_path)).await {
                Ok(cov) => cov,
                Err(e) => {
                    eprintln!("Coverage collection failed: {}", e);
                    eprintln!("Run with --no-coverage to skip coverage verification");
                    std::collections::HashMap::new()
                }
            }
        };
        
        (tests.tests, coverage)
    };
    
    // Step 4: Run verification engine
    let verify_config = VerifyConfig {
        check_coverage: !args.no_coverage,
        check_complexity: !args.no_complexity,
        use_cache: args.check,
    };
    
    // Get complexity from parsed facts
    let complexity: std::collections::HashMap<String, u32> = declarations_map
        .complexity
        .clone();
    
    let result = verify(
        &declarations_map.tests,
        &actual_tests,
        &actual_coverage,
        &complexity,
        &verify_config,
    );
    
    // Step 5: Generate report
    let report_config = ReportConfig {
        format: match args.format {
            FormatArg::Human => OutputFormat::Human,
            FormatArg::Json => OutputFormat::Json,
        },
        quiet: args.quiet,
        show_warnings: !args.quiet,
    };
    
    let report = generate_report(&result, &declarations, &report_config);
    
    // Output report
    print!("{}", report);
    
    // Exit with appropriate code
    std::process::exit(result.exit_code());
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_e2e_simple() {
        // This would test the full pipeline with a temp project
        // Implementation omitted for brevity
    }
}