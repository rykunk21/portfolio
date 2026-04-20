//! Knowledge Base Module
//! Uses Ascent for Datalog-style reasoning over TDD workflow

use ascent::{ascent, ascent_run, Ascent, Lattice};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Core knowledge base relations
ascent! {
    // === BASE FACTS (from GitHub/Webhooks) ===
    
    /// Issue exists: (number, title)
    relation issue(number: u32, title: String);
    
    /// Issue has label: (issue_number, label_name)
    relation has_label(issue: u32, label: String);
    
    /// Pull request exists: (pr_number, branch_name, is_draft)
    relation pull_request(pr: u32, branch: String, is_draft: bool);
    
    /// PR references issue: (pr_number, issue_number)
    relation references_issue(pr: u32, issue: u32);
    
    /// Test result: (test_name, passed)
    relation test_result(test: String, passed: bool);
    
    /// PR has test file changed: (pr_number, file_path)
    relation test_file_changed(pr: u32, path: String);
    
    /// PR has source file changed: (pr_number, file_path)
    relation source_file_changed(pr: u32, path: String);
    
    /// Issue blocks another: (blocker_issue, blocked_issue)
    relation blocks(blocker: u32, blocked: u32);
    
    /// Issue was last updated: (issue_number, unix_timestamp)
    relation last_updated(issue: u32, timestamp: u64);
    
    // === DERIVED RELATIONS (Rules) ===
    
    /// Issue is in specific phase
    relation in_phase(issue: u32, phase: String) =
        issue(issue, _), 
        has_label(issue, phase),
        (phase == "backlog" || phase == "red" || phase == "green" || phase == "refactor" || phase == "done");
    
    /// Issue has an active PR
    relation has_linked_pr(issue: u32) =
        issue(issue, _),
        references_issue(_, issue);
    
    /// Failing tests exist for issue
    relation has_failing_tests(issue: u32) =
        has_linked_pr(issue),
        references_issue(pr, issue),
        test_result(_, false);  // At least one test failed
    
    /// All tests passing for issue
    relation has_passing_tests(issue: u32) =
        has_linked_pr(issue),
        references_issue(pr, issue),
        test_result(_, true),
        !has_failing_tests(issue);
    
    /// Code changed for issue (not just tests)
    relation code_changed(issue: u32) =
        has_linked_pr(issue),
        references_issue(pr, issue),
        source_file_changed(pr, _);
    
    /// Tests changed for issue
    relation tests_changed(issue: u32) =
        has_linked_pr(issue),
        references_issue(pr, issue),
        test_file_changed(pr, _);
    
    /// Issue is blocked by another
    relation is_blocked(issue: u32) =
        issue(issue, _),
        blocks(blocker, issue),
        !in_phase(blocker, "done".to_string());
    
    // === TDD VALIDATION RELATIONS ===
    
    /// Valid Red phase: draft PR + failing tests
    relation valid_red_phase(issue: u32) =
        in_phase(issue, "red"),
        references_issue(pr, issue),
        pull_request(pr, _, true),  // is_draft = true
        has_failing_tests(issue);
    
    /// Ready to move Red -> Green
    relation ready_for_green(issue: u32) =
        in_phase(issue, "red"),
        has_linked_pr(issue),
        has_passing_tests(issue),
        references_issue(pr, issue),
        !pull_request(pr, _, true);  // PR is ready (not draft)
    
    /// Ready to move Green -> Refactor
    relation ready_for_refactor(issue: u32) =
        in_phase(issue, "green"),
        has_linked_pr(issue),
        has_passing_tests(issue),
        code_changed(issue);
    
    /// Ready to move Refactor -> Done (no test changes in refactor)
    relation ready_for_done(issue: u32) =
        in_phase(issue, "refactor"),
        has_linked_pr(issue),
        has_passing_tests(issue),
        code_changed(issue),
        !tests_changed(issue);
    
    /// TDD Violation: Red phase but tests passing (should be failing!)
    relation tdd_violation_red_passing(issue: u32, msg: String) =
        in_phase(issue, "red"),
        has_linked_pr(issue),
        has_passing_tests(issue),
        msg = "Red phase issue should have failing tests".to_string();
    
    /// TDD Violation: Red phase but PR is ready (not draft)
    relation tdd_violation_red_not_draft(issue: u32, msg: String) =
        in_phase(issue, "red"),
        references_issue(pr, issue),
        pull_request(pr, _, false),  // not draft
        msg = "Red phase PR should be a draft".to_string();
    
    /// TDD Violation: Green phase but tests failing
    relation tdd_violation_green_failing(issue: u32, msg: String) =
        in_phase(issue, "green"),
        has_linked_pr(issue),
        has_failing_tests(issue),
        msg = "Green phase issue should have all passing tests".to_string();
    
    // === WORKFLOW SUGGESTIONS ===
    
    /// Suggest: Move to Green phase
    relation suggest_move_to_green(issue: u32, reason: String) =
        ready_for_green(issue),
        reason = "Tests are passing and PR is ready for review".to_string();
    
    /// Suggest: Mark PR as draft
    relation suggest_mark_draft(issue: u32, reason: String) =
        in_phase(issue, "red"),
        references_issue(pr, issue),
        pull_request(pr, _, false),
        reason = "Red phase should start with a draft PR".to_string();
    
    /// Suggest: Work on this next (not blocked, in backlog)
    relation suggest_work_next(issue: u32, reason: String) =
        in_phase(issue, "backlog"),
        !is_blocked(issue),
        reason = "Available to work on - not blocked".to_string();
}

