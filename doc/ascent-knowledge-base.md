# Knowledge Base for Red-Green-Refactor with Ascent

## Overview

Ascent is a Rust logic programming library (Datalog-style) that enables declarative reasoning over relations. For TDD workflows, it can:

- **Track** the state of issues, PRs, tests, and code changes
- **Infer** blockers, dependencies, and next actions
- **Validate** that TDD principles are being followed
- **Suggest** what to work on next based on current state

---

## Core Concepts

### 1. Facts (Base Relations)

Raw data from GitHub, git, and test runners:

```rust
// Issues and their labels
fact issue(number: u32, title: String);
fact has_label(issue: u32, label: String);

// Pull requests and their state
fact pull_request(number: u32, branch: String, is_draft: bool);
fact references_issue(pr: u32, issue: u32);

// Test results from CI
fact test_result(test_name: String, passed: bool, commit: String);

// Code changes
fact file_changed(path: String, lines_added: u32, lines_removed: u32);
fact has_test_file(path: String);

// Dependencies between issues
fact blocks(blocker: u32, blocked: u32);
```

### 2. Rules (Derived Relations)

Logical inferences from the facts:

```rust
// TDD Phase inference
rel in_phase(issue: u32, phase: String) =
    issue(issue, _) &&
    has_label(issue, phase) &&
    (phase == "backlog" || phase == "red" || phase == "green" || phase == "refactor" || phase == "done");

// Issue has associated PR (linked)
rel has_linked_pr(issue: u32) =
    issue(issue, _) &&
    references_issue(_, issue);

// Issue is in Red phase with failing tests
rel red_with_failing_tests(issue: u32) =
    in_phase(issue, "red") &&
    has_linked_pr(issue) &&
    test_result(_, false, _);

// Issue is in Green phase (tests passing)
rel green_with_passing_tests(issue: u32) =
    in_phase(issue, "green") &&
    test_result(_, true, _);

// Issue is blocked
rel is_blocked(issue: u32) =
    issue(issue, _) &&
    blocks(blocker, issue) &&
    !in_phase(blocker, "done");

// TDD violation: Red phase but tests passing
rel tdd_violation_red_with_passing(issue: u32) =
    in_phase(issue, "red") &&
    has_linked_pr(issue) &&
    test_result(_, true, _) &&
    !has_label(issue, "done");

// Ready for Green phase (tests now passing)
rel ready_for_green(issue: u32) =
    in_phase(issue, "red") &&
    has_linked_pr(issue) &&
    test_result(_, true, _) &&
    !tdd_violation_red_with_passing(issue);

// Stale issue (no activity for 7 days)
rel stale_issue(issue: u32) =
    issue(issue, _) &&
    last_updated(issue, date) &&
    days_since(date) > 7;
```

---

## Knowledge Base File Structure

```
portfolio/
├── kb/                          # Knowledge base directory
│   ├── facts/
│   │   ├── issues.dl            # GitHub issues as facts
│   │   ├── prs.dl               # Pull request facts
│   │   ├── tests.dl             # Test result facts
│   │   └── git.dl               # Git commit/file facts
│   ├── rules/
│   │   ├── tdd.dl               # TDD workflow rules
│   │   ├── blockers.dl          # Dependency/blocking rules
│   │   ├── metrics.dl           # Velocity/complexity metrics
│   │   └── suggestions.dl       # Next-action suggestions
│   ├── queries/
│   │   ├── what_next.dl         # What should I work on?
│   │   ├── is_blocked.dl        # Is this issue blocked?
│   │   ├── tdd_health.dl        # Are we following TDD?
│   │   └── cycle_time.dl        # How long do phases take?
│   └── generated/
│       └── current_state.json   # Computed knowledge
└── src/
    └── kb.rs                    # Rust integration
```

---

## Static Analysis Use Cases

### 1. TDD Compliance Checking

```rust
// Check if workflow is following TDD properly
rel tdd_compliance_check() =
    // Every Red phase issue must have a failing test
    forall issue in_phase(issue, "red") =>
        exists test_name test_result(test_name, false, _);

    // Every Green phase issue must have passing tests
    forall issue in_phase(issue, "green") =>
        exists test_name test_result(test_name, true, _);

    // Refactor phase must have code changes without test changes
    forall issue in_phase(issue, "refactor") =>
        exists pr references_issue(pr, issue) &&
        !has_test_file_changed(pr);
```

