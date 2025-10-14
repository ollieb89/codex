# Epic 3.2 Sprint 1 - Implementation Plan

**Date**: October 10, 2025
**Sprint**: Sprint 1 (2 weeks)
**Story Points**: 29 SP
**Status**: 🟡 In Progress

---

## Executive Summary

This document details the implementation plan for Epic 3.2 Sprint 1, which delivers 4 core built-in agents and an agent registry. This sprint lays the foundation for the agent catalog system that will be expanded in subsequent sprints.

**Deliverables**:
- Review Agent (5 SP)
- Security Agent (8 SP)
- Performance Agent (8 SP)
- Refactor Agent (5 SP)
- Agent Registry (3 SP)
- 60+ comprehensive tests

---

## Architecture Analysis

### Existing Foundation (Epic 3.1)

Epic 3.1 provided a solid foundation for agent development:

**Agent Trait** (`core/src/agents/mod.rs`):
```rust
#[async_trait]
pub trait Agent: Send + Sync {
    fn id(&self) -> AgentId;
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn can_handle(&self, context: &TaskContext) -> ActivationScore;
    async fn execute(&self, task: Task, toolkit: &AgentToolkit) -> Result<AgentResult>;
    fn permissions(&self) -> &AgentPermissions;
    fn system_prompt(&self) -> &str;
}
```

**AgentResult Enum** (3 variants):
- `CodeReview { findings: Vec<CodeReviewFinding> }` - For review/security findings
- `Analysis { summary: String, details: HashMap<String, String> }` - For analysis results
- `Suggestions { items: Vec<Suggestion> }` - For refactoring suggestions

**AgentToolkit** (`core/src/agents/toolkit.rs`):
- `read_file()` - Read file with permission validation
- `write_file()` - Write file with permission validation
- `execute_command()` - Execute shell commands
- `workspace_root()` - Get workspace path

**Formatter** (`core/src/commands/agents/formatter.rs`):
- Supports 3 output formats: Markdown, JSON, PlainText
- Already handles all 3 AgentResult variants
- No changes needed for Sprint 1

### What Needs Implementation

1. **4 Built-in Agents** in `core/src/agents/builtin/`:
   - `review.rs` - Code quality analysis
   - `security.rs` - Vulnerability detection
   - `performance.rs` - Performance optimization
   - `refactor.rs` - Code improvement suggestions

2. **Agent Registry** in `core/src/agents/`:
   - `registry.rs` - Agent discovery and management

3. **Tests** in `core/tests/`:
   - Unit tests for each agent (10-15 per agent)
   - Integration tests for registry
   - E2E tests for agent execution

---

## Implementation Strategy

### Phase 1: Review Agent (Day 1-2, 5 SP)

**File**: `core/src/agents/builtin/review.rs`

**Purpose**: Comprehensive code review focusing on:
- Code quality and maintainability
- Best practices adherence
- Design patterns
- Code smells

**Implementation Approach**:

```rust
pub struct ReviewAgent {
    permissions: AgentPermissions,
}

impl ReviewAgent {
    pub fn new() -> Self {
        Self {
            permissions: AgentPermissions {
                file_access: FileAccessPolicy::ReadOnly,
                shell_execution: false,
                network_access: false,
                allowed_tools: vec![],
                max_iterations: 5,
                can_delegate: false,
            }
        }
    }

    /// Analyzes code files and generates review findings
    async fn analyze_code(&self, files: &[PathBuf], toolkit: &AgentToolkit)
        -> Result<Vec<CodeReviewFinding>> {
        // Implementation reads files and analyzes patterns
    }
}

#[async_trait]
impl Agent for ReviewAgent {
    fn id(&self) -> AgentId {
        AgentId::from("review")
    }

    fn name(&self) -> &str {
        "Code Review Agent"
    }

    fn description(&self) -> &str {
        "Performs comprehensive code review focusing on quality, \
         maintainability, and best practices"
    }

    fn can_handle(&self, context: &TaskContext) -> ActivationScore {
        let intent = context.user_intent.to_lowercase();
        let keywords = ["review", "check", "analyze", "quality", "lint"];

        let matches = keywords.iter()
            .filter(|k| intent.contains(*k))
            .count();

        ActivationScore::new(matches as f64 * 0.2)
    }

    async fn execute(&self, task: Task, toolkit: &AgentToolkit)
        -> Result<AgentResult> {
        let findings = self.analyze_code(
            &task.context.file_paths,
            toolkit
        ).await?;

        Ok(AgentResult::CodeReview { findings })
    }

    fn permissions(&self) -> &AgentPermissions {
        &self.permissions
    }

    fn system_prompt(&self) -> &str {
        "You are an expert code reviewer with deep knowledge of software \
         engineering best practices. Analyze code for quality, maintainability, \
         design patterns, and potential improvements. Focus on actionable \
         feedback with clear explanations."
    }
}
```