/// Query interface for the knowledge base
pub struct KanbanKB {
    program: AscentProgram,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IssueState {
    pub number: u32,
    pub title: String,
    pub phase: String,
    pub is_blocked: bool,
    pub suggestions: Vec<String>,
    pub violations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowHealth {
    pub total_issues: usize,
    pub valid_tdd_count: usize,
    pub violations: Vec<(u32, String)>,
    pub suggestions: Vec<(u32, String)>,
}

impl KanbanKB {
    pub fn new() -> Self {
        Self {
            program: AscentProgram::default(),
        }
    }
    
    /// Add an issue fact
    pub fn add_issue(&mut self, number: u32, title: String) {
        self.program.issue.push((number, title));
    }
    
    /// Add a label fact
    pub fn add_label(&mut self, issue: u32, label: String) {
        self.program.has_label.push((issue, label));
    }
    
    /// Add a PR fact
    pub fn add_pr(&mut self, pr: u32, branch: String, is_draft: bool) {
        self.program.pull_request.push((pr, branch, is_draft));
    }
    
    /// Link PR to issue
    pub fn link_pr_to_issue(&mut self, pr: u32, issue: u32) {
        self.program.references_issue.push((pr, issue));
    }
    
    /// Add test result
    pub fn add_test_result(&mut self, test_name: String, passed: bool) {
        self.program.test_result.push((test_name, passed));
    }
    
    /// Add file change
    pub fn add_source_change(&mut self, pr: u32, path: String) {
        self.program.source_file_changed.push((pr, path));
    }
    
    pub fn add_test_change(&mut self, pr: u32, path: String) {
        self.program.test_file_changed.push((pr, path));
    }
    
    /// Add dependency
    pub fn add_blocks(&mut self, blocker: u32, blocked: u32) {
        self.program.blocks.push((blocker, blocked));
    }
    
    /// Run the solver
    pub fn solve(&mut self) {
        self.program.run();
    }
    
    /// Get all TDD violations
    pub fn get_violations(&self) -> Vec<(u32, String)> {
        let mut violations = Vec::new();
        
        for (issue, msg) in &self.program.tdd_violation_red_passing {
            violations.push((*issue, msg.clone()));
        }
        
        for (issue, msg) in &self.program.tdd_violation_red_not_draft {
            violations.push((*issue, msg.clone()));
        }
        
        for (issue, msg) in &self.program.tdd_violation_green_failing {
            violations.push((*issue, msg.clone()));
        }
        
        violations
    }
    
    /// Get workflow suggestions
    pub fn get_suggestions(&self) -> Vec<(u32, String)> {
        let mut suggestions = Vec::new();
        
        for (issue, reason) in &self.program.suggest_move_to_green {
            suggestions.push((*issue, format!("Move to Green phase: {}", reason)));
        }
        
        for (issue, reason) in &self.program.suggest_mark_draft {
            suggestions.push((*issue, format!("Mark as draft: {}", reason)));
        }
        
        for (issue, reason) in &self.program.suggest_work_next {
            suggestions.push((*issue, format!("Work on next: {}", reason)));
        }
        
        suggestions
    }
    
    /// Get issues ready for phase transition
    pub fn get_ready_transitions(&self) -> HashMap<String, Vec<u32>> {
        let mut transitions = HashMap::new();
        
        let green_ready: Vec<u32> = self.program.ready_for_green
            .iter()
            .map(|(issue,)| *issue)
            .collect();
        
        let refactor_ready: Vec<u32> = self.program.ready_for_refactor
            .iter()
            .map(|(issue,)| *issue)
            .collect();
        
        let done_ready: Vec<u32> = self.program.ready_for_done
            .iter()
            .map(|(issue,)| *issue)
            .collect();
        
        if !green_ready.is_empty() {
            transitions.insert("red -> green".to_string(), green_ready);
        }
        
        if !refactor_ready.is_empty() {
            transitions.insert("green -> refactor".to_string(), refactor_ready);
        }
        
        if !done_ready.is_empty() {
            transitions.insert("refactor -> done".to_string(), done_ready);
        }
        
        transitions
    }
    
    /// Get full workflow health report
    pub fn get_health_report(&self) -> WorkflowHealth {
        let total = self.program.issue.len();
        let valid_tdd = self.program.valid_red_phase.len() 
            + self.program.ready_for_green.len()
            + self.program.ready_for_refactor.len()
            + self.program.ready_for_done.len();
        
        WorkflowHealth {
            total_issues: total,
            valid_tdd_count: valid_tdd,
            violations: self.get_violations(),
            suggestions: self.get_suggestions(),
        }
    }
    
    /// Get state of specific issue
    pub fn get_issue_state(&self, number: u32) -> Option<IssueState> {
        // Find issue title
        let title = self.program.issue
            .iter()
            .find(|(n, _)| *n == number)
            .map(|(_, t)| t.clone())?;
        
        // Find phase
        let phase = self.program.in_phase
            .iter()
            .find(|(n, _)| *n == number)
            .map(|(_, p)| p.clone())
            .unwrap_or_else(|| "unknown".to_string());
        
        // Check if blocked
        let is_blocked = self.program.is_blocked
            .iter()
            .any(|(n,)| *n == number);
        
        // Get suggestions for this issue
        let suggestions: Vec<String> = self.get_suggestions()
            .into_iter()
            .filter(|(n, _)| *n == number)
            .map(|(_, s)| s)
            .collect();
        
        // Get violations for this issue
        let violations: Vec<String> = self.get_violations()
            .into_iter()
            .filter(|(n, _)| *n == number)
            .map(|(_, v)| v)
            .collect();
        
        Some(IssueState {
            number,
            title,
            phase,
            is_blocked,
            suggestions,
            violations,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_tdd_red_phase() {
        let mut kb = KanbanKB::new();
        
        // Setup: Issue 15 in Red, with draft PR, failing tests
        kb.add_issue(15, "TEST-002".to_string());
        kb.add_label(15, "red".to_string());
        kb.add_pr(16, "feature/test".to_string(), true);  // draft
        kb.link_pr_to_issue(16, 15);
        kb.add_test_result("test_auth".to_string(), false);  // failing
        
        kb.solve();
        
        // Should be valid Red phase
        assert!(kb.program.valid_red_phase.iter().any(|(n,)| *n == 15));
        
        // Should NOT be ready for green (tests failing)
        assert!(!kb.program.ready_for_green.iter().any(|(n,)| *n == 15));
    }
    
    #[test]
    fn test_ready_for_green() {
        let mut kb = KanbanKB::new();
        
        // Setup: Issue 15 in Red, PR ready, tests passing
        kb.add_issue(15, "TEST-002".to_string());
        kb.add_label(15, "red".to_string());
        kb.add_pr(16, "feature/test".to_string(), false);  // ready
        kb.link_pr_to_issue(16, 15);
        kb.add_test_result("test_auth".to_string(), true);  // passing
        
        kb.solve();
        
        // Should be ready for green
        assert!(kb.program.ready_for_green.iter().any(|(n,)| *n == 15));
        
        // Should be a violation (Red with passing tests)
        assert!(kb.get_violations().iter().any(|(n, _)| *n == 15));
    }
    
    #[test]
    fn test_blocked_issue() {
        let mut kb = KanbanKB::new();
        
        // Issue 12 blocks Issue 15
        kb.add_issue(12, "Auth API".to_string());
        kb.add_issue(15, "Contact Form".to_string());
        kb.add_label(12, "red".to_string());  // Not done
        kb.add_label(15, "backlog".to_string());
        kb.add_blocks(12, 15);
        
        kb.solve();
        
        // Issue 15 should be blocked
        assert!(kb.program.is_blocked.iter().any(|(n,)| *n == 15));
        
        // Should NOT suggest working on 15
        let next_suggestions: Vec<u32> = kb.get_suggestions()
            .into_iter()
            .filter(|(_, s)| s.contains("Work on next"))
            .map(|(n, _)| n)
            .collect();
        
        assert!(!next_suggestions.contains(&15));
    }
}