### 2. Dependency Resolution

```rust
// What can I work on now? (not blocked, not done)
rel available_to_work() =
    issue(i, title) &&
    in_phase(i, phase) &&
    phase != "done" &&
    !is_blocked(i) &&
    !stale_issue(i);

// Critical path (blocking most other issues)
rel critical_path_issue(issue: u32) =
    issue(issue, _) &&
    count(blocked: blocks(issue, blocked)) > 3;
```

### 3. Cycle Time Analysis

```rust
// Track phase transitions
fact entered_phase(issue: u32, phase: String, timestamp: u64);

// Calculate time in current phase
rel time_in_phase(issue: u32, phase: String, hours: u64) =
    in_phase(issue, phase) &&
    entered_phase(issue, phase, start) &&
    hours = (now() - start) / 3600;

// Issues stuck in phase too long
rel stuck_issue(issue: u32, phase: String) =
    time_in_phase(issue, phase, hours) &&
    phase == "red" && hours > 24 ||  // Red should be quick
    phase == "green" && hours > 48 || // Green can take longer
    phase == "refactor" && hours > 12; // Refactor should be fast
```

---

## Integration with Rust

```rust
use ascent::{ascent, ascent_run};
use serde::{Deserialize, Serialize};

// Define the knowledge base
ascent! {
    // Facts
    relation issue(number: u32, title: String);
    relation has_label(issue: u32, label: String);
    relation pull_request(pr: u32, branch: String, is_draft: bool);
    relation references_pr(issue: u32, pr: u32);
    relation test_result(test: String, passed: bool);
    
    // Derived relations
    relation in_red_phase(issue: u32) = 
        issue(issue, _) && has_label(issue, "red");
    
    relation has_failing_tests(issue: u32) =
        references_pr(issue, pr) &&
        pull_request(pr, _, false) &&  // Not draft = has code
        test_result(_, false);
    
    relation ready_for_green(issue: u32) =
        in_red_phase(issue) &&
        !has_failing_tests(issue);
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KanbanKnowledgeBase {
    // Input data
    pub issues: Vec<(u32, String)>,
    pub labels: Vec<(u32, String)>,
    pub prs: Vec<(u32, String, bool)>,
    pub test_results: Vec<(String, bool)>,
}

impl KanbanKnowledgeBase {
    pub fn new() -> Self {
        Self {
            issues: vec![],
            labels: vec![],
            prs: vec![],
            test_results: vec![],
        }
    }
    
    pub fn from_github_data(repo: &str) -> anyhow::Result<Self> {
        // Fetch from GitHub API
        let issues = fetch_issues(repo)?;
        let prs = fetch_prs(repo)?;
        let test_results = fetch_test_results(repo)?;
        
        Ok(Self {
            issues: issues.iter().map(|i| (i.number, i.title.clone())).collect(),
            labels: issues.iter()
                .flat_map(|i| i.labels.iter().map(|l| (i.number, l.name.clone())))
                .collect(),
            prs: prs.iter().map(|p| (p.number, p.branch.clone(), p.is_draft)).collect(),
            test_results: test_results.iter().map(|t| (t.name.clone(), t.passed)).collect(),
        })
    }
    
    pub fn query_next_actions(&self) -> Vec<String> {
        let mut prog = AscentProgram::default();
        
        // Load facts
        for (num, title) in &self.issues {
            prog.issue.push((*num, title.clone()));
        }
        for (issue, label) in &self.labels {
            prog.has_label.push((*issue, label.clone()));
        }
        for (pr, branch, is_draft) in &self.prs {
            prog.pull_request.push((*pr, branch.clone(), *is_draft));
        }
        for (test, passed) in &self.test_results {
            prog.test_result.push((test.clone(), *passed));
        }
        
        // Run the solver
        prog.run();
        
        // Extract results
        prog.ready_for_green
            .iter()
            .map(|(issue,)| format!("Issue #{} ready to move to green phase", issue))
            .collect()
    }
    
    pub fn check_tdd_violations(&self) -> Vec<String> {
        // Query for TDD rule violations
        vec![]
    }
}
```