**Analysis Logic**:
The agent will analyze code by:
1. Reading each file using `toolkit.read_file()`
2. Applying heuristic rules for common issues:
   - Long functions (>50 lines)
   - High cyclomatic complexity
   - Duplicate code patterns
   - Magic numbers
   - Poor naming conventions
   - Missing error handling
3. Categorizing findings by severity (Error, Warning, Info)
4. Including file path and line numbers

**Tests** (`core/tests/test_review_agent.rs`):
- `test_review_agent_activation_scoring()` - Test context matching
- `test_review_agent_identifies_long_functions()` - Detect complexity
- `test_review_agent_finds_magic_numbers()` - Detect constants
- `test_review_agent_checks_error_handling()` - Validate error patterns
- `test_review_agent_respects_permissions()` - Permission enforcement
- `test_review_agent_formats_findings_correctly()` - Result structure
- `test_review_agent_handles_empty_files()` - Edge case
- `test_review_agent_handles_binary_files()` - Error handling
- `test_review_agent_multiple_severities()` - Severity classification
- `test_review_agent_integration()` - Full execution flow

---

### Phase 2: Security Agent (Day 3-4, 8 SP)

**File**: `core/src/agents/builtin/security.rs`

**Purpose**: Security vulnerability detection:
- SQL injection risks
- XSS vulnerabilities
- Authentication/authorization flaws
- Cryptographic issues
- Dependency vulnerabilities

**Implementation Approach**:

```rust
pub struct SecurityAgent {
    permissions: AgentPermissions,
    vulnerability_patterns: HashMap<String, VulnerabilityPattern>,
}

struct VulnerabilityPattern {
    name: String,
    pattern: regex::Regex,
    severity: Severity,
    cve_references: Vec<String>,
    description: String,
}

impl SecurityAgent {
    pub fn new() -> Self {
        let mut patterns = HashMap::new();

        // SQL Injection patterns
        patterns.insert(
            "sql_injection".to_string(),
            VulnerabilityPattern {
                name: "SQL Injection".to_string(),
                pattern: regex::Regex::new(
                    r#"execute.*\+.*["']|query.*format\(|SELECT.*\+.*FROM"#
                ).unwrap(),
                severity: Severity::Error,
                cve_references: vec!["CWE-89".to_string()],
                description: "Potential SQL injection vulnerability...".to_string(),
            },
        );

        // Add more patterns for XSS, auth, crypto, etc.

        Self {
            permissions: AgentPermissions {
                file_access: FileAccessPolicy::ReadOnly,
                shell_execution: false, // Security agent doesn't need shell
                network_access: false,
                allowed_tools: vec![],
                max_iterations: 10,
                can_delegate: false,
            },
            vulnerability_patterns: patterns,
        }
    }

    async fn scan_for_vulnerabilities(&self, files: &[PathBuf], toolkit: &AgentToolkit)
        -> Result<Vec<CodeReviewFinding>> {
        // Scan files for security patterns
    }
}
```

**Security Patterns**:
- **SQL Injection**: String concatenation in queries
- **XSS**: Unescaped user input in HTML/JS
- **Auth Bypass**: Missing authorization checks
- **Crypto**: Weak algorithms (MD5, SHA1), hardcoded secrets
- **SSRF**: User-controlled URLs in HTTP requests

