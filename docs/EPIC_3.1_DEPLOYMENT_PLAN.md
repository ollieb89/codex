# Epic 3.1 Deployment Plan - Staging & Production

**Version**: 1.0
**Date**: October 10, 2025
**Status**: Ready for Staging Deployment
**Epic**: 3.1 - Command Agent System

---

## Table of Contents

1. [Executive Summary](#executive-summary)
2. [Staging Deployment](#staging-deployment)
3. [User Testing Strategy](#user-testing-strategy)
4. [UX Feedback Collection](#ux-feedback-collection)
5. [Performance Monitoring](#performance-monitoring)
6. [Production Rollout](#production-rollout)
7. [Rollback Plan](#rollback-plan)

---

## Executive Summary

Epic 3.1 (Command Agent System) is **production-ready** with 63/63 tests passing and all quality gates met. This document outlines the deployment strategy from staging through production, including user testing, feedback collection, and monitoring.

### Deployment Timeline

| Phase | Duration | Start Date | Status |
|-------|----------|------------|--------|
| Staging Deployment | 1 day | 2025-10-11 | 🟡 Pending |
| User Testing | 1 week | 2025-10-11 | 🟡 Pending |
| Feedback Analysis | 2 days | 2025-10-18 | 🟡 Pending |
| Production Deployment | 1 day | 2025-10-21 | 🟡 Pending |

### Success Criteria

- ✅ All 63 tests passing in staging
- ✅ 🤖 icon renders correctly across terminals
- ✅ Agent commands execute <100ms average
- ✅ Positive UX feedback (>70% satisfaction)
- ✅ No critical bugs in staging

---

## Staging Deployment

### Pre-Deployment Checklist

**Code Validation**:
- [ ] All 63 tests passing locally
- [ ] No compiler warnings (run `cargo clippy`)
- [ ] Code formatted (`just fmt`)
- [ ] Documentation up-to-date

**Environment Setup**:
- [ ] Staging environment available
- [ ] Test users configured
- [ ] LLM providers configured (OpenAI, Azure, Ollama)
- [ ] Telemetry/logging enabled

**Feature Flags** (if applicable):
- [ ] `enable_agent_commands` = true
- [ ] `show_agent_icons` = true
- [ ] `collect_agent_telemetry` = true

### Deployment Steps

**1. Build Release Binary** (15 min)
```bash
cd codex-rs
cargo build --release

# Verify binary
./target/release/codex --version
```

**2. Run Test Suite** (5 min)
```bash
# Run all tests
cargo test --all-features

# Expected: 63/63 passing
# - Core E2E: 10 tests
# - TUI: 14 tests
# - Agent framework: 16 tests
# - Formatter: 15 tests
# - Metadata: 8 tests
```

**3. Deploy to Staging** (10 min)
```bash
# Copy binary to staging server
scp ./target/release/codex staging:/opt/codex/bin/

# Update symlink
ssh staging "ln -sf /opt/codex/bin/codex /usr/local/bin/codex"

# Verify deployment
ssh staging "codex --version"
```

**4. Smoke Test** (10 min)
```bash
# On staging server
codex

# Test regular command
/explain src/main.rs

# Test agent command (should show 🤖 icon)
Ctrl+K
> review
[Select review command]
/review src/main.rs
```

**5. Validation Checks** (20 min)

| Check | Command | Expected Result |
|-------|---------|-----------------|
| TUI launches | `codex` | No errors, UI renders |
| Command palette | `Ctrl+K` | Opens, shows commands |
| Agent icon | `Ctrl+K`, scroll to review | 🤖 icon visible |
| Agent execution | `/review test.rs` | Returns structured results |
| Regular command | `/explain test.rs` | Works as before |
| Performance | `time codex exec "/review test.rs"` | <2s total |

### Post-Deployment Validation

**Automated Tests**:
```bash
# Run E2E tests against staging
CODEX_ENV=staging cargo test -p codex-core --test test_agent_e2e

# Expected: 10/10 passing
```

**Manual Validation**:
1. Open command palette (`Ctrl+K`)
2. Verify 🤖 icon appears for agent commands
3. Filter commands (type "rev")
4. Verify 🤖 icon persists after filtering
5. Execute agent command
6. Verify structured output with formatting

**Terminal Compatibility** (test each):
- [ ] iTerm2 (macOS)
- [ ] Terminal.app (macOS)
- [ ] Windows Terminal
- [ ] GNOME Terminal (Linux)
- [ ] Alacritty
- [ ] Kitty

### Common Issues & Solutions

**Issue**: 🤖 icon not rendering

**Solution**:
```bash
# Check terminal UTF-8 support
echo $LANG  # Should include UTF-8

# Test emoji rendering
echo "🤖 Test"

# If not supported, fallback to text label
export CODEX_AGENT_ICON="[AI]"
```

**Issue**: Agent command timing out

**Solution**:
```bash
# Check LLM provider configuration
codex config show

# Verify API key
codex auth status

# Check network connectivity
curl -I https://api.openai.com
```

---

## User Testing Strategy

### Test User Selection

**Criteria**:
- Mix of experience levels (beginner, intermediate, advanced)
- Variety of development languages (Rust, Python, JavaScript, Go)
- Different terminal emulators
- Geographic diversity (latency testing)

**Target**: 15-20 test users

### Testing Scenarios

**Scenario 1: Command Discovery**
```
Task: Find and execute the code review command
Steps:
1. Open Codex CLI
2. Press Ctrl+K
3. Observe command palette
4. Identify agent commands by 🤖 icon
5. Select "review" command
6. Execute on sample file

Questions:
- Did you notice the 🤖 icon?
- What did you think it meant?
- Was it helpful for identifying AI-powered commands?
```

**Scenario 2: Agent Result Usefulness**
```
Task: Use review command on real code
Steps:
1. Execute /review on your own code
2. Read the structured results
3. Follow one suggestion

Questions:
- Were the results more helpful than simple text?
- Could you act on the suggestions?
- Preferred format: Markdown, JSON, or Plain?
```

**Scenario 3: Performance Perception**
```
Task: Compare agent vs regular commands
Steps:
1. Execute /explain (regular)
2. Execute /review (agent)
3. Note perceived speed difference

Questions:
- Did agent commands feel slow?
- Was the wait time acceptable given the richer results?
- Any performance issues?
```

### Testing Protocol

**Week 1**: Controlled Testing
- Days 1-2: Internal team testing (5 users)
- Days 3-4: Beta users (10 users)
- Days 5-7: Extended beta (20 users)

**Feedback Cadence**:
- Daily: Bug reports (critical issues)
- End of week: Survey responses
- Continuous: Usage telemetry

---

## UX Feedback Collection

### Feedback Mechanisms

**1. In-App Survey** (First-Time Agent Command)

After executing an agent command for the first time:
```
╔════════════════════════════════════════════════════╗
║  Quick Feedback: Agent Commands (30 seconds)      ║
╠════════════════════════════════════════════════════╣
║                                                    ║
║  You just used an agent command (marked with 🤖)  ║
║                                                    ║
║  1. Did you notice the 🤖 icon?                   ║
║     [ ] Yes, it was clear                         ║
║     [ ] Yes, but unsure what it meant             ║
║     [ ] No, didn't notice                         ║
║                                                    ║
║  2. Were the results helpful?                     ║
║     [1] [2] [3] [4] [5]                          ║
║     Not helpful    Very helpful                   ║
║                                                    ║
║  3. Performance acceptable?                       ║
║     [ ] Yes  [ ] No  [ ] Didn't notice           ║
║                                                    ║
║  [Submit] [Skip] [Don't ask again]               ║
╚════════════════════════════════════════════════════╝
```

**Implementation**:
```rust
// Trigger after first agent command execution
if user_prefs.agent_command_count == 1 && !user_prefs.survey_completed {
    show_quick_survey();
}
```

**2. Usage Telemetry** (Opt-In)

Collect anonymized metrics:
```json
{
  "event": "agent_command_executed",
  "command": "review",
  "duration_ms": 850,
  "result_type": "code_review",
  "format": "markdown",
  "user_action": "completed",
  "terminal": "iTerm2",
  "timestamp": "2025-10-11T10:30:00Z"
}
```

**Metrics to Track**:
- Agent command usage frequency
- Completion rate (did user cancel or complete?)
- Time to first agent command use
- Agent vs regular command ratio
- Format preference (Markdown/JSON/Plain)
- Error rates by agent type

**3. GitHub Discussions**

Create discussion threads:
- "🤖 Agent Commands: Share Your Experience"
- "Feature Request: New Agent Ideas"
- "Bug Reports: Agent Command Issues"

**4. Exit Survey** (End of Testing Week)

```markdown
# Agent Command System - User Feedback Survey

## Icon & Visual Design
1. How clear was the 🤖 icon in identifying agent commands?
   - [ ] Very clear
   - [ ] Somewhat clear
   - [ ] Not clear
   - [ ] Didn't notice the icon

2. Would you prefer a different indicator?
   - [ ] Keep 🤖 icon
   - [ ] Use text label like "[AI]"
   - [ ] Use both icon and text
   - [ ] Use color coding instead

## Agent Results
3. Were agent results more useful than regular commands?
   - [ ] Much more useful
   - [ ] Somewhat more useful
   - [ ] About the same
   - [ ] Less useful

4. Which output format do you prefer?
   - [ ] Markdown (structured with headers)
   - [ ] JSON (machine-readable)
   - [ ] Plain text (simple)

5. What did you like most about agent commands?
   [Free text]

6. What needs improvement?
   [Free text]

## Performance
7. How would you rate agent command performance?
   - [ ] Very fast (no noticeable delay)
   - [ ] Acceptable (slight delay worth it for results)
   - [ ] Slow (noticeable wait time)
   - [ ] Too slow (frustrating)

## Overall
8. Would you continue using agent commands?
   - [ ] Definitely yes
   - [ ] Probably yes
   - [ ] Not sure
   - [ ] Probably no
   - [ ] Definitely no

9. Net Promoter Score: How likely are you to recommend Codex with agent commands?
   [0-10 scale]
```

### Feedback Analysis

**Quantitative Metrics**:
- Icon recognition rate (target: >80%)
- Satisfaction score (target: >4.0/5.0)
- Completion rate (target: >85%)
- NPS (target: >40)

**Qualitative Themes**:
- Common pain points
- Feature requests
- Unexpected use cases
- Workflow integration

**Decision Criteria**:
- If satisfaction <70%: Iterate on design before production
- If critical bugs found: Fix before production
- If performance issues: Optimize before production
- Otherwise: Proceed to production

---

## Performance Monitoring

### Metrics to Monitor

**1. Command Execution Metrics**

```rust
struct CommandMetrics {
    command_name: String,
    is_agent: bool,
    duration_ms: u64,
    result_size_bytes: usize,
    success: bool,
    error_type: Option<String>,
}
```

**Key Performance Indicators**:
| Metric | Target | Alert Threshold |
|--------|--------|-----------------|
| Agent command latency (p50) | <1000ms | >2000ms |
| Agent command latency (p95) | <2000ms | >5000ms |
| Command dispatch overhead | <100μs | >1ms |
| Error rate (agent) | <5% | >10% |
| Error rate (regular) | <2% | >5% |

**2. User Experience Metrics**

```rust
struct UXMetrics {
    time_to_first_agent_command: Duration,
    agent_command_frequency: f64, // per session
    command_palette_open_count: u32,
    agent_icon_click_rate: f64,
}
```

**Targets**:
- Time to first agent command: <5 min (discovery)
- Agent command usage: >20% of total commands
- Icon click rate: >10% (shows recognition)

**3. System Health Metrics**

```rust
struct HealthMetrics {
    agent_router_success_rate: f64,
    formatter_success_rate: f64,
    agent_activation_latency: Duration,
}
```

### Monitoring Setup

**1. Local Telemetry** (File-based)

```rust
// Write metrics to ~/.codex/telemetry/
struct TelemetryWriter {
    log_file: File,
    buffer: Vec<Metric>,
}

impl TelemetryWriter {
    fn record_command_execution(&mut self, metrics: CommandMetrics) {
        self.buffer.push(Metric::CommandExecution(metrics));
        if self.buffer.len() >= 100 {
            self.flush();
        }
    }
}
```

**2. Aggregation Script**

```python
#!/usr/bin/env python3
# scripts/analyze_telemetry.py

import json
import statistics
from pathlib import Path

def analyze_telemetry(telemetry_dir):
    metrics = load_metrics(telemetry_dir)

    # Agent command performance
    agent_latencies = [m['duration_ms'] for m in metrics if m['is_agent']]
    print(f"Agent command latency:")
    print(f"  p50: {statistics.median(agent_latencies)}ms")
    print(f"  p95: {statistics.quantiles(agent_latencies, n=20)[-1]}ms")

    # Usage patterns
    agent_count = sum(1 for m in metrics if m['is_agent'])
    total_count = len(metrics)
    print(f"Agent command usage: {agent_count/total_count*100:.1f}%")

    # Error rates
    agent_errors = sum(1 for m in metrics if m['is_agent'] and not m['success'])
    print(f"Agent error rate: {agent_errors/agent_count*100:.1f}%")

if __name__ == "__main__":
    analyze_telemetry(Path.home() / ".codex" / "telemetry")
```

**3. Dashboard** (Optional - Grafana)

If centralized monitoring is available:
- Real-time latency charts
- Error rate alerts
- Usage trends
- User funnel (discovery → first use → regular use)

### Performance Alerts

**Critical Alerts** (Page on-call):
- Agent error rate >20%
- Average latency >10s
- System crashes related to agents

**Warning Alerts** (Slack notification):
- Agent error rate >10%
- p95 latency >5s
- Icon rendering failures >5%

**Info Alerts** (Daily digest):
- Usage trends
- New error types
- Performance improvements

---

## Production Rollout

### Rollout Strategy

**Phase 1: Canary Deployment** (10% of users, 24 hours)
- Enable for 10% of traffic
- Monitor metrics closely
- Rollback if critical issues

**Phase 2: Gradual Rollout** (50% of users, 48 hours)
- Increase to 50% if Phase 1 successful
- Continue monitoring
- Gather additional feedback

**Phase 3: Full Deployment** (100% of users)
- Complete rollout
- Announce in release notes
- Monitor for 1 week

### Production Checklist

**Pre-Deployment**:
- [ ] All staging tests passed
- [ ] User feedback analyzed (>70% positive)
- [ ] Performance validated (<2s p95)
- [ ] Documentation updated
- [ ] Release notes prepared

**Deployment**:
- [ ] Binary built and tested
- [ ] Canary deployment (10%)
- [ ] Monitor for 24 hours
- [ ] Gradual rollout (50%)
- [ ] Monitor for 48 hours
- [ ] Full deployment (100%)

**Post-Deployment**:
- [ ] Monitor metrics for 1 week
- [ ] Address user feedback
- [ ] Document lessons learned
- [ ] Plan Epic 3.2 based on insights

### Release Notes

```markdown
# Codex CLI v0.X.X - Agent Commands

## 🤖 New: Intelligent Agent Commands

Codex now includes AI-powered agent commands that provide structured,
actionable insights beyond simple template expansion.

### What's New
- **Agent-backed commands** with 🤖 icon in command palette
- **Structured results** with categories, severity levels, and line numbers
- **Intelligent analysis** context-aware code review and suggestions
- **Rich formatting** in Markdown, JSON, or Plain text

### Available Agent Commands
- `/review` - Comprehensive code review with best practice checks
- `/refactor` - Refactoring suggestions and code quality improvements

### How to Use
1. Open command palette (`Ctrl+K`)
2. Look for commands with 🤖 icon
3. Execute like regular commands
4. Receive structured, actionable results

### Performance
- Command dispatch: <100μs overhead
- Agent execution: <2s average
- All tests passing: 63/63

See the [Agent Command User Guide](docs/COMMAND_AGENT_SYSTEM.md) for details.
```

---

## Rollback Plan

### Rollback Triggers

**Critical Issues** (immediate rollback):
- System crashes or data loss
- Agent error rate >50%
- Security vulnerabilities discovered
- Complete feature failure

**Major Issues** (rollback after 4 hours):
- Agent error rate >20%
- Average latency >10s
- Widespread user complaints
- Performance degradation affecting overall system

**Minor Issues** (fix forward, no rollback):
- Agent error rate <10%
- Individual agent failures
- UI inconsistencies
- Minor performance issues

### Rollback Procedure

**1. Immediate Rollback** (< 5 minutes)
```bash
# Revert to previous version
ssh production "ln -sf /opt/codex/bin/codex-v0.X.X-prev /usr/local/bin/codex"

# Restart services if needed
ssh production "systemctl restart codex-server"

# Verify rollback
ssh production "codex --version"
```

**2. Feature Flag Rollback** (< 1 minute)
```bash
# Disable agent commands via feature flag
codex config set --global enable_agent_commands=false

# Verify
codex config show | grep enable_agent_commands
```

**3. Communication**
- Notify users via status page
- Post in Discord/Slack
- Update GitHub issue
- Send email to beta users

**4. Post-Mortem**
- Document what went wrong
- Analyze root cause
- Plan fix
- Schedule re-deployment

---

## Success Metrics

### Deployment Success

**Staging Success** (required for production):
- ✅ 63/63 tests passing
- ✅ >70% user satisfaction
- ✅ <5% error rate
- ✅ No critical bugs

**Production Success** (after 1 week):
- ✅ >80% user satisfaction
- ✅ <3% error rate
- ✅ Agent commands >15% of usage
- ✅ No production incidents

### Long-Term Success (after 1 month)

- Agent commands >25% of total usage
- User retention improved
- NPS increase >5 points
- Feature requests for more agents

---

## Timeline & Responsibilities

| Phase | Owner | Duration | Dates |
|-------|-------|----------|-------|
| Staging Deployment | DevOps | 1 day | Oct 11 |
| User Testing | Product | 1 week | Oct 11-17 |
| Feedback Analysis | Product + UX | 2 days | Oct 18-19 |
| Production Deployment | DevOps | 1 day | Oct 21 |
| Monitoring & Support | Engineering | Ongoing | Oct 21+ |

---

## Appendix

### A. Test User Invitation Email

```
Subject: Beta Testing Invitation - Codex Agent Commands

Hi [Name],

You're invited to test a new feature in Codex CLI: Agent Commands!

What's New:
- AI-powered commands with structured, actionable results
- Easy identification via 🤖 icon in command palette
- Intelligent code review, refactoring suggestions, and more

How to Participate:
1. Update to staging version: [link]
2. Use Codex as normal for 1 week
3. Try agent commands (look for 🤖 icon)
4. Share feedback: [survey link]

Testing Period: Oct 11-17
Estimated Time: 30 min initial + normal usage

Thank you for helping make Codex better!

Questions? Reply to this email or join #codex-beta in Slack.
```

### B. Monitoring Query Examples

**Datadog**:
```
avg:codex.agent.latency{command:review} by {region}
sum:codex.agent.errors{} by {error_type}.as_count()
```

**Prometheus**:
```promql
histogram_quantile(0.95, rate(codex_agent_duration_seconds_bucket[5m]))
rate(codex_agent_errors_total[5m]) / rate(codex_agent_executions_total[5m])
```

### C. Feature Flag Configuration

```toml
# ~/.codex/config.toml
[features]
enable_agent_commands = true
show_agent_icons = true
collect_agent_telemetry = true
agent_timeout_ms = 5000
```

---

**Document Version**: 1.0
**Last Updated**: 2025-10-10
**Next Review**: After staging deployment (2025-10-11)