---

## Solver Queries

### Command Line Interface

```bash
# What's the next priority?
cargo run --bin kb -- query "next_action"

# Check for TDD violations
cargo run --bin kb -- query "tdd_violations"

# What's blocking issue #15?
cargo run --bin kb -- query "blockers 15"

# How long has issue #20 been in Red?
cargo run --bin kb -- query "cycle_time 20"

# Generate full report
cargo run --bin kb -- report
```

### Example Query Implementations

```rust
// kb/queries/next_action.rs
pub fn next_action(kb: &KanbanKnowledgeBase) -> Vec<Action> {
    ascent_run! {
        // Find issues that:
        // 1. Are in backlog and have no blockers
        // 2. Are in red with passing tests (ready for green)
        // 3. Are in green and need refactor
        
        relation priority_score(issue: u32, score: u32);
        
        // High priority: Ready for green
        priority_score(i, 100) := ready_for_green(i);
        
        // Medium priority: In backlog, not blocked
        priority_score(i, 50) := 
            in_phase(i, "backlog"), 
            !is_blocked(i);
        
        // Low priority: Stuck issues that need attention
        priority_score(i, 25) := 
            stuck_issue(i, _);
    }
}
```

---

## Automatic Fact Generation

### GitHub Webhook → Facts

```rust
// When PR is opened
fn on_pr_opened(pr: PullRequest) {
    let fact = format!(
        "pull_request({}, \"{}\", {}).",
        pr.number, pr.branch, pr.is_draft
    );
    append_fact("prs.dl", fact);
    
    // Re-run solver
    run_solver();
}

// When label changes
fn on_label_change(issue: u32, new_label: String) {
    // Retract old label fact
    retract_fact("issues.dl", issue);
    
    // Assert new label fact
    let fact = format!("has_label({}, \"{}\").", issue, new_label);
    append_fact("issues.dl", fact);
    
    // Re-run solver
    run_solver();
}

// When CI completes
fn on_ci_complete(commit: String, results: Vec<TestResult>) {
    for result in results {
        let fact = format!(
            "test_result(\"{}\", {}, \"{}\").",
            result.name, result.passed, commit
        );
        append_fact("tests.dl", fact);
    }
    
    run_solver();
}
```

---

## What Can Be Done

### 1. **Smart Issue Assignment**
```rust
// Who should work on what?
rel assigned_dev(issue: u32, dev: String) =
    issue_skills_required(issue, skill) &&
    dev_has_skill(dev, skill) &&
    dev_workload(dev) < 3;  // Not overloaded
```

### 2. **Predictive Blockers**
```rust
// This issue will likely be blocked by...
rel predicted_blocker(issue: u32, blocker: u32) =
    shares_component(issue, blocker) &&
    in_phase(blocker, "red") &&  // Blocker is still being developed
    !in_phase(issue, "done");
```

### 3. **Code Quality Gates**
```rust
// Can't move to green if...
rel green_blocked(issue: u32, reason: String) =
    in_phase(issue, "red") &&
    test_coverage(issue, coverage) &&
    coverage < 80 &&
    reason = "Insufficient test coverage";
```

### 4. **Complexity Estimation**
```rust
rel estimated_effort(issue: u32, hours: u32) =
    files_touched(issue, count) &&
    count < 5 && hours = 4;
    
estimated_effort(issue, hours) =
    files_touched(issue, count) &&
    count >= 5 && count < 20 && hours = 16;
```

---

## Implementation Steps

1. **Add Ascent to Cargo.toml**
   ```toml
   [dependencies]
   ascent = "0.8"
   ```

2. **Create fact generation from GitHub API**

3. **Define TDD workflow rules in Datalog**

4. **Expose solver as CLI and library**

5. **Integrate with GitHub webhooks for real-time updates**

6. **Add to dashboard as "Insights" panel**

This creates a **living knowledge base** that reasons about your TDD workflow, enabling intelligent automation beyond simple label changes.