**Tests** (15 unit + 5 integration):
- Pattern matching tests for each vulnerability type
- False positive avoidance tests
- Multi-language support tests
- CVE reference validation
- Remediation guidance tests

---

### Phase 3: Performance Agent (Day 5-6, 8 SP)

**File**: `core/src/agents/builtin/performance.rs`

**Purpose**: Performance optimization analysis:
- Algorithmic complexity (O(n), O(n²), etc.)
- Memory allocation issues
- I/O bottlenecks
- Concurrency opportunities

**Implementation Approach**:

```rust
pub struct PerformanceAgent {
    permissions: AgentPermissions,
}

impl PerformanceAgent {
    async fn analyze_complexity(&self, code: &str) -> ComplexityAnalysis {
        // Analyze loops, nested structures, recursion
        ComplexityAnalysis {
            time_complexity: "O(n²)".to_string(),
            space_complexity: "O(n)".to_string(),
            hot_paths: vec![],
            optimization_opportunities: vec![],
        }
    }
}
```

**Analysis Focus**:
- **Nested Loops**: Detect O(n²) and O(n³) patterns
- **Unnecessary Allocations**: Vec clones, String allocations in loops
- **I/O in Loops**: Database queries, file reads in iterations
- **Synchronous Blocking**: Missing async/await opportunities
- **Cache Opportunities**: Repeated expensive computations

**Tests** (12 unit + 4 integration):
- Complexity detection tests
- Optimization suggestion tests
- Benchmark recommendation tests
- Before/after example validation

---

### Phase 4: Refactor Agent (Day 7-8, 5 SP)

**File**: `core/src/agents/builtin/refactor.rs`

**Purpose**: Code improvement suggestions:
- Extract method opportunities
- Reduce complexity
- Apply design patterns
- Deduplicate code

**Implementation Approach**:

```rust
pub struct RefactorAgent {
    permissions: AgentPermissions,
}

impl RefactorAgent {
    async fn identify_refactoring_opportunities(&self, code: &str)
        -> Vec<Suggestion> {
        // Analyze code structure for improvements
    }
}
```

**Returns**: `AgentResult::Suggestions { items }`

**Refactoring Patterns**:
- **Extract Method**: Long functions, repeated code blocks
- **Extract Variable**: Complex expressions
- **Introduce Parameter Object**: Long parameter lists
- **Replace Conditional with Polymorphism**: Complex if/else chains
- **Consolidate Duplicate Code**: DRY violations

**Tests** (10 unit + 3 integration):
- Pattern detection tests
- Suggestion quality tests
- Estimated effort calculation
- Code change generation tests

---

### Phase 5: Agent Registry (Day 9-10, 3 SP)

**File**: `core/src/agents/registry.rs`

**Purpose**: Central registry for agent discovery and management

**Implementation Approach**:

```rust
pub struct AgentRegistry {
    agents: HashMap<AgentId, Box<dyn Agent>>,
}

impl AgentRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            agents: HashMap::new(),
        };

        // Register built-in agents
        registry.register(Box::new(ReviewAgent::new()));
        registry.register(Box::new(SecurityAgent::new()));
        registry.register(Box::new(PerformanceAgent::new()));
        registry.register(Box::new(RefactorAgent::new()));

        registry
    }

    pub fn register(&mut self, agent: Box<dyn Agent>) {
        self.agents.insert(agent.id(), agent);
    }

    pub fn get(&self, id: &AgentId) -> Option<&dyn Agent> {
        self.agents.get(id).map(|a| a.as_ref())
    }

    pub fn list(&self) -> Vec<&dyn Agent> {
        self.agents.values().map(|a| a.as_ref()).collect()
    }

    pub fn find_best_match(&self, context: &TaskContext) -> Option<&dyn Agent> {
        // Find agent with highest activation score above threshold
        let threshold = 0.5;

        self.agents.values()
            .map(|agent| (agent.as_ref(), agent.can_handle(context)))
            .filter(|(_, score)| score.0 >= threshold)
            .max_by(|(_, a), (_, b)| a.0.partial_cmp(&b.0).unwrap())
            .map(|(agent, _)| agent)
    }
}
```

