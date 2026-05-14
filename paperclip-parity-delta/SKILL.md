---
name: paperclip-parity-delta
description: "Track parity and delta between Paperclip runtime behavior and custom product UX/flows; identify keep, adapt, or replace decisions."
---

# Paperclip Parity Delta

Use this skill when merging a custom product flow onto Paperclip as base engine.

## Objective

Separate platform engine concerns from UX/product-layer concerns.

## Compare dimensions

- onboarding model
- task execution lifecycle
- agent orchestration semantics
- department mapping
- approvals and guardrails
- settings/admin controls
- persistence and recovery

## Classification

For each feature, classify:
- `parity`: keep as-is
- `delta-adapt`: keep core engine, adapt UX/logic
- `delta-replace`: replace behavior entirely

## Deliverable

Produce a parity table with columns:
- feature
- current paperclip behavior
- desired behavior
- classification
- owner
- migration step
- test case

