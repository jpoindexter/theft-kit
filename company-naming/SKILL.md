---
name: company-naming
description: "When the user wants help naming or renaming a company, product, app, or brand. Also use when the user says 'help me name this,' 'generate company names,' 'evaluate these names,' 'rename the company,' 'our name is too generic,' 'score these finalists,' 'we need a better brand name,' or wants a structured naming process instead of a raw brainstorm. Use this to run a naming sprint that maps category conventions, finds the deeper benefit, generates broad candidate territory, and evaluates finalists for distinctiveness, fluency, energy, and longevity."
metadata:
  version: 1.0.0
---

# Company Naming

You help users name or rename companies in a way that is strategic, distinctive, and durable.

The goal is not to dump a list of pleasant-sounding names. The goal is to help the user choose a name that stands out, feels right for the company's ambition, and can survive years of use.

Use this whenever a user needs to:
- create a company name from scratch
- rename an existing company or product
- evaluate finalists
- understand why a current name feels weak

## Workflow

### Step 1: Reuse Shared Context First

Before asking repeated questions, check whether these exist:
- `.agents/product-marketing-context.md`
- `.claude/product-marketing-context.md` for older setups
- `.agents/brand-page-context.md`
- `.claude/brand-page-context.md` for older setups

Use them for:
- category and product definition
- target audience
- emotional outcomes and desired impression
- brand voice and positioning
- known competitors and alternatives

If the user has not provided enough context and no shared context exists, gather only the missing inputs needed to name well:
- what the company does
- who it is for
- what feeling the name should create
- names they like or dislike and why

### Step 2: Map The Naming Landscape

Use [references/landscape-and-brief.md](references/landscape-and-brief.md).

Before generating names:
- list the main competitors or adjacent players
- identify category naming patterns
- define which naming territories are overused or off-limits
- note what would make the company blend in

Treat this as a "we are not going there" pass. The point is to eliminate obvious, generic category language before brainstorming.

### Step 3: Find The Ultimate Benefit

Use [references/benefit-ladder.md](references/benefit-ladder.md).

Do not stop at what the product does.

Climb the ladder:
- feature
- functional benefit
- emotional benefit
- ultimate benefit

Strong naming territory usually appears at the emotional or ultimate-benefit layer, not the literal feature layer.

If helpful, create or update:
- `artifacts/company-naming-brief.md`

That artifact should capture:
- what the company does
- who it serves
- the desired feeling
- the off-limits naming territory
- the ultimate-benefit concept

### Step 4: Run The Treasure Hunt

Use [references/name-generation-playbook.md](references/name-generation-playbook.md).

Generate a broad set of candidates before filtering.

Explore multiple territory types:
- evocative or metaphorical names
- invented words
- compound names
- Latin- or Greek-inspired fragments
- sensory or motion-based names
- names that could plausibly become verbs or category labels

Target at least 100 candidates for a light pass and 200 to 300 for a serious sprint.

Do not optimize for safe, literal, committee-friendly names too early.

### Step 5: Filter For Real-World Use

Use [references/finalist-scorecard.md](references/finalist-scorecard.md).

Say the names out loud and remove candidates that are:
- hard to pronounce
- hard to spell after hearing once
- too close to a competitor
- literally descriptive in a forgettable way
- too narrow for the company's likely future scope

If the user wants an early shortlist, narrow to 10 to 20.
If the user wants finalists, narrow to 3 to 5.

When helpful, create or update:
- `artifacts/company-name-shortlist.md`

### Step 6: Run The Tension Test

When evaluating finalists, ask:
- does this create a reaction, or does it disappear into the category?
- is it surprisingly familiar rather than generic?
- does it feel like the ambition of the company, not just the current feature set?

Do not automatically prefer the name that everyone finds "fine."

If one name is a little polarizing but clearly more distinctive and energetic, say so directly.

### Step 7: Final Recommendation

For each finalist, score:
- distinctiveness
- processing fluency
- emotional resonance
- energy
- longevity

Availability checks can be noted as a follow-up dimension, but do not present legal or trademark clearance as complete unless the user explicitly asks for formal research and the required tools or evidence are available.

The final recommendation should include:
1. the recommended winner
2. why it wins
3. the main risk or tradeoff
4. 2-4 alternates
5. optional domain or naming-angle notes when useful

## Deliverable Shapes

Use one of these depending on the ask:

### Naming Brief
- positioning summary
- naming territory to avoid
- ultimate-benefit territory
- desired feel
- generation constraints

### Candidate Pack
- grouped candidate list by naming style or territory
- short rationale for the strongest options

### Finalist Evaluation
- shortlist table or scored list
- strongest reactions
- clear winner plus tradeoffs

## Guardrails

- Do not default to literal descriptive names unless the user explicitly wants that tradeoff
- Do not confuse "professional" with "forgettable"
- Do not optimize for consensus at the expense of distinctiveness
- Do not overstate domain, handle, or trademark availability
- Do not present bland AI list dumps without explaining the strategic territory behind them
- Prefer a few strong naming directions over dozens of weak near-duplicates

## Cross-References

- **product-marketing-context**: Use first when the company positioning is still fuzzy
- **voice-of-customer-synthesis**: Use when customer language should shape naming territory
- **copywriting**: Use after naming when the company needs taglines, homepage copy, or brand story
- **brand-page-context**: Use after naming when the visual direction should evolve with the chosen name

## What Good Looks Like

A strong naming output should make the team say:
- "These options feel more distinctive than our category defaults"
- "We understand why the recommended name fits"
- "We have a real shortlist, not a random brainstorm"
- "We know which tradeoffs we are accepting"

A weak output:
- only generates literal names
- ignores competitors and category sameness
- treats all finalists as equally good
- never makes a recommendation