**Tests** (8 unit tests):
- `test_registry_register_agent()` - Registration
- `test_registry_get_agent()` - Retrieval
- `test_registry_list_all_agents()` - Listing
- `test_registry_find_best_match()` - Context matching
- `test_registry_activation_threshold()` - Threshold filtering
- `test_registry_duplicate_id_handling()` - Error cases
- `test_registry_empty_registry()` - Edge cases
- `test_registry_agent_metadata()` - Metadata access

---

## File Structure

```
codex-rs/
├── core/
│   ├── src/
│   │   ├── agents/
│   │   │   ├── builtin/
│   │   │   │   ├── mod.rs                    [MODIFY]
│   │   │   │   ├── review.rs                 [NEW - 250 lines]
│   │   │   │   ├── security.rs               [NEW - 400 lines]
│   │   │   │   ├── performance.rs            [NEW - 350 lines]
│   │   │   │   └── refactor.rs               [NEW - 300 lines]
│   │   │   ├── mod.rs                        [MODIFY - export registry]
│   │   │   ├── registry.rs                   [NEW - 200 lines]
│   │   │   ├── permissions.rs                [NO CHANGE]
│   │   │   ├── toolkit.rs                    [NO CHANGE]
│   │   │   └── router.rs                     [MODIFY - use registry]
│   │   └── lib.rs                            [NO CHANGE]
│   └── tests/
│       ├── test_review_agent.rs              [NEW - 350 lines]
│       ├── test_security_agent.rs            [NEW - 450 lines]
│       ├── test_performance_agent.rs         [NEW - 400 lines]
│       ├── test_refactor_agent.rs            [NEW - 380 lines]
│       └── test_agent_registry.rs            [NEW - 250 lines]
└── docs/
    ├── EPIC_3.2_SPRINT_1_IMPLEMENTATION_PLAN.md  [THIS FILE]
    └── EPIC_3.2_SPRINT_1_COMPLETION.md           [FUTURE]
```

**Total New Code**: ~3,330 lines
**Total Tests**: ~1,830 lines
**Ratio**: 55% test code (high quality)

---

## Testing Strategy

### Unit Tests (Per Agent)

**Review Agent** (10 tests):
1. Activation scoring with various keywords
2. Long function detection
3. Magic number detection
4. Error handling validation
5. Permission enforcement
6. Result formatting
7. Empty file handling
8. Binary file error handling
9. Severity classification
10. Multiple file processing

**Security Agent** (15 tests):
1. SQL injection pattern detection
2. XSS vulnerability detection
3. Auth bypass detection
4. Weak crypto detection
5. Hardcoded secret detection
6. SSRF detection
7. False positive avoidance
8. Multi-language support (Rust, Python, JS)
9. CVE reference inclusion
10. Remediation guidance
11. Severity assessment
12. Pattern regex validation
13. Edge cases (commented code, strings)
14. Performance of pattern matching
15. Integration with vulnerability DB

**Performance Agent** (12 tests):
1. O(n²) detection
2. O(n³) detection
3. Memory allocation analysis
4. I/O bottleneck detection
5. Missing async opportunities
6. Cache opportunity detection
7. Complexity calculation
8. Hot path identification
9. Optimization ranking
10. Before/after examples
11. Benchmark suggestions
12. Multi-function analysis

**Refactor Agent** (10 tests):
1. Extract method detection
2. Long parameter list detection
3. Duplicate code detection
4. Complex conditional detection
5. Suggestion generation
6. Effort estimation
7. Benefit calculation
8. Code change generation
9. Priority ranking
10. Pattern application validation

### Integration Tests

**Agent Registry** (8 tests):
- Complete registration flow
- Multi-agent discovery
- Best match selection
- Threshold behavior
- Error handling

