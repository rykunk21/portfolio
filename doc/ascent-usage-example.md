# Ascent Knowledge Base - Usage Examples

## Basic Usage

### 1. Initialize the Knowledge Base

```rust
use portfolio::kb::KanbanKB;

let mut kb = KanbanKB::new();

// Add issues from GitHub
kb.add_issue(15, "TEST-002: Verify backlog to red".to_string());
kb.add_label(15, "red".to_string());

// Add PR information
kb.add_pr(16, "feature/TEST-002".to_string(), true);  // draft
kb.link_pr_to_issue(16, 15);

// Add test results
kb.add_test_result("test_auth_user".to_string(), false);
kb.add_test_result("test_auth_admin".to_string(), false);

// Run the solver
kb.solve();
```

### 2. Query Results

```rust
// Get violations
let violations = kb.get_violations();
for (issue, msg) in violations {
    println!("Issue #{}: {}", issue, msg);
}

// Get suggestions
let suggestions = kb.get_suggestions();
for (issue, suggestion) in suggestions {
    println!("Issue #{}: {}", issue, suggestion);
}

// Get ready transitions
let transitions = kb.get_ready_transitions();
for (phase, issues) in transitions {
    println!("Ready for {}: {:?}", phase, issues);
}

// Get full health report
let health = kb.get_health_report();
println!("Total issues: {}", health.total_issues);
println!("TDD violations: {}", health.violations.len());
```

### 3. Integrate with Backend API

```rust
// Rocket route
#[get("/api/insights")]
async fn get_insights(github: &State<GitHubClient>) -> Json<WorkflowHealth> {
    // Fetch from GitHub
    let issues = github.get_all_issues().await.unwrap();
    let prs = github.get_all_prs().await.unwrap();
    let test_results = github.get_latest_test_results().await.unwrap();
    
    // Build knowledge base
    let mut kb = KanbanKB::new();
    
    for issue in issues {
        kb.add_issue(issue.number, issue.title);
        for label in issue.labels {
            kb.add_label(issue.number, label.name);
        }
    }
    
    for pr in prs {
        kb.add_pr(pr.number, pr.branch, pr.is_draft);
        if let Some(issue) = pr.linked_issue {
            kb.link_pr_to_issue(pr.number, issue);
        }
    }
    
    for test in test_results {
        kb.add_test_result(test.name, test.passed);
    }
    
    // Solve and return
    kb.solve();
    Json(kb.get_health_report())
}
```

### 4. Integrate with Frontend

```rust
// Yew component
#[function_component(InsightsPanel)]
fn insights_panel() -> Html {
    let health = use_state(|| None::<WorkflowHealth>);
    
    use_effect_with((), move |_| {
        spawn_local(async move {
            let resp = Request::get("/api/insights").send().await.unwrap();
            let data: WorkflowHealth = resp.json().await.unwrap();
            health.set(Some(data));
        });
        || ()
    });
    
    html! {
        <div class="insights-panel">
            { if let Some(h) = health.as_ref() {
                html! {
                    <>
                        <h3>{ "TDD Health" }</h3>
                        <p>{ format!("Total: {}, Violations: {}", h.total_issues, h.violations.len()) }</p>
                        <ul>
                            { for h.violations.iter().map(|(i, v)| html! {
                                <li>{ format!("#{}: {}", i, v) }</li>
                            })}
                        </ul>
                        <h4>{ "Suggestions" }</h4>
                        <ul>
                            { for h.suggestions.iter().map(|(i, s)| html! {
                                <li>{ format!("#{}: {}", i, s) }</li>
                            })}
                        </ul>
                    </>
                }
            } else {
                html! { <p>{ "Loading insights..." }</p> }
            }}
        </div>
    }
}
```

## Workflow Automation

### Auto-promote Issues

```rust
// Check if any issues are ready for promotion
let transitions = kb.get_ready_transitions();

for ("red -> green", issues) in transitions {
    for issue in issues {
        // Use GitHub API to move label
        github.move_label(issue, "red", "green").await;
    }
}
```

### Blocker Detection

```rust
// Before starting work, check if blocked
let state = kb.get_issue_state(15);

if let Some(s) = state {
    if s.is_blocked {
        println!("Issue #15 is blocked! Work on dependencies first.");
    }
}
```

## CLI Examples

```bash
# Check overall TDD health
cargo run --bin kb check

# What's the next priority?
cargo run --bin kb next

# Show violations
cargo run --bin kb violations

# Status of specific issue
cargo run --bin kb status 15

# Full report
cargo run --bin kb report
```

## Advanced: Custom Rules

Add your own rules to `kb/rules/`:

```prolog
% custom.dl
% Flag PRs that have been in Red phase too long

rel red_phase_too_long(issue, days) =
    in_phase(issue, "red"),
    entered_phase(issue, "red", timestamp),
    days = (now() - timestamp) / 86400,
    days > 2.  % 2 days max in Red

rel suggest_pair_programming(issue) =
    red_phase_too_long(issue, _).
```

Then query it:

```rust
// After kb.solve()
for (issue,) in kb.program.suggest_pair_programming {
    println!("Issue #{}: Consider pair programming", issue);
}
```

## Performance Notes

- Ascent computations are fast (microseconds for hundreds of facts)
- Re-run on webhook events for real-time updates
- Cache results and invalidate on changes
- Use incremental solving for large knowledge bases
