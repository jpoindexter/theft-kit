# Agent Handoff & Incident Response Patterns

## Bounded Retry Loop (3-attempt max)
- QA FAIL includes: exact issue description, expected vs actual, fix instructions, specific files
- Each retry is scoped: "Fix ONLY the issues listed, do NOT introduce new features"
- After 3 failures: escalation report with root cause analysis across all 3 attempts
- Escalation options: reassign, decompose, revise approach, accept with limitations, defer

## Structured Handoff Template (agent-to-agent)
Key fields that prevent context loss:
- Current state (what's been completed)
- Relevant files with descriptions
- Dependencies and constraints
- Acceptance criteria (measurable)
- Evidence required (proof of completion)
- Who receives output next and what format they need

## Incident Severity Classification
| Level | Definition | Response Time |
|-------|-----------|---------------|
| P0 | Service down, data loss, security breach | Immediate |
| P1 | Major feature broken, 50%+ error rate | < 1 hour |
| P2 | Minor feature broken, workaround exists | < 4 hours |
| P3 | Cosmetic, minor inconvenience | Next sprint |

## Incident Response Sequence
1. Detection & Triage (0-5 min): classify severity
2. Investigation (5-30 min): parallel investigation
3. Mitigation (15-60 min): fastest fix, not best fix
4. Resolution Verification: evidence of fix, 30-min monitoring
5. Post-Mortem (within 48h): timeline, 5 Whys, prevention measures

## Reality Checker QA Pattern
- Default to "NEEDS WORK"
- Automatic fail triggers: "zero issues found", perfect scores without evidence
- First implementations typically need 2-3 revision cycles
- Evidence-based: every claim needs proof