**E2E Tests** (5 tests):
- Full agent execution pipeline
- Command → Agent → Result → Format
- Multi-agent scenarios
- Performance benchmarks
- Error recovery

---

## Dependencies

### Existing Crates (No Changes)
- `anyhow` - Error handling
- `async-trait` - Async trait support
- `tokio` - Async runtime
- `serde` / `serde_json` - Serialization

### New Dependencies (Add to Cargo.toml)
```toml
[dependencies]
regex = "1.10"           # Security pattern matching
lazy_static = "1.4"      # Static regex compilation
```

---

## Quality Gates

### Code Quality
- [ ] All code passes `cargo fmt`
- [ ] All code passes `cargo clippy --tests`
- [ ] No compiler warnings
- [ ] Documentation comments on all public items

### Testing
- [ ] All unit tests pass (60+ tests)
- [ ] All integration tests pass (8 tests)
- [ ] E2E tests pass (5 tests)
- [ ] Test coverage >90%

### Performance
- [ ] Agent activation <10μs
- [ ] Agent execution <3s (p95)
- [ ] Registry lookup <1μs
- [ ] No memory leaks

### Documentation
- [ ] All agents have system prompts
- [ ] API documentation complete
- [ ] Example usage provided
- [ ] Troubleshooting guide

---

## Risk Mitigation

### Technical Risks

**Risk 1: Agent execution timeout**
- **Impact**: High
- **Mitigation**: Implement timeout in toolkit, add cancellation support

**Risk 2: Pattern matching performance**
- **Impact**: Medium
- **Mitigation**: Use lazy_static for regex compilation, benchmark critical paths

**Risk 3: False positives in security scanning**
- **Impact**: Medium
- **Mitigation**: Comprehensive test suite, tuneable severity thresholds

### Schedule Risks

**Risk 1: Agent implementation complexity**
- **Impact**: High
- **Mitigation**: Start with simplified logic, iterate based on feedback

**Risk 2: Test coverage takes longer than expected**
- **Impact**: Medium
- **Mitigation**: Prioritize critical path tests, defer edge case tests to Sprint 2

---

## Sprint Timeline

| Day | Task | SP | Status |
|-----|------|----|----|
| 1-2 | Review Agent implementation + tests | 5 | 🟡 Pending |
| 3-4 | Security Agent implementation + tests | 8 | 🟡 Pending |
| 5-6 | Performance Agent implementation + tests | 8 | 🟡 Pending |
| 7-8 | Refactor Agent implementation + tests | 5 | 🟡 Pending |
| 9 | Agent Registry implementation + tests | 3 | 🟡 Pending |
| 10 | Integration testing, polish, documentation | - | 🟡 Pending |

**Total**: 29 SP over 10 working days (2 weeks)

---

## Success Criteria

### Sprint 1 Complete When:
- ✅ 4 agents implemented and tested
- ✅ Agent registry functional
- ✅ 70+ tests passing (unit + integration + E2E)
- ✅ All quality gates met
- ✅ Documentation updated
- ✅ No regressions in Epic 3.1 functionality

### User-Facing Success:
- Users can `/review` code and get actionable findings
- Users can `/security-audit` and see vulnerability reports
- Users can `/performance-analysis` and get optimization suggestions
- Users can `/refactor` and receive improvement recommendations
- All agents discoverable via command palette with 🤖 icon

---

## Next Steps (Post-Sprint 1)

**Sprint 2** (Week 3): Advanced Formatting
- Syntax highlighting integration
- Interactive clickable elements
- Rich output formatting
- Export functionality

**Sprint 3** (Week 4): Additional Agents
- Test Agent (8 SP)
- Explain Agent (3 SP)
- Document Agent (5 SP)

**Sprint 4** (Week 5): Plugin Architecture
- Plugin manifest spec
- Plugin loader
- Plugin CLI
- Developer documentation

---

**Document Version**: 1.0
**Last Updated**: 2025-10-10
**Status**: Ready for Implementation
**Next Review**: After Sprint 1 Day 5 (mid-sprint checkpoint)
