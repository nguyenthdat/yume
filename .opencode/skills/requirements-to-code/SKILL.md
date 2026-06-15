---
name: requirements-to-code
description: Turn vague user requests into implementable coding briefs, deep-reason through ambiguity, decide whether to ask one blocking question, decompose work, assign Task prompts to coding subagents, and verify final changes. Use for implement, code, build, fix, refactor, scaffold, feature request, bug report, failing test, Android/Rust implementation, rerun, re-execute, update, modify, supplement, redo only one part, or based on previous results. Do not use for pure discussion with no implementation intent.
---

# Requirements To Code Skill

Use this skill when a request needs to become verified code instead of a loose plan.

## Requirement Intake

- Restate the user goal in one sentence.
- Identify the affected area: product behavior, code module, tests, build, architecture boundary, or developer workflow.
- Extract constraints: platform, language, package names, performance, privacy, compatibility, and verification.
- Identify non-goals from the user's wording and existing code.
- Ask one concise question only if implementation would be unsafe or directionless without it.

## Deep Reasoning Before Delegation

- Decide whether the request is single-domain or cross-domain.
- Prefer direct subagent delegation for narrow work and an orchestrated flow for cross-domain work.
- Give each subagent a precise Task prompt with context, files to inspect, acceptance criteria, allowed edit scope, and expected verification.
- For parallel Android and Rust work, define the shared boundary first: data types, error cases, generated bindings, package names, and build artifacts.
- Keep `_workspace/requirements-coding/` artifacts for large briefs, subagent outputs, and QA results.

## Implementation Brief Template

```markdown
# Requirement Brief

Goal:

Assumptions:

Non-goals:

Affected areas/files:

Subagent assignments:

Acceptance criteria:

Verification commands:

Risks:
```

## Delegation Rules

- Use `senior-coding-engineer` for general implementation, glue code, repo-wide edits, tests, and refactors.
- Use `android-senior-engineer` for Android/Kotlin/Gradle/Compose/CameraX/lifecycle/permissions/native packaging.
- Use `rust-senior-engineer` for Rust/Cargo/crates/error models/tests/performance/FFI-safe implementation.
- Use `architecture-qa-reviewer` after implementation or when boundaries changed.

## Verification

- Run the most local fast check first, then broader tests if relevant.
- If no test harness exists, use compile/type checks or add focused tests when practical.
- Report exact commands and outcomes.
- Separate failures caused by new changes from pre-existing or environment failures.

## Trigger Tests

Should trigger:

- "Implement the OCR import flow."
- "Fix the Rust build and update tests."
- "Break down this request and send the coding task to a subagent."
- "Add an Android screen for chat history."
- "Refactor the retrieval code and verify it."
- "Redo only the Rust part based on previous results."
- "Update this feature after QA feedback."
- "Build the code scaffold for this architecture."

Should not trigger:

- "Explain what Rust is."
- "Brainstorm product names."
- "Review this PR only for risks, no edits."
- "Design high-level architecture with no coding intent."
- "Summarize an article."
- "Find a skill for PDF processing."
- "Write marketing copy."
- "Compare two phones."
