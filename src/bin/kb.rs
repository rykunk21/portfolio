//! Knowledge Base CLI Tool
//! Run the solver and query the TDD knowledge base

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        print_usage();
        return;
    }
    
    match args[1].as_str() {
        "check" => check_tdd_health(),
        "next" => suggest_next_actions(),
        "violations" => list_violations(),
        "status" => show_issue_status(args.get(2)),
        "transitions" => show_ready_transitions(),
        "report" => generate_full_report(),
        _ => print_usage(),
    }
}

fn print_usage() {
    println!("Knowledge Base CLI - TDD Workflow Analysis");
    println!();
    println!("Usage: cargo run --bin kb <COMMAND> [ARGS]");
    println!();
    println!("Commands:");
    println!("  check           Check overall TDD health");
    println!("  next            Suggest what to work on next");
    println!("  violations      List all TDD violations");
    println!("  status <issue>  Show detailed status of an issue");
    println!("  transitions     Show issues ready for phase transition");
    println!("  report          Generate full workflow report");
}

fn check_tdd_health() {
    println!("Checking TDD workflow health...");
    // Implementation: Load from GitHub, build KB, get health report
    println!("✓ Not yet implemented - add GitHub integration");
}

fn suggest_next_actions() {
    println!("Next actions:");
    // Implementation: Query suggest_work_next, suggest_move_to_green, etc.
    println!("✓ Not yet implemented");
}

fn list_violations() {
    println!("TDD Violations:");
    // Implementation: Query tdd_violation_* relations
    println!("✓ Not yet implemented");
}

fn show_issue_status(issue_arg: Option<&String>) {
    match issue_arg {
        Some(issue) => {
            println!("Status for issue {}", issue);
            // Implementation: Query specific issue state
        }
        None => {
            println!("Error: Issue number required");
            println!("Usage: cargo run --bin kb status <issue_number>");
        }
    }
}

fn show_ready_transitions() {
    println!("Issues ready for phase transition:");
    println!("✓ Not yet implemented");
}

fn generate_full_report() {
    println!("===================");
    println!("TDD Workflow Report");
    println!("===================");
    println!();
    
    check_tdd_health();
    println!();
    
    println!("--- Blocked Issues ---");
    println!();
    
    println!("--- Ready Transitions ---");
    show_ready_transitions();
    println!();
    
    println!("--- Next Actions ---");
    suggest_next_actions();
}