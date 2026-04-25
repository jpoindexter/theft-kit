# Agent Handoff and Incident Response Patterns

Reference for multi-agent orchestration and incident management. Use when designing retry loops, structured handoffs, or incident response sequences.

Sources: Linear incident response methodology; Anthropic multi-agent design patterns (claude.ai/docs); SRE incident management frameworks (Google SRE Book, 2016).

---

## Bounded retry loop

Max 3 attempts. Each attempt is scoped. After 3 failures, escalate -- do not retry.

**QA FAIL message must include:**
- Exact issue description (not "it doesn't work")
- Expected output vs. actual output (both specified)
- Fix instructions (specific, not directional)
- Specific files involved

**Retry scope rule:**
Each retry message must say: "Fix ONLY the issues listed. Do NOT introduce new features or refactor unrelated code." This prevents scope expansion under the guise of fixing.

**After 3 failures:**
Produce an escalation report:
- Root cause hypothesis across all 3 attempts
- Pattern in how the failures differ (converging on the fix, or diverging?)
- Options: reassign, decompose the task, revise the approach, accept with documented limitations, defer

---

## Structured handoff template

These fields prevent context loss when an agent hands work to another agent or to a human.

| Field | Required | Notes |
|-------|----------|-------|
| Current state | Yes | What has been completed, not what was attempted |
| Relevant files | Yes | Paths + one-sentence description of what each file does |
| Dependencies and constraints | Yes | What cannot be changed and why |
| Acceptance criteria | Yes | Measurable. "Works correctly" is not acceptance criteria. |
| Evidence required | Yes | What proof of completion the receiving party needs |
| Output recipient | Yes | Who receives this output and what format they need |

A handoff without acceptance criteria passes ambiguity forward, not work.

---

## Incident severity classification

| Level | Definition | Response time |
|-------|-----------|--------------|
| P0 | Service down, data loss, security breach | Immediate |
| P1 | Major feature broken, 50%+ error rate | Under 1 hour |
| P2 | Minor feature broken, workaround exists | Under 4 hours |
| P3 | Cosmetic issue, minor inconvenience | Next sprint |

Classify at detection. Do not spend time debating severity during active mitigation. Reclassify after the incident if the initial classification was wrong.

---

## Incident response sequence

**Phase 1: Detection and triage (0-5 min)**
Classify severity. Assign an incident lead. Open a dedicated incident channel or thread. All communication goes there.

**Phase 2: Investigation (5-30 min)**
Parallel investigation: one person on the live system, one reviewing recent changes, one monitoring error rates. Do not serialize investigation. Do not wait for a root cause before starting mitigation.

**Phase 3: Mitigation (15-60 min)**
Fastest fix, not best fix. A revert is often faster than a forward fix. Document what you changed and why.

**Phase 4: Resolution verification**
Evidence of fix: screenshot, log excerpt, metric showing recovery. 30-minute monitoring window after mitigation before declaring resolved.

**Phase 5: Post-mortem (within 48 hours)**
Required for P0 and P1. Optional for P2.

Post-mortem format:
1. Timeline (what happened, minute by minute)
2. 5 Whys analysis (do not stop at the proximate cause)
3. Prevention measures with owners and deadlines
4. No blame. The system failed, not the person.

---

## Reality check QA pattern

Apply when reviewing any agent output, code review, or spec validation.

Default posture: "NEEDS WORK". Change only with specific evidence.

Automatic fail triggers:
- "Zero issues found" with no description of what was checked
- Perfect scores without cited evidence
- "Looks good" without a verification method stated

First implementations typically need 2-3 revision cycles. A first-pass "PASS" result from an agent on a non-trivial task is a signal to look harder, not to ship.

Evidence requirement: every claim needs a traceable source (file path, line number, test result, metric). Conclusions without evidence are opinions.

---

## Common handoff failure patterns

| Pattern | Symptom | Fix |
|---------|---------|-----|
| Acceptance criteria left undefined | Next agent interprets scope differently | Write measurable criteria before handoff |
| "Current state" describes intention, not completion | Receiving agent re-does work | State only what is verifiably done |
| Constraints not listed | Receiving agent breaks something upstream | List constraints explicitly, even obvious ones |
| Evidence format not specified | Output format mismatch causes rework | State the exact format the output recipient needs |
