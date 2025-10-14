# Command & Agent System - Sprint Planning Materials

## Table of Contents

1. [Sprint Planning Templates](#sprint-planning-templates)
2. [Quality Assurance Checklists](#quality-assurance-checklists)
3. [Daily Execution Templates](#daily-execution-templates)
4. [Sprint Review Materials](#sprint-review-materials)
5. [Retrospective Frameworks](#retrospective-frameworks)
6. [Sprint 0 Specific Materials](#sprint-0-specific-materials)

---

## Sprint Planning Templates

### 1.1 Sprint Planning Agenda

**Duration:** 2 hours
**Participants:** Development team, Product Owner (if applicable), Scrum Master

```markdown
# Sprint [X] Planning - [Date]

## Agenda (2 hours)

### Part 1: Sprint Goal & Backlog Review (45 min)
- [ ] Review sprint goal from roadmap
- [ ] Review completed stories from previous sprint
- [ ] Discuss any carry-over items
- [ ] Review top priority backlog items

### Part 2: Story Sizing & Commitment (45 min)
- [ ] Size each story (Planning Poker)
- [ ] Identify dependencies
- [ ] Assess risks
- [ ] Determine sprint capacity
- [ ] Commit to sprint backlog

### Part 3: Task Breakdown & Assignment (30 min)
- [ ] Break stories into tasks
- [ ] Estimate task hours (2-4 hours ideal)
- [ ] Assign initial tasks
- [ ] Identify quality gates

## Sprint Goal
[One sentence describing what we aim to achieve]

## Sprint Metrics
- **Capacity:** [X] story points / [Y] hours
- **Committed Stories:** [N]
- **Quality Target:** ≥80% test coverage, 0 critical bugs

## Risks Identified
1. [Risk 1] - Mitigation: [Strategy]
2. [Risk 2] - Mitigation: [Strategy]

## Notes
[Any important decisions or discussions]
```

---

### 1.2 Story Sizing Reference (Fibonacci)

Use this guide for consistent story point estimation:

| Points | Complexity | Time Estimate | Example |
|--------|-----------|---------------|---------|
| **1** | Trivial | 2-4 hours | Update documentation, fix typo |
| **2** | Simple | 4-8 hours | Add simple validation, update config |
| **3** | Moderate | 1-2 days | Implement basic feature, refactor module |
| **5** | Complex | 2-4 days | New component with tests, integration |
| **8** | Very Complex | 1 week | New subsystem, complex algorithm |
| **13** | Epic | 1-2 weeks | Major feature, needs breakdown |
| **20+** | Too Large | - | Split into smaller stories |

**Factors to Consider:**
- Code complexity
- Testing requirements
- Integration points
- Risk/uncertainty
- Documentation needs

**Sizing Tips:**
- Compare to previous stories
- Consider whole team capacity
- Include testing and review time
- Account for unknowns (add buffer)

---

### 1.3 Definition of Ready (DoR) Checklist

Stories must meet these criteria BEFORE entering a sprint:

```markdown
## Story: [Story Name]

### Requirements ✓
- [ ] User story written in standard format: "As a [user], I want [goal], so that [benefit]"
- [ ] Acceptance criteria clearly defined (3-7 criteria)
- [ ] Dependencies identified and documented
- [ ] Technical approach discussed and agreed

### Design & Planning ✓
- [ ] Architecture impact assessed
- [ ] UI/UX design available (if applicable)
- [ ] Data model changes identified
- [ ] API changes documented (if applicable)

### Quality & Testing ✓
- [ ] Test strategy defined (unit, integration, E2E)
- [ ] Performance requirements specified
- [ ] Security considerations reviewed
- [ ] Non-functional requirements clear

### Estimation & Priority ✓
- [ ] Story sized (points assigned)
- [ ] Priority confirmed
- [ ] Team capacity verified
- [ ] No blocking dependencies

**Ready for Sprint?** ☐ Yes ☐ No (if No, list missing items)
```

---

### 1.4 Sprint Backlog Template

```markdown
# Sprint [X] Backlog - [Start Date] to [End Date]

## Sprint Goal
[One clear, achievable goal for this sprint]

## Committed Stories (Total: [X] points)

### Story 1: [Story Name] ([Y] points)
**Priority:** High | Medium | Low
**Owner:** [Name]

**Acceptance Criteria:**
1. [ ] [Criterion 1]
2. [ ] [Criterion 2]
3. [ ] [Criterion 3]

**Tasks:**
- [ ] Task 1.1 - [Owner] - [2h] - [Description]
- [ ] Task 1.2 - [Owner] - [4h] - [Description]
- [ ] Task 1.3 - [Owner] - [3h] - [Description]

**Quality Gates:**
- [ ] Unit tests ≥80% coverage
- [ ] Integration test written
- [ ] Code review completed
- [ ] Performance benchmark passes

---

### Story 2: [Story Name] ([Y] points)
[... repeat structure ...]

---

## Sprint Calendar

| Day | Ceremony | Focus |
|-----|----------|-------|
| Mon Week 1 | Sprint Planning | Backlog commitment |
| Tue-Fri W1 | Daily Standup | Development |
| Mon Week 2 | Mid-Sprint Sync | Progress check |
| Tue-Thu W2 | Daily Standup | Development |
| Fri Week 2 | Sprint Review | Demo & feedback |
| Fri Week 2 | Retrospective | Improvement planning |

## Sprint Risks

| Risk | Impact | Probability | Mitigation |
|------|--------|-------------|------------|
| [Risk 1] | High | Medium | [Strategy] |
| [Risk 2] | Medium | High | [Strategy] |

## Notes & Decisions
- [Important decision 1]
- [Important decision 2]
```

---

### 1.5 Task Breakdown Template

```markdown
## Story: [Story Name]

### Task Breakdown (Total: [X] hours)

#### Task 1: [Task Name] - [Est: 2h]
**Owner:** [Name]
**Dependencies:** None | [Task X]

**Description:**
[What needs to be done]

**Implementation Notes:**
- [Key technical detail 1]
- [Key technical detail 2]

**Acceptance Criteria:**
- [ ] [Specific deliverable 1]
- [ ] [Specific deliverable 2]

**Quality Checks:**
- [ ] Unit tests written
- [ ] Code follows style guide
- [ ] Documentation updated

**Definition of Done:**
- [ ] Code implemented
- [ ] Tests passing
- [ ] Code reviewed
- [ ] Documentation complete

---

[Repeat for each task]

---

### Dependencies Map

```
Task 1 → Task 2 → Task 4
      ↘ Task 3 ↗
```

### Parallel Execution Opportunities
- Tasks 1 and 3 can run in parallel
- Tasks 2 and 5 can run in parallel

### Risk Areas
- Task 2: Complex algorithm, may need spike
- Task 4: Integration with execpolicy, validation needed
```

---

## Quality Assurance Checklists

### 2.1 Definition of Done (DoD) Checklist

Every story must meet ALL criteria before marking complete:

```markdown
## Story: [Story Name]

### Code Quality ✓
- [ ] Code implemented and working
- [ ] Follows Rust coding standards (clippy rules)
- [ ] No `unsafe` code (or justified and reviewed)
- [ ] No `unwrap()`/`expect()` without error handling
- [ ] Formatted with `cargo fmt`
- [ ] All clippy warnings resolved

### Testing ✓
- [ ] Unit tests written (≥80% coverage for new code)
- [ ] Integration tests added (if applicable)
- [ ] E2E test added (if user-facing feature)
- [ ] All tests passing locally
- [ ] All tests passing in CI
- [ ] Edge cases covered
- [ ] Error cases tested

### Performance ✓
- [ ] Performance benchmarks run
- [ ] No degradation (≤5% acceptable)
- [ ] Memory usage validated
- [ ] Meets specific targets (if defined)

### Code Review ✓
- [ ] PR created with clear description
- [ ] Code reviewed by ≥1 team member
- [ ] All review comments addressed
- [ ] Approved by reviewer

### Documentation ✓
- [ ] Code comments for complex logic
- [ ] Public API documented (rustdoc)
- [ ] User-facing docs updated (if needed)
- [ ] CHANGELOG.md updated (if user-facing)

### Integration ✓
- [ ] Integrates with existing codebase
- [ ] No breaking changes (or properly communicated)
- [ ] Feature flags used (if phased rollout)
- [ ] Backward compatible (if applicable)

### Security & Safety ✓
- [ ] No obvious security vulnerabilities
- [ ] Input validation in place
- [ ] execpolicy integration (if executing commands)
- [ ] Permission model enforced (if applicable)

### Acceptance ✓
- [ ] All acceptance criteria met
- [ ] Demoed to team (if significant feature)
- [ ] Product owner accepts (if applicable)

**Story Complete?** ☐ Yes ☐ No (if No, list incomplete items)
```

---

### 2.2 Code Review Checklist (Rust-Specific)

Use this checklist when reviewing Rust code:

```markdown
# Code Review Checklist - PR #[Number]

## Reviewer: [Name] | Date: [Date]

### Code Quality ✓
- [ ] **Naming:** Clear, descriptive variable/function names
- [ ] **Structure:** Logical code organization
- [ ] **Complexity:** Functions are reasonably small (<100 lines)
- [ ] **Duplication:** No copy-paste code, proper abstractions
- [ ] **Comments:** Complex logic is explained

### Rust Best Practices ✓
- [ ] **Error Handling:** `Result<T, E>` used, no `unwrap()` in prod code
- [ ] **Ownership:** Proper use of borrowing, no unnecessary clones
- [ ] **Lifetimes:** Lifetime annotations correct and minimal
- [ ] **Traits:** Appropriate trait usage, no over-engineering
- [ ] **Unsafe:** No unsafe blocks (or justified with SAFETY comment)
- [ ] **Generics:** Generic code is constrained appropriately

### Testing ✓
- [ ] **Coverage:** New code has ≥80% test coverage
- [ ] **Test Quality:** Tests are clear, independent, repeatable
- [ ] **Edge Cases:** Boundary conditions tested
- [ ] **Error Paths:** Error handling tested
- [ ] **Integration:** Integration tests for cross-module features

### Performance ✓
- [ ] **Algorithms:** Efficient algorithms used (O(n) vs O(n²))
- [ ] **Allocations:** Unnecessary allocations avoided
- [ ] **Cloning:** Clones justified, borrowing preferred
- [ ] **Async:** Async code doesn't block unnecessarily

### Security ✓
- [ ] **Input Validation:** User input validated and sanitized
- [ ] **execpolicy:** Command execution goes through policy
- [ ] **File Access:** File operations respect permissions
- [ ] **Dependencies:** New deps are vetted and necessary

### Project-Specific (Codex) ✓
- [ ] **TUI Style:** Follows `tui/styles.md` guidelines (if TUI code)
- [ ] **Permissions:** Agent permissions enforced (if agent code)
- [ ] **Templates:** Command templates validated (if command code)
- [ ] **Sandbox:** Sandbox boundaries respected

### Documentation ✓
- [ ] **Public API:** All public items have rustdoc comments
- [ ] **Examples:** Complex APIs have usage examples
- [ ] **User Docs:** User-facing features documented
- [ ] **Architecture:** ADR created (if architectural change)

## Review Outcome
- ☐ **Approve** - Ready to merge
- ☐ **Request Changes** - Issues must be addressed
- ☐ **Comment** - Suggestions for improvement

## Comments
[Detailed feedback, suggestions, or concerns]

## Approval
**Reviewer Signature:** [Name] | **Date:** [Date]
```

---

### 2.3 Testing Checklist

```markdown
# Testing Checklist - [Feature/Story Name]

## Unit Tests ✓
- [ ] **Happy Path:** Normal operation tested
- [ ] **Edge Cases:** Boundary values tested
- [ ] **Error Cases:** Error handling tested
- [ ] **Coverage:** ≥80% line coverage
- [ ] **Isolation:** Tests are independent
- [ ] **Speed:** Tests run in <1s (except integration)

### Example Unit Test Structure
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_happy_path() {
        // Arrange
        let input = create_test_input();

        // Act
        let result = function_under_test(input);

        // Assert
        assert_eq!(result, expected_value);
    }

    #[test]
    fn test_error_case() {
        let invalid_input = create_invalid_input();
        assert!(function_under_test(invalid_input).is_err());
    }
}
```

## Integration Tests ✓
- [ ] **Cross-Module:** Module interactions tested
- [ ] **API Contracts:** Interfaces validated
- [ ] **Data Flow:** End-to-end data flow verified
- [ ] **Real Dependencies:** Uses real (not mocked) dependencies where appropriate

### Example Integration Test
```rust
#[tokio::test]
async fn test_command_to_agent_flow() {
    let registry = CommandRegistry::new(test_dir()).await.unwrap();
    let router = AgentRouter::new();

    let input = "/review src/main.rs";
    let result = execute_with_agent(input, &registry, &router).await.unwrap();

    assert!(matches!(result, AgentResult::CodeReview { .. }));
}
```

## E2E Tests ✓
- [ ] **User Workflows:** Complete user scenarios tested
- [ ] **UI Integration:** TUI interactions validated
- [ ] **System Integration:** All components working together
- [ ] **Real Environment:** Tests run in realistic conditions

### Example E2E Test
```rust
#[tokio::test]
async fn test_complete_review_workflow() {
    // Setup complete environment
    let workspace = create_test_workspace();
    let app = App::new(workspace).await;

    // Simulate user interaction
    app.send_input("/review src/auth.rs").await;
    app.wait_for_completion().await;

    // Validate complete flow
    let findings = app.get_agent_findings();
    assert!(!findings.is_empty());
}
```

## Performance Tests ✓
- [ ] **Benchmarks:** Performance benchmarks written
- [ ] **Targets Met:** Meets defined performance targets
- [ ] **No Regression:** No >10% degradation from baseline
- [ ] **Load Testing:** Handles expected load

### Benchmark Template
```rust
#[bench]
fn bench_template_expansion(b: &mut Bencher) {
    let expander = TemplateExpander::new();
    let context = create_benchmark_context();

    b.iter(|| {
        expander.expand(TEMPLATE, &context)
    });
}
```

## Test Results Summary

| Test Type | Count | Passed | Failed | Coverage |
|-----------|-------|--------|--------|----------|
| Unit | [N] | [X] | [Y] | [Z%] |
| Integration | [N] | [X] | [Y] | - |
| E2E | [N] | [X] | [Y] | - |
| Benchmark | [N] | [X] | [Y] | - |

**Overall Test Status:** ☐ Pass ☐ Fail

## CI/CD Validation ✓
- [ ] All tests pass in CI
- [ ] No flaky tests
- [ ] Test execution time acceptable (<5 min total)
- [ ] Coverage report generated
```

---

### 2.4 Performance Validation Checklist

```markdown
# Performance Validation - [Feature Name]

## Performance Targets

| Metric | Target | Actual | Status |
|--------|--------|--------|--------|
| Command execution | <100ms | [X]ms | ☐ Pass ☐ Fail |
| Agent activation | <100ms | [X]ms | ☐ Pass ☐ Fail |
| Template expansion | <50ms | [X]ms | ☐ Pass ☐ Fail |
| Multi-agent workflow | <2s | [X]s | ☐ Pass ☐ Fail |
| TUI frame rate | >60fps (16ms) | [X]fps | ☐ Pass ☐ Fail |
| Memory usage | <100MB | [X]MB | ☐ Pass ☐ Fail |

## Benchmark Results

### Template Expansion Benchmark
```
test bench_template_expansion ... bench: [X] ns/iter (+/- [Y])
```
**Analysis:** [Performance analysis]

### Agent Activation Benchmark
```
test bench_agent_activation ... bench: [X] ns/iter (+/- [Y])
```
**Analysis:** [Performance analysis]

## Profiling Results
- [ ] CPU profiling performed
- [ ] Memory profiling performed
- [ ] Hot paths identified
- [ ] Optimization opportunities documented

## Regression Analysis
- [ ] Compared to baseline
- [ ] No >10% degradation
- [ ] Improvements documented

## Optimization Notes
[Any performance optimizations applied or recommended]

**Performance Validation:** ☐ Pass ☐ Fail
```

---

## Daily Execution Templates

### 3.1 Daily Standup Format

```markdown
# Daily Standup - [Date]

**Time:** 15 minutes max
**Format:** Round-robin, time-boxed (2 min per person)

## [Team Member 1]

### Yesterday ✓
- [ ] Completed: [Task/Story completed]
- [ ] Progress: [Task in progress, % complete]

### Today 🎯
- [ ] Focus: [Primary task for today]
- [ ] Goal: [What will be complete by EOD]

### Blockers 🚫
- [ ] Blocker 1: [Description] - Needs: [Who/What]
- [ ] None

### Quality Note 📊
- Test coverage: [X%]
- Code review: [Pending/Complete]

---

[Repeat for each team member]

---

## Team Status Summary

### Sprint Progress
- **Completed:** [X] / [Y] story points
- **In Progress:** [Z] stories
- **Blocked:** [N] stories

### Quality Metrics
- **Test Coverage:** [X%]
- **Open PRs:** [N]
- **Pending Reviews:** [N]
- **CI Status:** ☐ Green ☐ Yellow ☐ Red

### Action Items
1. [ ] [Action 1] - Owner: [Name] - Due: [Date]
2. [ ] [Action 2] - Owner: [Name] - Due: [Date]
```

---

### 3.2 Blocker Escalation Process

```markdown
# Blocker Escalation - [Blocker ID]

## Blocker Details
**Reported By:** [Name]
**Date Reported:** [Date]
**Severity:** ☐ Critical ☐ High ☐ Medium ☐ Low

### Description
[Clear description of the blocker]

### Impact
- **Stories Blocked:** [N]
- **Team Members Affected:** [Names]
- **Sprint Goal Impact:** ☐ High ☐ Medium ☐ Low

### Root Cause
[What is causing this blocker?]

## Resolution Steps

### Immediate Actions (Today)
1. [ ] [Action 1] - Owner: [Name]
2. [ ] [Action 2] - Owner: [Name]

### Short-term (This Week)
1. [ ] [Action 1] - Owner: [Name]
2. [ ] [Action 2] - Owner: [Name]

### Escalation Path
- [ ] Team can resolve (no escalation)
- [ ] Tech Lead involvement needed
- [ ] External dependency (escalate to PM)
- [ ] Architecture change required (escalate to Arch team)

## Workaround
[Temporary solution to unblock, if available]

## Prevention
[How can we prevent this blocker in the future?]

**Status:** ☐ Open ☐ In Progress ☐ Resolved
```

---

### 3.3 Code Review Assignment Template

```markdown
# Code Review Assignments - Week [X]

## Review Schedule

| PR # | Author | Reviewer | Complexity | Priority | Due Date | Status |
|------|--------|----------|-----------|----------|----------|--------|
| #123 | Alice | Bob | Medium | High | [Date] | ☐ Pending |
| #124 | Bob | Charlie | High | High | [Date] | ☐ Pending |
| #125 | Charlie | Alice | Low | Medium | [Date] | ☐ Complete |

## Review Guidelines

### Priority Levels
- **High:** Blocking other work, review within 4 hours
- **Medium:** Important but not blocking, review within 1 day
- **Low:** Nice to have, review within 2 days

### Complexity Levels
- **Low:** <100 lines, straightforward changes
- **Medium:** 100-500 lines, some complexity
- **High:** >500 lines or complex logic, may need pairing

### Review Capacity
- Each reviewer: Max 2-3 reviews per day
- High complexity: Allocate 1-2 hours
- Medium: 30-60 minutes
- Low: 15-30 minutes

## Review Quality Checklist
Use the [Code Review Checklist](#22-code-review-checklist-rust-specific) for all reviews.

## SLA Tracking
- [ ] All high priority reviews completed within 4 hours
- [ ] All medium priority reviews completed within 1 day
- [ ] No PRs waiting >2 days

**Review Health:** ☐ Good ☐ Needs Attention ☐ Critical
```

---

### 3.4 Technical Debt Log

```markdown
# Technical Debt Log - Sprint [X]

## New Debt Added

### Debt Item 1: [Description]
**Date Added:** [Date]
**Added By:** [Name]
**Reason:** [Why this debt was incurred]

**Impact:**
- Maintainability: ☐ High ☐ Medium ☐ Low
- Performance: ☐ High ☐ Medium ☐ Low
- Security: ☐ High ☐ Medium ☐ Low

**Estimated Effort to Fix:** [X] hours/points

**Proposed Solution:**
[How to address this debt]

**Priority:** ☐ Must Fix ☐ Should Fix ☐ Nice to Have

---

## Debt Reduced This Sprint

### Debt Item 2: [Description]
**Original Date:** [Date]
**Resolved:** [Date]
**Resolved By:** [Name]

**Solution Applied:**
[What was done to address the debt]

**Lessons Learned:**
[What we learned from this]

---

## Outstanding Debt Summary

| Item | Age (Sprints) | Priority | Estimated Effort |
|------|---------------|----------|------------------|
| [Item 1] | 3 | Must Fix | 8 hours |
| [Item 2] | 1 | Should Fix | 3 hours |

**Debt Trend:** ☐ Decreasing ☐ Stable ☐ Increasing

## Debt Reduction Plan
[Strategy for addressing accumulated debt]
```

---

## Sprint Review Materials

### 4.1 Sprint Review Agenda

```markdown
# Sprint [X] Review - [Date]

**Duration:** 1 hour
**Attendees:** Dev team, stakeholders, product owner

## Agenda

### 1. Sprint Summary (5 min)
- Sprint goal
- Committed vs completed
- Key achievements

### 2. Feature Demos (35 min)
- [Feature 1] - Demo by [Name] (10 min)
- [Feature 2] - Demo by [Name] (10 min)
- [Feature 3] - Demo by [Name] (10 min)
- Q&A (5 min)

### 3. Quality Metrics (10 min)
- Test coverage
- Performance benchmarks
- Code quality metrics
- Bug status

### 4. Challenges & Learnings (5 min)
- What went well
- What was challenging
- Key learnings

### 5. Feedback & Next Steps (5 min)
- Stakeholder feedback
- Backlog adjustments
- Next sprint preview

## Notes
[Important discussions and decisions]
```

---

### 4.2 Sprint Summary Template

```markdown
# Sprint [X] Summary

## Sprint Goal
[What we aimed to achieve]

**Goal Achieved:** ☐ Yes ☐ Partially ☐ No

## Story Completion

| Story | Points | Status | Notes |
|-------|--------|--------|-------|
| [Story 1] | 5 | ✅ Complete | Exceeded expectations |
| [Story 2] | 3 | ✅ Complete | - |
| [Story 3] | 8 | 🔄 Partial | Carry over 3 points |
| [Story 4] | 2 | ❌ Not Started | Deprioritized |

**Total:** [X] / [Y] points completed ([Z%])

## Quality Achievements

### Test Coverage
- **Sprint Start:** [X%]
- **Sprint End:** [Y%]
- **Change:** +[Z%] ✓

### Performance
- All benchmarks passing ✓
- No performance regressions ✓
- Template expansion: [X]ms (target: <50ms) ✓

### Code Quality
- Clippy warnings: 0 ✓
- Code reviews: [N] completed ✓
- Documentation: Updated ✓

## Key Deliverables
1. ✅ [Deliverable 1] - [Brief description]
2. ✅ [Deliverable 2] - [Brief description]
3. 🔄 [Deliverable 3] - [Status/reason]

## Challenges Overcome
1. [Challenge 1] - [How we solved it]
2. [Challenge 2] - [How we solved it]

## Learnings
1. [Key learning 1]
2. [Key learning 2]

## Next Sprint Preview
[Brief overview of what's coming in next sprint]
```

---

### 4.3 Demo Script Template

```markdown
# Demo Script - [Feature Name]

## Preparation (Before Review)
- [ ] Demo environment ready
- [ ] Test data prepared
- [ ] Edge cases identified
- [ ] Backup plan if demo fails

## Demo Flow (10 minutes)

### 1. Context Setting (1 min)
**Say:** "This sprint we implemented [feature]. The goal was to [objective]."

**Show:** [Screenshot/diagram of before state]

### 2. Feature Demo (6 min)

#### Scenario 1: Happy Path
**Say:** "Let's start with the most common use case..."

**Do:**
1. [Action 1]
2. [Action 2]
3. [Action 3]

**Show:** [Expected result]

#### Scenario 2: Edge Case
**Say:** "Now let's see how it handles [edge case]..."

**Do:**
1. [Action 1]
2. [Action 2]

**Show:** [Expected result]

#### Scenario 3: Error Handling
**Say:** "And here's how it handles errors..."

**Do:**
1. [Trigger error condition]

**Show:** [Graceful error handling]

### 3. Quality Highlights (2 min)

**Say:** "Let's look at the quality metrics..."

**Show:**
- Test coverage: [X%]
- Performance: [benchmark results]
- Code quality: [clippy, reviews]

### 4. Q&A (1 min)
**Anticipated Questions:**
1. Q: [Question] → A: [Answer]
2. Q: [Question] → A: [Answer]

## Backup Plan
If demo fails: [Alternative approach or recorded demo]

## Talking Points
- [Key point 1 to emphasize]
- [Key point 2 to emphasize]
- [User benefit 1]
- [User benefit 2]
```

---

### 4.4 Metrics Dashboard Template

```markdown
# Sprint [X] Metrics Dashboard

## Velocity & Completion

```
Sprint Velocity Trend
30 ┤                     ╭─●
25 ┤              ╭──●──╯
20 ┤       ╭──●──╯
15 ┤──●───╯
10 ┤
   └──────────────────────
   S1  S2  S3  S4  S5  S6
```

- **This Sprint:** [X] points
- **Average Velocity:** [Y] points
- **Completion Rate:** [Z%]

## Test Coverage

```
Coverage by Module
commands   ████████████████████ 92%
agents     ███████████████████  87%
executor   █████████████████    80%
tui        ██████████████       75%
```

- **Overall Coverage:** [X%]
- **Target:** ≥80% ✓
- **Change from Last Sprint:** +[Y%]

## Code Quality

### Clippy Warnings Trend
```
10 ┤●
 8 ┤ ╲
 6 ┤  ●
 4 ┤   ╲
 2 ┤    ●
 0 ┤     ●───●
   └──────────────
   S1 S2 S3 S4 S5
```

- **This Sprint:** 0 warnings ✓
- **Code Reviews:** [N] completed
- **Average Review Time:** [X] hours

## Performance

| Metric | Target | Actual | Status |
|--------|--------|--------|--------|
| Command execution | <100ms | [X]ms | ✓ |
| Agent activation | <100ms | [X]ms | ✓ |
| Template expansion | <50ms | [X]ms | ✓ |
| TUI frame rate | >60fps | [X]fps | ✓ |

## Bug Tracking

- **Bugs Found:** [N]
- **Bugs Fixed:** [M]
- **Critical Bugs:** 0 ✓
- **Outstanding:** [K]

### Bug Trend
```
Critical: ●─────────────  (0)
High:     ●●────────────  (2)
Medium:   ●●●●──────────  (4)
Low:      ●●●●●●────────  (6)
```

## Technical Debt

- **Debt Added:** [X] items
- **Debt Reduced:** [Y] items
- **Net Change:** [Z] items
- **Debt Trend:** ☐ Decreasing ☐ Stable ☐ Increasing
```

---

## Retrospective Frameworks

### 5.1 Start/Stop/Continue Retrospective

```markdown
# Sprint [X] Retrospective - Start/Stop/Continue

**Date:** [Date]
**Duration:** 30 minutes
**Facilitator:** [Name]

## Instructions
1. Individual reflection (5 min): Write sticky notes
2. Group sharing (15 min): Share and cluster ideas
3. Voting (5 min): Vote on top items
4. Action planning (5 min): Define concrete actions

## Start (Things we should start doing)

### Quality-Related 🎯
- [ ] [Idea 1] - Votes: [X]
- [ ] [Idea 2] - Votes: [X]

### Process-Related ⚙️
- [ ] [Idea 1] - Votes: [X]

### Team-Related 👥
- [ ] [Idea 1] - Votes: [X]

## Stop (Things we should stop doing)

### Quality-Related 🎯
- [ ] [Idea 1] - Votes: [X]

### Process-Related ⚙️
- [ ] [Idea 1] - Votes: [X]

### Team-Related 👥
- [ ] [Idea 1] - Votes: [X]

## Continue (Things working well, keep doing)

### Quality-Related 🎯
- [ ] [Idea 1] - Votes: [X]
- [ ] [Idea 2] - Votes: [X]

### Process-Related ⚙️
- [ ] [Idea 1] - Votes: [X]

### Team-Related 👥
- [ ] [Idea 1] - Votes: [X]

## Action Items (Top 3)

### Action 1: [Start/Stop/Continue] [Idea]
**Owner:** [Name]
**Due Date:** [Date]
**Success Criteria:** [How we'll know it worked]

### Action 2: [Start/Stop/Continue] [Idea]
**Owner:** [Name]
**Due Date:** [Date]
**Success Criteria:** [How we'll know it worked]

### Action 3: [Start/Stop/Continue] [Idea]
**Owner:** [Name]
**Due Date:** [Date]
**Success Criteria:** [How we'll know it worked]

## Notes
[Important discussions or context]
```

---

### 5.2 Quality-Focused Retrospective

```markdown
# Sprint [X] Quality Retrospective

**Focus:** How well did we maintain quality this sprint?

## Test Quality 🧪

### What Went Well ✓
- [ ] Test coverage increased to [X%]
- [ ] [Specific test quality win]

### What Could Improve ⚠️
- [ ] [Test gap or issue]
- [ ] [Test quality concern]

### Actions 🎯
1. [ ] [Improvement action] - Owner: [Name]

---

## Code Quality 💎

### What Went Well ✓
- [ ] Zero clippy warnings maintained
- [ ] [Specific code quality win]

### What Could Improve ⚠️
- [ ] [Code quality concern]
- [ ] [Technical debt added]

### Actions 🎯
1. [ ] [Improvement action] - Owner: [Name]

---

## Review Quality 👀

### What Went Well ✓
- [ ] Reviews caught [N] issues before merge
- [ ] [Specific review win]

### What Could Improve ⚠️
- [ ] Review turnaround time: [X] hours (target: <24h)
- [ ] [Review process concern]

### Actions 🎯
1. [ ] [Improvement action] - Owner: [Name]

---

## Performance Quality ⚡

### What Went Well ✓
- [ ] All benchmarks passing
- [ ] [Specific performance win]

### What Could Improve ⚠️
- [ ] [Performance concern]
- [ ] [Optimization opportunity]

### Actions 🎯
1. [ ] [Improvement action] - Owner: [Name]

---

## Documentation Quality 📚

### What Went Well ✓
- [ ] [Documentation achievement]

### What Could Improve ⚠️
- [ ] [Documentation gap]

### Actions 🎯
1. [ ] [Improvement action] - Owner: [Name]

---

## Overall Quality Trend

**This Sprint:** ☐ Improved ☐ Maintained ☐ Declined

**Evidence:**
- Test coverage: [change]
- Code quality: [change]
- Performance: [change]
- Review quality: [change]

**Next Sprint Quality Goal:**
[Specific, measurable quality goal]
```

---

### 5.3 Mad/Sad/Glad Retrospective

```markdown
# Sprint [X] Retrospective - Mad/Sad/Glad

## Mad (What frustrated us) 😠

### Quality Issues
- [ ] [Frustration 1] - Impact: [Description]
- [ ] [Frustration 2] - Impact: [Description]

### Process Issues
- [ ] [Frustration 1] - Impact: [Description]

### Technical Issues
- [ ] [Frustration 1] - Impact: [Description]

**Root Causes:**
1. [Root cause 1]
2. [Root cause 2]

**Actions to Reduce Frustration:**
1. [ ] [Action 1] - Owner: [Name]
2. [ ] [Action 2] - Owner: [Name]

---

## Sad (What disappointed us) 😢

### Quality Misses
- [ ] [Disappointment 1]
- [ ] [Disappointment 2]

### Missed Opportunities
- [ ] [Disappointment 1]

**What Can We Learn:**
1. [Learning 1]
2. [Learning 2]

**Actions to Improve:**
1. [ ] [Action 1] - Owner: [Name]
2. [ ] [Action 2] - Owner: [Name]

---

## Glad (What made us happy) 😊

### Quality Wins
- [ ] [Success 1]
- [ ] [Success 2]

### Team Wins
- [ ] [Success 1]

### Technical Wins
- [ ] [Success 1]

**What Made This Possible:**
1. [Factor 1]
2. [Factor 2]

**Actions to Repeat:**
1. [ ] [Action 1] - Owner: [Name]
2. [ ] [Action 2] - Owner: [Name]

---

## Team Health Check

**Morale:** ☐ High ☐ Medium ☐ Low
**Collaboration:** ☐ Excellent ☐ Good ☐ Needs Work
**Confidence:** ☐ High ☐ Medium ☐ Low

**Overall Sprint Feeling:** [Emoji + brief description]
```

---

## Sprint 0 Specific Materials

### 6.1 Sprint 0 Planning Checklist

```markdown
# Sprint 0 Planning - Foundation & Risk Mitigation

## Pre-Sprint Preparation ✓
- [ ] Team assembled and roles assigned
- [ ] Development environment setup guide ready
- [ ] Repository access granted
- [ ] Tools installed (Rust, Cargo, IDE, etc.)

## Architecture & Design ✓
- [ ] Review specification document
- [ ] Architecture design session scheduled
- [ ] Module structure agreed
- [ ] Integration points identified
- [ ] Technology stack confirmed

## Technical Spikes Planning ✓

### Spike 1: Permission Model Integration
**Objective:** Validate execpolicy integration
**Time Box:** 6 hours
**Success Criteria:**
- [ ] AgentPermissions prototype works
- [ ] Integration with execpolicy validated
- [ ] Performance acceptable (<5ms)

### Spike 2: Template Engine Performance
**Objective:** Confirm <50ms template expansion
**Time Box:** 4 hours
**Success Criteria:**
- [ ] Handlebars benchmarked
- [ ] Meets performance target
- [ ] Memory usage acceptable

## Infrastructure Setup ✓
- [ ] CI/CD pipeline configured
- [ ] Test framework setup
- [ ] Code coverage tools installed
- [ ] Documentation structure created
- [ ] Issue tracking configured

## Risk Assessment ✓
- [ ] High-risk areas identified
- [ ] Mitigation strategies defined
- [ ] Contingency plans created
- [ ] Technical debt baseline established

## Sprint 0 Goals ✓
- [ ] Module structure created
- [ ] All spikes completed successfully
- [ ] Architecture validated
- [ ] Development environment ready
- [ ] Team aligned on approach

## Exit Criteria ✓
- [ ] Architecture approved by team
- [ ] All high-risk items de-risked
- [ ] Test infrastructure operational
- [ ] Documentation framework in place
- [ ] Ready to start Sprint 1
```

---

### 6.2 Technical Spike Template

```markdown
# Technical Spike: [Spike Name]

## Objective
[Clear statement of what we're trying to learn or validate]

## Time Box
**Duration:** [X] hours
**Due Date:** [Date]

## Success Criteria
1. [ ] [Specific criterion 1]
2. [ ] [Specific criterion 2]
3. [ ] [Specific criterion 3]

## Approach

### Research Phase (30%)
- [ ] Review relevant documentation
- [ ] Study similar implementations
- [ ] Identify key challenges

### Prototype Phase (50%)
- [ ] Implement proof of concept
- [ ] Test critical scenarios
- [ ] Measure performance/feasibility

### Documentation Phase (20%)
- [ ] Document findings
- [ ] Create recommendations
- [ ] Update architecture (if needed)

## Key Questions to Answer
1. [Question 1]
2. [Question 2]
3. [Question 3]

## Risks & Unknowns
- [Risk 1]
- [Risk 2]

## Deliverables
- [ ] Working prototype
- [ ] Spike findings document
- [ ] Recommendation for production implementation
- [ ] Updated architecture (if applicable)

## Implementation Notes

### What Worked
[Successful approaches, libraries, patterns]

### What Didn't Work
[Dead ends, unsuitable approaches]

### Performance Results
[Benchmark results, metrics]

### Recommendations
**Recommended Approach:**
[Detailed recommendation for production implementation]

**Alternative Approaches:**
1. [Alternative 1] - Pros/Cons
2. [Alternative 2] - Pros/Cons

**Risk Mitigation:**
[How to mitigate discovered risks]

## Decision
☐ Proceed with recommended approach
☐ Needs more investigation
☐ Pivot to alternative approach
☐ Not feasible

**Next Steps:**
1. [ ] [Action 1]
2. [ ] [Action 2]
```

---

### 6.3 Architecture Decision Record (ADR) Template

```markdown
# ADR [Number]: [Title]

**Date:** [YYYY-MM-DD]
**Status:** ☐ Proposed ☐ Accepted ☐ Rejected ☐ Deprecated

## Context
[What is the issue we're facing? What factors are driving this decision?]

## Decision
[What did we decide to do? Be specific and concrete.]

## Consequences

### Positive
- [Benefit 1]
- [Benefit 2]
- [Benefit 3]

### Negative
- [Trade-off 1]
- [Trade-off 2]

### Neutral
- [Implication 1]
- [Implication 2]

## Alternatives Considered

### Alternative 1: [Name]
**Pros:**
- [Pro 1]
- [Pro 2]

**Cons:**
- [Con 1]
- [Con 2]

**Why Rejected:**
[Reason for not choosing this]

### Alternative 2: [Name]
[... same structure ...]

## Implementation Notes
[Key implementation details, gotchas, patterns to follow]

## Related Decisions
- [Link to related ADR 1]
- [Link to related ADR 2]

## References
- [Link to relevant documentation]
- [Link to spike findings]
- [Link to external resources]
```

---

### 6.4 Sprint 0 Deliverables Checklist

```markdown
# Sprint 0 Deliverables - Quality Gate

## Code & Structure ✓
- [ ] Module structure created in `codex-rs/core/src/`
  - [ ] `commands/` module
  - [ ] `agents/` module
  - [ ] All subdirectories
- [ ] Dependencies added to `Cargo.toml`
- [ ] Project compiles successfully
- [ ] No compiler warnings

## Technical Spikes ✓
- [ ] Permission spike completed
  - [ ] Findings documented
  - [ ] Integration validated
  - [ ] Performance verified
- [ ] Template engine spike completed
  - [ ] Benchmarks run
  - [ ] Performance target met
  - [ ] Recommendations clear

## Testing Infrastructure ✓
- [ ] Test framework configured
  - [ ] Unit test structure
  - [ ] Integration test setup
  - [ ] E2E test framework
- [ ] Snapshot testing (insta) configured
- [ ] Coverage reporting enabled
- [ ] CI/CD pipeline running tests
- [ ] Benchmarking framework setup

## Documentation ✓
- [ ] Architecture document created
- [ ] Spike findings documented
- [ ] ADRs created for key decisions
- [ ] README updated
- [ ] Development guide created

## Quality Standards ✓
- [ ] Code review process defined
- [ ] Definition of Done created
- [ ] Definition of Ready created
- [ ] Test coverage targets set (≥80%)
- [ ] Performance benchmarks defined

## Team Alignment ✓
- [ ] Architecture reviewed with team
- [ ] Approach consensus reached
- [ ] Roles and responsibilities clear
- [ ] Sprint 1 backlog ready
- [ ] Development environment validated

## Risk Mitigation ✓
- [ ] High-risk areas identified
- [ ] Mitigation strategies defined
- [ ] Contingency plans created
- [ ] Risk score reduced by ≥50%

## Exit Criteria Validation ✓
- [ ] All spikes successful
- [ ] Architecture approved
- [ ] Tests running in CI
- [ ] Zero high-risk unknowns
- [ ] Team ready for Sprint 1

**Sprint 0 Complete:** ☐ Yes ☐ No (if No, list incomplete items)

**Recommendation:** ☐ Proceed to Sprint 1 ☐ Extend Sprint 0
```

---

## Appendix: Quick Reference Cards

### A.1 Story Sizing Quick Reference

```markdown
# Story Sizing Cheat Sheet

## Fibonacci Scale
1, 2, 3, 5, 8, 13, 20+

## Reference Stories (Codex Project)

**1 Point (2-4 hours):**
- Update documentation
- Fix simple bug
- Add logging statement
- Simple configuration change

**2 Points (4-8 hours):**
- Add input validation
- Simple refactor
- Update dependency
- Add unit tests to existing code

**3 Points (1-2 days):**
- Implement command parser
- Add new template helper
- Create simple agent
- TUI widget (basic)

**5 Points (2-4 days):**
- Command registry implementation
- Agent router logic
- Integration with exec_command
- TUI component (complex)

**8 Points (1 week):**
- Complete agent with toolkit
- Multi-agent orchestrator
- Hot-reload system
- E2E feature implementation

**13+ Points:**
- SPLIT INTO SMALLER STORIES!

## Sizing Factors
✓ Code complexity
✓ Testing effort
✓ Integration points
✓ Unknown/risk
✓ Documentation

## Tips
- Compare to past stories
- Include review/test time
- Add buffer for unknowns
- When in doubt, size up
```

---

### A.2 Daily Workflow Quick Reference

```markdown
# Daily Workflow Cheat Sheet

## Morning (30 min)
☐ Check CI/CD status
☐ Review overnight PRs
☐ Plan today's focus
☐ Daily standup (15 min)

## Development (4-6 hours)
☐ Pick task from sprint backlog
☐ Write tests first (TDD)
☐ Implement feature
☐ Run local tests
☐ Format & lint (`cargo fmt`, `cargo clippy`)

## Before PR (30 min)
☐ Run full test suite
☐ Check coverage (≥80%)
☐ Update documentation
☐ Self-review changes

## Code Review (1-2 hours)
☐ Create PR with clear description
☐ Request review
☐ Review others' PRs
☐ Address feedback

## End of Day (15 min)
☐ Push all work
☐ Update task status
☐ Note blockers
☐ Plan tomorrow

## Quality Checks
✓ Tests passing
✓ Coverage ≥80%
✓ Clippy clean
✓ Formatted
✓ Documented
```

---

### A.3 Quality Gates Quick Reference

```markdown
# Quality Gates Cheat Sheet

## Task Level
☐ Tests written
☐ Code reviewed
☐ Documentation updated

## Story Level
☐ All acceptance criteria met
☐ Definition of Done complete
☐ Demo-able

## Sprint Level
☐ Sprint goal achieved
☐ All stories complete or explicitly carried over
☐ Quality metrics met
☐ No critical bugs

## Project Level (Codex)
☐ Test coverage ≥80%
☐ Performance targets met:
  - Command exec <100ms
  - Agent activation <100ms
  - Template expansion <50ms
☐ Zero clippy warnings
☐ All PRs reviewed
☐ Documentation current

## Red Flags 🚩
✗ Skipping tests
✗ Disabling clippy
✗ Committing `unwrap()`
✗ PR without description
✗ >10% performance degradation
```

---

## Summary

This comprehensive set of sprint planning materials provides everything needed for quality-focused agile development:

✅ **Planning Templates** - Sprint planning, story sizing, backlog management
✅ **Quality Checklists** - DoD, DoR, code review, testing
✅ **Daily Templates** - Standups, blockers, reviews
✅ **Review Materials** - Demo scripts, metrics, summaries
✅ **Retrospective Frameworks** - Multiple formats for team improvement
✅ **Sprint 0 Specifics** - Foundation and risk mitigation materials

All templates are:
- **Practical:** Ready to use as-is or customize
- **Quality-Focused:** Emphasize testing, performance, code quality
- **Project-Specific:** Tailored to Codex and Rust development
- **Comprehensive:** Cover all phases of sprint lifecycle

**Usage:**
1. Copy templates to project wiki or shared docs
2. Customize for team preferences
3. Use consistently across sprints
4. Iterate and improve based on retrospectives
