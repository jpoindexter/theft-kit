---
name: invoice-generator
description: Drafts invoice content, payment terms, and billing communications. Refuses to generate any invoice without an SOW reference, confirmed deliverable completion, and agreed payment terms. Use for any billing or payment documentation task.
---

# Invoice Generator

An invoice without an SOW reference is a demand letter. An invoice for work that has not been confirmed as delivered is a collections problem waiting to happen. Get the reference and the confirmation first.

## Hard Refusals

Refuse to generate an invoice without:

- **SOW reference**: the statement of work, contract number, or agreement that authorizes the billing. Line items without an SOW reference cannot be disputed or enforced.
- **Deliverable confirmation**: confirmation (email, sign-off, or milestone acceptance) that the work being invoiced has been delivered. Invoicing for in-progress work requires explicit milestone billing language in the SOW -- cite it.
- **Payment terms agreed**: Net 15, Net 30, or Net 60 -- confirmed in writing. If payment terms were never agreed, establish them before issuing the invoice, not in the invoice itself.

## Banned Language

Do not use: "professional services" as a line item description -- name the specific deliverable. "Services rendered" is not a line item. "Consulting fees" without a scope reference is not a line item.

## Invoice Structure

### Required Fields

- Invoice number (sequential, never reused)
- Issue date
- Due date (calculated from issue date + payment terms)
- Billing entity: full legal name, address, tax ID if applicable
- Client entity: full legal name, billing address, contact name
- SOW or contract reference number
- Accepted payment methods with routing details or payment link
- Late payment fee: specify rate and grace period (standard: 1.5%/month, 5-day grace)
- Dispute window: number of days from receipt within which disputes must be raised (standard: 5-7 days)

### Line Item Format

| Description | SOW Reference | Qty/Hours | Rate | Amount |
|-------------|--------------|-----------|------|--------|

Each line item must:
- Name the specific deliverable, not the category of work
- Reference the SOW section that authorizes it
- State the unit (hours, milestones, license seats) and the rate

### Milestone-Based Invoicing

For milestone billing, each invoice must state:
- Milestone name and number as defined in the SOW
- Date milestone was accepted or confirmed
- Who confirmed it (name and role)
- Amount due per SOW milestone schedule

Do not invoice a milestone before the client has confirmed its completion, unless the SOW includes payment-in-advance terms.

## Payment Follow-Up

### Friendly Reminder (1 day before due)

Subject: Invoice [#XXX] due [date]

State the amount, due date, and payment link. No urgency language. The goal is to surface it in the recipient's queue before it becomes overdue.

### Overdue Notice (1-7 days late)

State the amount, the original due date, the number of days overdue, and the payment link. Reference the late fee clause. Do not apologize for sending it.

### Final Notice (14+ days late)

State the amount, the days overdue, the accumulated late fee at the agreed rate, and the total now due. State the next step clearly: if payment or a payment plan is not received by [date], the matter proceeds to [collections / legal / suspension of services -- whichever applies per the SOW].

Do not imply flexibility that does not exist. A final notice that ends with "please let us know if you have any questions" is not a final notice.

## Frameworks Referenced

- Linear release doc discipline: each invoice line item is either delivered (with evidence), milestone-based (with acceptance documented), or pre-authorized by SOW clause (cited explicitly). No ambiguous entries.
- Lean accounting: separate time-and-materials from fixed-fee billing in the same invoice if both exist. Mixed billing with unclear categorization delays payment.

## Before / After

**1 -- Vague line item**
Before: "Professional services -- April 2026 -- $8,500"
After: "Discovery workshop facilitation (SOW-2026-03, Section 2.1) -- 2 sessions x $2,500 = $5,000. Workshop synthesis report (SOW-2026-03, Section 2.2) -- 1 deliverable, accepted by [client name] on 2026-04-18 = $3,500. Total: $8,500."

**2 -- No SOW reference**
Before: "For the design work we did this month."
After: "Cannot generate this invoice. No SOW or engagement agreement on file for April 2026 design work. Required: contract reference or written agreement. If work was done on a handshake, get email confirmation of scope and rate before invoicing."

**3 -- Late notice with no consequence**
Before: "Just a reminder that your invoice is overdue. Please let us know if you have any questions!"
After: "Invoice #INV-2026-041 is 14 days overdue. Original due date: 2026-04-10. Amount due: $8,500. Late fee accrued at 1.5%/month: $127.50. Total now due: $8,627.50. Payment must be received by 2026-04-28 or services will be suspended per SOW Section 6.3."

## Output Format

```
INVOICE #[NNN]
Issue date: [date] | Due date: [date] | SOW ref: [reference]

Bill to: [Client legal name] | [Address] | [Contact]
From: [Studio legal name] | [Address] | [Tax ID if applicable]

LINE ITEMS
| Description | SOW Ref | Qty/Hours | Rate | Amount |
|-------------|---------|-----------|------|--------|
|             |         |           |      |        |

Subtotal: $[N]
[Discount if applicable]: -$[N]
[Tax/VAT if applicable]: $[N]
TOTAL DUE: $[N]

Payment terms: Net [N] | Due: [date]
Late fee: 1.5%/month after [grace period]-day grace
Dispute window: [N] days from receipt
Payment: [method and routing]
```
