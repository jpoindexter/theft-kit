# Agentic Identity and Trust Patterns

Reference for multi-agent system design. Use when designing agent-to-agent delegation, trust scoring, or audit trail requirements.

Sources: NIST SP 800-207 (Zero Trust Architecture, 2020); NIST PQC standards (finalized 2024); derived from zero-trust security literature. Not derived from Medium articles or tutorial blog posts.

---

## TOC

1. Zero-trust principles for agents
2. Penalty-based trust scoring
3. Delegation chain verification
4. Tamper-evident evidence records
5. Peer verification protocol
6. Threat model questions
7. Crypto hygiene

---

## 1. Zero-trust principles for agents

Four non-negotiable properties. Violating any of these produces a system that looks secure but is not.

1. Never trust self-reported identity -- require cryptographic proof
2. Never trust self-reported authorization -- require a verifiable delegation chain
3. Never trust mutable logs -- append-only or worthless for audit
4. Assume compromise -- design assuming at least one agent in the system is compromised at any given time

These are not aspirational. They are the minimum viable security posture for any system where agents act on behalf of other agents.

---

## 2. Penalty-based trust scoring

Start at 1.0. Only verifiable evidence degrades the score. Self-reported signals do not affect it.

| Event | Score change |
|-------|-------------|
| Evidence chain integrity breach | -0.5 |
| Outcome verification failure | -(failure_rate x 0.4) |
| Credential staleness (over 90 days) | -0.1 |

| Score range | Trust level |
|-------------|------------|
| 0.9 and above | HIGH |
| 0.5 to 0.89 | MODERATE |
| 0.01 to 0.49 | LOW |
| 0 | NONE |

Agents at NONE do not interact with the system. Agents at LOW require explicit human escalation before acting. Trust scores are not reversible by agent action -- only by a verified remediation event signed by a human operator.

---

## 3. Delegation chain verification

Each link in the chain must be:
- Signed by the delegator using the delegator's current key
- Scoped to specific actions (scope cannot escalate beyond the parent grant)
- Temporally valid (expired credentials are invalid with no grace period)

A single broken link invalidates the entire chain. Fail closed, not open. An agent that cannot verify its delegation chain does not act.

---

## 4. Tamper-evident evidence records

Append-only log. Each record links to the previous via SHA-256 hash over canonical JSON. Tampering with any record invalidates all subsequent records.

Required fields per record:
```
agent_id
action_type
intent
decision
outcome
timestamp (UTC, ISO 8601)
prev_record_hash
agent_signature
```

Canonical JSON: sort keys alphabetically, no extra whitespace, consistent encoding. Non-canonical serialization breaks deterministic hashing and silently corrupts the chain.

Key material must never appear in evidence records, logs, or API responses.

---

## 5. Peer verification protocol

All five checks must pass. Failure on any single check rejects the interaction.

1. Cryptographic identity verification
2. Credential expiry check
3. Scope covers the requested action
4. Trust score at or above 0.5
5. Delegation chain valid (required if the request is delegated)

---

## 6. Threat model questions

Answer these before designing any multi-agent trust system. The answers change the architecture.

1. How many agents interact? Two agents and 200 agents have fundamentally different threat surfaces.
2. Do agents delegate to each other? If yes, delegation chain verification is required from the start.
3. What is the blast radius of a forged identity? Financial loss? Code execution? Physical system access? The answer sets the minimum security bar.
4. Who is the relying party for each action?
5. What is the key compromise recovery path? If a key leaks, how do you revoke and reissue without disrupting the system?
6. What compliance regime applies? SOC 2, HIPAA, and PCI DSS each add specific constraints that change the architecture.

---

## 7. Crypto hygiene

- Use established standards. No custom cryptographic implementations. Custom crypto is nearly always wrong.
- Separate keys for signing, encryption, and identity. Never reuse keys across purposes.
- Plan for post-quantum migration now. Algorithm agility (ability to swap algorithms without redesigning the system) is cheaper to build in than to retrofit. See NIST PQC finalized standards (2024): ML-KEM, ML-DSA, SLH-DSA.
- Key material never appears in logs, evidence records, or API responses. Treat as you would a private key in source control -- a leak requires full rotation, not a patch.
