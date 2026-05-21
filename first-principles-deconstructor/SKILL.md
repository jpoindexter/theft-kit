---
name: first-principles-deconstructor
description: Use when a user wants to deconstruct a complex problem, business challenge, product decision, strategy question, career crossroads, or stuck situation by stripping assumptions, identifying first principles, rebuilding options from foundational truths, and choosing one high-leverage action.
---

# First Principles Deconstructor

Use this skill to turn an overloaded or assumption-heavy problem into a clear decision foundation.

The goal is not to sound philosophical. The goal is to separate inherited assumptions from things that are actually true, then rebuild practical options from that smaller truth set.

## Trigger Question

If the user has not supplied a concrete problem, ask exactly:

> What problem, decision, or situation do you want me to deconstruct to its foundation?

If the user already supplied the problem, do not ask for permission. Run the workflow.

## Workflow

Use this five-phase structure. Keep language direct, concrete, and low on ceremony. Do not pad with generic advice.

### Phase 1: Assumption Autopsy

Identify assumptions embedded in how the user framed the problem.

Include:

- Explicit assumptions the user stated.
- Hidden assumptions implied by wording, constraints, urgency, or solution framing.
- Borrowed assumptions from industry norms, competitors, conventional advice, team politics, fear, sunk cost, or status expectations.
- Assumptions that might be true but still require proof.

Output format:

```markdown
**Phase 1: Assumption Autopsy**
- Assumption: ...
  Source: convention / competitor norm / fear / inherited process / unverified belief / stated constraint
  Why it matters: ...
```

### Phase 2: Irreducible Truths

Strip the situation down to what remains after assumptions are removed.

Only include claims that are:

- directly stated by the user,
- logically necessary,
- observable in the current situation,
- or clearly marked as requiring verification if not yet known.

Do not include "best practices," competitor behavior, prior approaches, social proof, or guesses as truths.

Output format:

```markdown
**Phase 2: Irreducible Truths**
1. ...
2. ...
3. ...
```

### Phase 3: Reconstruction From Zero

Using only the truths from Phase 2, rebuild possible solutions as if no conventional approach existed.

Generate three distinct approaches. Each approach must:

- start from a named first principle,
- explain the solution shape,
- state what it refuses to assume,
- include the smallest practical next test.

Output format:

```markdown
**Phase 3: Reconstruction From Zero**

1. Approach name
   First principle: ...
   Build: ...
   Refuses to assume: ...
   Smallest test: ...
```

### Phase 4: Assumption vs. Truth Map

Show how the original framing changes when replaced by first principles.

Use a compact comparison. The map should make clear where conventional thinking was pulling the user and where the truth set points instead.

Output format:

```markdown
**Phase 4: Assumption vs. Truth Map**
| Starting assumption | Replacement truth | What changes |
| --- | --- | --- |
| ... | ... | ... |
```

### Phase 5: The First-Principles Move

Choose the single highest-leverage action that follows from the deconstruction.

The move must be:

- specific,
- immediately executable,
- tied to one or more irreducible truths,
- and different from the obvious conventional next step when the analysis supports that.

Output format:

```markdown
**Phase 5: The First-Principles Move**

Do this: ...

Why this is the move: ...

Next 24 hours:
1. ...
2. ...
3. ...
```

## Quality Bar

- Be skeptical, not theatrical.
- Do not pretend uncertain claims are first principles.
- Do not over-index on the Aristotle branding; use it as a method, not a persona.
- If the user's framing lacks facts, explicitly mark the missing facts and still proceed with provisional truths.
- Prefer fewer, sharper truths over long lists.
- Avoid generic strategy grids unless they clarify the decision.
- End with action, not reflection.
