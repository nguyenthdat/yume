---
name: senior-implementation-engineering
description: Senior implementation guidance for coding subagents working on general code, native Android, Kotlin, Gradle, Jetpack Compose, CameraX, Rust, Cargo, UniFFI-adjacent APIs, tests, build fixes, refactors, performance, and verification. Use when code should be written or reviewed by a senior coding, Android, or Rust engineer. Do not use for pure architecture-only planning unless implementation tradeoffs are being resolved.
---

# Senior Implementation Engineering Skill

Use this skill to keep coding work small, correct, verified, and aligned with existing project structure.

## Universal Coding Rules

- Read current code before editing; local conventions beat generic preferences.
- Implement the smallest complete change that satisfies the brief.
- Keep public contracts stable unless the brief requires a breaking change.
- Prefer straightforward code over abstractions that have only one caller.
- Add tests for changed behavior when the repository has a practical test surface.
- Report verification honestly, including commands not run and why.

## Android Senior Engineering

- Check the existing Gradle layout, namespace, min SDK, Compose usage, and dependency style before adding files.
- Keep long-running OCR, indexing, model, and Rust calls off the main thread.
- Treat permissions, lifecycle, process death, and configuration changes as core behavior.
- Keep generated native bindings separate from handwritten facades.
- Verify native library packaging by ABI and build variant when Rust or UniFFI is involved.

## Rust Senior Engineering

- Use typed errors for recoverable failures and avoid panics in library code.
- Keep FFI-exported models simple: owned strings, numbers, lists, records, enums, and explicit errors.
- Put ergonomic Rust internals behind stable exported boundaries.
- Add deterministic unit tests for text, parsing, chunking, retrieval ranking, and error cases.
- Verify `cargo test`, `cargo check`, target-specific builds, or the closest available command.

## Cross-Boundary Checks

- Compare producer and consumer contracts before claiming completion.
- Android calling Rust requires matching package names, generated bindings, native library names, error mapping, and threading policy.
- OCR feeding RAG requires text, confidence, page/source metadata, correction status, and stable document ids.
- Chat tools require typed input/output, permission policy, and non-destructive defaults.

## Output Expectations

- State changed files and why.
- State tests or checks run.
- State assumptions and any blocked verification.
- Flag boundary risks for QA.

## Trigger Tests

Should trigger:

- "Senior Android engineer: fix this Gradle issue."
- "Rust senior engineer: implement chunk ranking tests."
- "Write the Kotlin facade for generated UniFFI bindings."
- "Refactor this implementation with verification."
- "Implement the code from this requirement brief."
- "Fix the failing cargo test."
- "Update only the Android native library packaging."
- "Review Android/Rust boundary before code changes."

Should not trigger:

- "Create a product roadmap."
- "Explain Android at a beginner level."
- "Design a logo."
- "Summarize generic RAG concepts."
- "Draft a non-technical email."
- "Audit harness registry only."
- "Search the web for phone prices."
- "Plan architecture with no implementation constraints."
