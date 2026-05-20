# Design Thinking

## Core Philosophy

Good design is invisible. It guides users naturally without calling attention to itself.

### Design Principles

1. **Purpose-Driven**: Every element should have a clear reason for existing. Remove anything that doesn't serve the user's goals.

2. **Clarity Over Cleverness**: Simple, straightforward solutions beat clever but confusing ones.

3. **Consistency**: Use consistent patterns for similar interactions. Users build mental models faster when patterns repeat.

4. **Hierarchy**: Guide users through clear visual hierarchy. Size, color, spacing, and position all communicate importance.

5. **Feedback**: Every action should provide appropriate feedback. Users need to know their interactions registered.

6. **Efficiency**: Design for the expert while accommodating the novice. Shortcuts and power features should be discoverable but not overwhelming.

### Typography

Typography is 80% of web design. Good typography makes interfaces readable, establishes hierarchy, and creates mood.

**Font Selection:**
- Use no more than 2-3 font families per project
- Pair contrasting typefaces (serif + sans-serif for contrast, or different weights of the same family)
- Ensure fonts are readable at all sizes they'll be used
- Consider loading performance (system fonts are instant, web fonts add weight)

**Sizing & Hierarchy:**
```
Heading 1: 2.5-3rem (40-48px) - Page titles
Heading 2: 2-2.5rem (32-40px) - Section headers  
Heading 3: 1.5-1.75rem (24-28px) - Subsections
Heading 4: 1.25-1.5rem (20-24px) - Card titles
Body: 1rem (16px) - Default text
Small: 0.875rem (14px) - Secondary text, captions
Tiny: 0.75rem (12px) - Labels, metadata
```

**Line Height:**
- Headings: 1.2-1.3 (tight, impactful)
- Body text: 1.5-1.7 (readable, comfortable)
- Small text: 1.4-1.5 (prevents dense blocks)

**Color & Contrast:**
- Body text should have at least 4.5:1 contrast ratio
- Large text (18px+) should have at least 3:1 contrast ratio
- Use color to reinforce hierarchy, not create it

### Avoiding "AI Slop"

When generating designs with AI, watch for these common tells:

**Over-designing:**
- Too many different font sizes (creates visual chaos)
- Excessive use of gradients and shadows
- Unnecessary decorative elements
- Complex layouts where simple ones work better

**Inconsistency:**
- Mismatched border radii on similar elements
- Inconsistent spacing (8px here, 13px there)
- Different button styles for the same hierarchy level
- Varying shadow intensities without purpose

**Missing Context:**
- Generic placeholder content that doesn't reflect real use
- Perfect symmetry where asymmetry would add interest
- Lorem ipsum in user-facing mocks
- No consideration of edge cases (long names, empty states, errors)

**Performance Blindness:**
- Heavy animations without considering mobile
- Multiple web fonts when system fonts suffice
- Complex SVGs where simple ones work
- Images without optimization

### Design Thinking Questions

Before implementing any design, ask:
1. What is the user trying to accomplish?
2. What is the simplest path to that goal?
3. How can we reduce cognitive load?
4. What could confuse a first-time user?
5. How does this scale to different screen sizes?
6. What happens when content is longer/shorter than expected?
7. Is the design accessible to users with disabilities?
8. Does this design work for colorblind users?
9. How does this perform on slow connections?
10. What would make this delightful rather than just functional?
