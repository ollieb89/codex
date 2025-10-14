# Epic 3.2 Roadmap - Agent Catalog & Advanced Formatting

**Version**: 1.0
**Date**: October 10, 2025
**Status**: Planning
**Dependencies**: Epic 3.1 (Complete ✅)

---

## Table of Contents

1. [Epic Overview](#epic-overview)
2. [User Stories](#user-stories)
3. [Feature Breakdown](#feature-breakdown)
4. [Implementation Plan](#implementation-plan)
5. [Success Criteria](#success-criteria)
6. [Timeline & Resources](#timeline--resources)

---

## Epic Overview

### Vision

Expand the agent command system with a rich catalog of built-in agents and advanced formatting capabilities, making Codex the premier AI-powered development assistant with context-aware, specialized agents for every development task.

### Goals

1. **Agent Catalog**: Provide 7-10 production-ready agents covering common development workflows
2. **Advanced Formatting**: Deliver rich, interactive terminal output with syntax highlighting and clickable elements
3. **Foundation for Extensibility**: Establish plugin architecture for custom agent development

### Value Proposition

**For Users**:
- Access to specialized agents for every task (review, security, performance, refactor, test, explain, document)
- Beautiful, actionable results with syntax highlighting and interactive elements
- Faster workflows with intelligent, context-aware assistance

**For Organization**:
- Differentiation from competitors (first agent catalog in CLI tools)
- Foundation for agent marketplace ecosystem
- Increased user engagement and retention

###Dependencies

- ✅ Epic 3.1: Command agent system complete
- 🟡 Epic 3.1 deployed to production (Week 1)
- 🟡 User feedback incorporated (Week 1-2)

---

## User Stories

### Theme 1: Agent Catalog

**US-3.2.1: As a developer, I want to discover available agents**
```
Given I open the command palette
When I type a keyword like "security" or "performance"
Then I see all relevant agents with 🤖 icon and descriptions
And I can preview what each agent does before using it
```
**Acceptance Criteria**:
- Command palette shows agent metadata (name, description, expertise)
- Filtering works by keywords and agent capabilities
- Agent help command shows examples (`/review --help`)

**Story Points**: 3
**Priority**: P0 (Must Have)

---

**US-3.2.2: As a developer, I want comprehensive code review**
```
Given I have code that needs review
When I execute /review on a file or directory
Then I receive categorized findings:
  - Security issues
  - Performance problems
  - Code quality concerns
  - Best practice violations
And each finding includes severity, category, location, and recommendation
```
**Acceptance Criteria**:
- Review agent analyzes code structure and patterns
- Results include severity levels (Error, Warning, Info)
- Line numbers and file paths clickable (in terminals that support it)
- Recommendations are actionable and specific

**Story Points**: 5
**Priority**: P0 (Must Have)

---

**US-3.2.3: As a developer, I want security vulnerability detection**
```
Given I have code that may contain security issues
When I execute /security-audit
Then I receive security-specific findings:
  - SQL injection risks
  - XSS vulnerabilities
  - Authentication/authorization flaws
  - Data exposure risks
  - Dependency vulnerabilities
And each finding includes CVE references where applicable
```
**Acceptance Criteria**:
- Security agent focuses specifically on vulnerability detection
- Integrates with vulnerability databases (CVE, OWASP)
- Provides remediation guidance
- Supports multiple languages (Rust, Python, JavaScript, Go)

**Story Points**: 8
**Priority**: P0 (Must Have)

---

**US-3.2.4: As a developer, I want performance analysis**
```
Given I have code with potential performance issues
When I execute /performance-analysis
Then I receive performance-specific recommendations:
  - Algorithmic complexity issues
  - Memory allocation problems
  - I/O bottlenecks
  - Concurrency opportunities
And each recommendation includes complexity analysis (O(n), O(n²), etc.)
```
**Acceptance Criteria**:
- Performance agent analyzes time and space complexity
- Identifies hot paths and optimization opportunities
- Provides before/after examples
- Supports benchmarking suggestions

**Story Points**: 8
**Priority**: P1 (Should Have)

---

**US-3.2.5: As a developer, I want refactoring suggestions**
```
Given I have code that could be improved
When I execute /refactor
Then I receive refactoring suggestions:
  - Extract method opportunities
  - Reduce complexity recommendations
  - Design pattern applications
  - Code deduplication ideas
And each suggestion includes estimated effort and benefit
```
**Acceptance Criteria**:
- Refactor agent identifies code smells
- Suggests specific refactoring patterns
- Provides estimated improvement metrics
- Supports incremental refactoring

**Story Points**: 5
**Priority**: P1 (Should Have)

---

**US-3.2.6: As a developer, I want test generation assistance**
```
Given I have code that needs tests
When I execute /generate-tests
Then I receive:
  - Test cases for happy paths
  - Edge case tests
  - Error condition tests
  - Mock/stub suggestions for dependencies
And tests follow project conventions
```
**Acceptance Criteria**:
- Test agent generates runnable test code
- Detects test framework (pytest, Jest, cargo test)
- Follows project testing patterns
- Includes assertions and edge cases

**Story Points**: 8
**Priority**: P1 (Should Have)

---

**US-3.2.7: As a developer, I want code explanation**
```
Given I encounter unfamiliar code
When I execute /explain
Then I receive:
  - High-level purpose explanation
  - Line-by-line breakdown
  - Algorithm explanation
  - Related concepts and patterns
In language appropriate to my skill level
```
**Acceptance Criteria**:
- Explain agent provides multi-level explanations
- Includes visual diagrams (ASCII art for control flow)
- Links to relevant documentation
- Adapts explanation depth to context

**Story Points**: 3
**Priority**: P2 (Nice to Have)

---

**US-3.2.8: As a developer, I want documentation generation**
```
Given I have code that needs documentation
When I execute /document
Then I receive:
  - Function/method docstrings
  - Module-level documentation
  - README sections
  - API documentation
Following project documentation style
```
**Acceptance Criteria**:
- Document agent follows language conventions (JSDoc, rustdoc, etc.)
- Generates structured documentation
- Includes examples where appropriate
- Respects existing documentation style

**Story Points**: 5
**Priority**: P2 (Nice to Have)

---

### Theme 2: Advanced Formatting

**US-3.2.9: As a developer, I want syntax-highlighted code in results**
```
Given I receive agent results with code snippets
When the result is displayed
Then code snippets are syntax-highlighted
With colors appropriate to the file type
```
**Acceptance Criteria**:
- Code blocks include language detection
- Syntax highlighting uses terminal color capabilities
- Supports 10+ languages (Rust, Python, JS, Go, Java, etc.)
- Respects terminal color scheme (light/dark mode)

**Story Points**: 5
**Priority**: P0 (Must Have)

---

**US-3.2.10: As a developer, I want interactive, clickable results**
```
Given I receive agent results with file locations
When I click on a file path or line number
Then my editor opens to that exact location
```
**Acceptance Criteria**:
- File paths are clickable (terminals with OSC 8 support)
- Opens in configured editor (VS Code, Vim, etc.)
- Falls back gracefully for unsupported terminals
- Keyboard shortcuts for navigation

**Story Points**: 5
**Priority**: P1 (Should Have)

---

**US-3.2.11: As a developer, I want beautiful, structured output**
```
Given I receive agent results
When they are displayed
Then I see:
  - Clear section headers with icons
  - Color-coded severity levels
  - Aligned tables and lists
  - Progress indicators for long operations
```
**Acceptance Criteria**:
- Uses box-drawing characters for structure
- Color palette is configurable
- Works in 256-color and true-color terminals
- Graceful degradation for limited terminals

**Story Points**: 3
**Priority**: P0 (Must Have)

---

**US-3.2.12: As a developer, I want to export agent results**
```
Given I receive useful agent results
When I want to save them for later
Then I can export to:
  - Markdown file
  - JSON file
  - HTML report
  - PDF (via external tool)
```
**Acceptance Criteria**:
- Export command: `/review src/ --export results.md`
- Multiple format support
- Preserves formatting and structure
- Includes metadata (timestamp, command, version)

**Story Points**: 3
**Priority**: P2 (Nice to Have)

---

### Theme 3: Plugin Architecture Foundation

**US-3.2.13: As a developer, I want to create custom agents**
```
Given I have specialized needs not covered by built-in agents
When I create a custom agent following the plugin spec
Then I can:
  - Package it as a plugin
  - Install it locally
  - Use it like built-in agents
```
**Acceptance Criteria**:
- Agent plugin manifest format defined
- Plugin installation command (`codex plugin install`)
- Plugin validation and sandboxing
- Documentation for custom agent development

**Story Points**: 8
**Priority**: P2 (Nice to Have)

---

**US-3.2.14: As a developer, I want agent marketplace discovery**
```
Given community agents are available
When I want to find new agents
Then I can browse and search:
  - codex plugin search "accessibility"
  - codex plugin browse --category security
And I see ratings, downloads, and descriptions
```
**Acceptance Criteria**:
- Plugin registry/marketplace infrastructure
- Search and filtering capabilities
- Community ratings and reviews
- Security scanning for plugins

**Story Points**: 13
**Priority**: P3 (Future)

---

## Feature Breakdown

### Feature 1: Agent Catalog (36 story points)

**Components**:

**1.1 Review Agent** (5 SP)
- File: `core/src/agents/builtin/review.rs`
- Analyzes code structure, patterns, best practices
- Returns `CodeReview` results with categorized findings
- Tests: 10 unit tests + 3 integration tests

**1.2 Security Agent** (8 SP)
- File: `core/src/agents/builtin/security.rs`
- Vulnerability detection (SQL injection, XSS, etc.)
- CVE database integration
- Tests: 15 unit tests + 5 integration tests

**1.3 Performance Agent** (8 SP)
- File: `core/src/agents/builtin/performance.rs`
- Complexity analysis (big-O notation)
- Bottleneck identification
- Tests: 12 unit tests + 4 integration tests

**1.4 Refactor Agent** (5 SP)
- File: `core/src/agents/builtin/refactor.rs`
- Code smell detection
- Refactoring pattern suggestions
- Tests: 10 unit tests + 3 integration tests

**1.5 Test Agent** (8 SP)
- File: `core/src/agents/builtin/test_generator.rs`
- Test case generation
- Framework detection (pytest, Jest, cargo test)
- Tests: 15 unit tests + 5 integration tests

**1.6 Explain Agent** (3 SP)
- File: `core/src/agents/builtin/explain.rs`
- Multi-level explanations
- Algorithm visualization
- Tests: 8 unit tests + 2 integration tests

**1.7 Document Agent** (5 SP)
- File: `core/src/agents/builtin/document.rs`
- Docstring generation
- Style adherence (rustdoc, JSDoc, etc.)
- Tests: 10 unit tests + 3 integration tests

**1.8 Agent Registry** (3 SP)
- File: `core/src/agents/registry.rs`
- Agent discovery and loading
- Metadata management
- Tests: 8 unit tests

**Total**: 36 story points (~2 weeks, 2 developers)

---

### Feature 2: Advanced Formatting (16 story points)

**Components**:

**2.1 Syntax Highlighting** (5 SP)
- File: `core/src/commands/agents/formatter/syntax.rs`
- Integration with `syntect` or similar
- Language detection
- Terminal color support
- Tests: 8 unit tests

**2.2 Interactive Elements** (5 SP)
- File: `core/src/commands/agents/formatter/interactive.rs`
- OSC 8 hyperlinks for file paths
- Keyboard navigation
- Editor integration
- Tests: 6 unit tests + 2 integration tests

**2.3 Rich Output** (3 SP)
- File: `core/src/commands/agents/formatter/rich.rs`
- Box-drawing characters
- Color-coded severity
- Progress indicators
- Tests: 8 unit tests

**2.4 Export Functionality** (3 SP)
- File: `core/src/commands/agents/formatter/export.rs`
- Multi-format export (MD, JSON, HTML)
- Template system
- Tests: 6 unit tests

**Total**: 16 story points (~1 week, 1 developer)

---

### Feature 3: Plugin Architecture (21 story points)

**Components**:

**3.1 Plugin Manifest Spec** (2 SP)
- Document: `docs/PLUGIN_SPEC.md`
- TOML format definition
- Validation rules
- Example plugins

**3.2 Plugin Loader** (8 SP)
- File: `core/src/agents/plugin_loader.rs`
- Dynamic loading (.so, .dylib, .dll)
- Validation and sandboxing
- Error handling
- Tests: 10 unit tests + 3 integration tests

**3.3 Plugin CLI** (5 SP)
- File: `cli/src/commands/plugin.rs`
- Install, list, remove commands
- Local and remote plugins
- Tests: 8 unit tests

**3.4 Plugin Marketplace Foundation** (6 SP)
- API design: `docs/PLUGIN_MARKETPLACE_API.md`
- Registry infrastructure planning
- Security model
- Documentation

**Total**: 21 story points (~1.5 weeks, 1 developer)

---

## Implementation Plan

### Sprint 1 (Week 1-2): Core Agent Catalog

**Goals**: Deliver 4 essential agents

**Week 1**:
- Day 1-2: Review Agent implementation
- Day 3-4: Security Agent implementation
- Day 5: Integration testing

**Week 2**:
- Day 1-2: Performance Agent implementation
- Day 3-4: Refactor Agent implementation
- Day 5: Agent Registry + testing

**Deliverables**:
- 4 production-ready agents
- Agent registry for discovery
- 50+ tests passing
- Documentation updated

---

### Sprint 2 (Week 3): Advanced Formatting

**Goals**: Rich, beautiful terminal output

**Week 3**:
- Day 1-2: Syntax highlighting integration
- Day 3: Interactive elements (clickable paths)
- Day 4: Rich output formatting
- Day 5: Export functionality

**Deliverables**:
- Syntax-highlighted code blocks
- Clickable file paths (where supported)
- Beautiful structured output
- Export to MD/JSON/HTML
- 25+ tests passing

---

### Sprint 3 (Week 4): Additional Agents + Polish

**Goals**: Complete agent catalog

**Week 4**:
- Day 1-2: Test Agent implementation
- Day 3: Explain Agent implementation
- Day 4: Document Agent implementation
- Day 5: Integration testing + polish

**Deliverables**:
- 7 total agents
- Complete agent catalog
- E2E testing
- Performance optimization

---

### Sprint 4 (Week 5): Plugin Architecture

**Goals**: Foundation for extensibility

**Week 5**:
- Day 1: Plugin manifest spec
- Day 2-3: Plugin loader implementation
- Day 4: Plugin CLI commands
- Day 5: Documentation + examples

**Deliverables**:
- Plugin architecture
- Installation workflow
- Example custom agent
- Developer guide

---

## Success Criteria

### Functional Requirements

**Agent Catalog**:
- ✅ 7 agents implemented and tested
- ✅ Agent discovery via registry
- ✅ Consistent result format across agents
- ✅ Comprehensive test coverage (>90%)

**Advanced Formatting**:
- ✅ Syntax highlighting for 10+ languages
- ✅ Interactive elements work in supported terminals
- ✅ Export to 3+ formats
- ✅ Graceful degradation for limited terminals

**Plugin Architecture**:
- ✅ Plugin manifest spec defined
- ✅ Plugin loader working with sandboxing
- ✅ Example custom agent functional
- ✅ Developer documentation complete

### Non-Functional Requirements

**Performance**:
- Agent execution <3s (p95)
- Syntax highlighting adds <100ms
- Plugin loading <200ms

**Quality**:
- All tests passing (target: 150+ tests)
- No critical bugs
- Documentation complete

**User Experience**:
- >80% user satisfaction
- Agent discovery intuitive
- Results actionable and beautiful

---

## Timeline & Resources

### Timeline

| Sprint | Duration | Start Date | End Date | Deliverables |
|--------|----------|------------|----------|--------------|
| Sprint 1 | 2 weeks | 2025-10-21 | 2025-11-01 | 4 core agents |
| Sprint 2 | 1 week | 2025-11-04 | 2025-11-08 | Advanced formatting |
| Sprint 3 | 1 week | 2025-11-11 | 2025-11-15 | Complete catalog |
| Sprint 4 | 1 week | 2025-11-18 | 2025-11-22 | Plugin architecture |
| **Total** | **5 weeks** | | | **Epic 3.2 Complete** |

### Resource Requirements

**Engineering**:
- 2 senior engineers (full-time)
- 1 junior engineer (part-time for testing)

**Design**:
- UX designer (1 week for formatting design)

**Product**:
- Product manager (ongoing for prioritization)

**QA**:
- QA engineer (week 5 for comprehensive testing)

### Dependencies

**Internal**:
- Epic 3.1 deployed to production
- User feedback from Epic 3.1 incorporated
- Performance baseline established

**External**:
- Syntax highlighting library (syntect or similar)
- Terminal capability detection (terminfo)
- Plugin loading framework (libloading)

---

## Risks & Mitigations

### Technical Risks

**Risk**: Syntax highlighting performance impact
- **Probability**: Medium
- **Impact**: Medium
- **Mitigation**: Lazy loading, caching, async rendering

**Risk**: Plugin sandboxing complexity
- **Probability**: High
- **Impact**: High
- **Mitigation**: Start with simple sandboxing, iterate

**Risk**: Terminal compatibility issues
- **Probability**: High
- **Impact**: Low
- **Mitigation**: Graceful degradation, comprehensive testing

### Schedule Risks

**Risk**: Agent implementation takes longer than estimated
- **Probability**: Medium
- **Impact**: High
- **Mitigation**: Prioritize P0 agents, defer P2/P3

**Risk**: Plugin architecture scope creep
- **Probability**: Medium
- **Impact**: Medium
- **Mitigation**: Clear scope definition, MVP approach

---

## Epic 3.3 Preview

### Potential Features (Post-3.2)

1. **Agent Chaining**: Multi-agent workflows
   - `/review --then refactor --then test`
   - Pipeline composition

2. **Agent Learning**: Feedback-driven improvement
   - User feedback on agent suggestions
   - Agent fine-tuning based on accepted recommendations

3. **Collaborative Agents**: Multi-agent problem solving
   - Agents consult each other
   - Specialized agents for complex tasks

4. **Agent Marketplace**: Community contributions
   - Public plugin registry
   - Rating and review system
   - Curated collections

5. **Custom Agent SDK**: Framework for developers
   - Agent template generator
   - Testing framework for agents
   - CI/CD integration

---

## Appendix

### A. Agent Comparison Matrix

| Agent | Focus | Complexity | Languages | Output Type |
|-------|-------|------------|-----------|-------------|
| Review | Code quality | Medium | All | CodeReview |
| Security | Vulnerabilities | High | Web, Backend | CodeReview |
| Performance | Optimization | High | All | Analysis |
| Refactor | Maintainability | Medium | All | Suggestions |
| Test | Test coverage | High | All | Code |
| Explain | Understanding | Low | All | Analysis |
| Document | Documentation | Low | All | Code |

### B. Syntax Highlighting Library Options

| Library | Language | Pros | Cons |
|---------|----------|------|------|
| syntect | Rust | Native, fast, many languages | Large binary size |
| bat | CLI | Battle-tested, beautiful | External dependency |
| tree-sitter | C/Rust | Accurate, incremental | Complex integration |

### C. Terminal Capability Detection

```rust
use terminfo::Database;

fn supports_hyperlinks() -> bool {
    if let Ok(db) = Database::from_env() {
        db.get::<bool>("XT").unwrap_or(false) // OSC 8 support
    } else {
        false
    }
}

fn supports_true_color() -> bool {
    std::env::var("COLORTERM")
        .map(|v| v == "truecolor" || v == "24bit")
        .unwrap_or(false)
}
```

---

**Document Version**: 1.0
**Last Updated**: 2025-10-10
**Next Review**: After Epic 3.1 user feedback (2025-10-18)
