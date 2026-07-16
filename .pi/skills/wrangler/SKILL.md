---
name: wrangler
description: "Use for implementing gor feature stories. When the user says 'work on story X', 'implement X', 'next story', or references a story from docs/issues/, use this skill. Coordinates cheap worker subagents for rote implementation while the wrangler (you) plans, validates, integrates, and commits. Never implement a story directly — always delegate the bulk coding to workers."
---

# Wrangler — Story Implementation Workflow

Coordinate cheap worker subagents to implement `gor` feature stories. You (the wrangler) plan, validate, integrate, and commit. Workers do the rote coding.

## Model Assignment

| Role | Model | Purpose |
|------|-------|---------|
| **Wrangler** | Your current model | Plan, review, validate, commit |
| **Worker** | `ollama-cloud/deepseek-v4-flash` | Write code for one small, isolated task |

Use `ollama-cloud/deepseek-v4-flash` for all worker subagents unless the user overrides.

## Workflow (per story)

### Step 1: Read & Plan

Read the story file from `docs/issues/<story>.md`. Break it into 2-5 small, independent tasks. Each task should touch at most 2-3 files and have zero dependencies on other tasks.

Create a todo list for the story tasks.

### Step 2: Delegate (Parallel)

For each task, launch a worker subagent with:
- `context: "fork"` — clean context, no pollution from other tasks
- `model: "ollama-cloud/deepseek-v4-flash"`
- A detailed prompt specifying exact file paths, function signatures, and expected behavior
- `reads: ["src/"]` so the worker can see existing code

**Prompt template for workers:**
```
You are implementing part of the <story-name> story for the gor CLI (GitHub on Rust).

Task: <one-sentence description>

Files to create/modify:
- path/to/file.rs: <what to do>

Context:
<relevant code snippets or design decisions>

Requirements:
- Follow the project conventions in AGENTS.md
- Use anyhow::Result in command code, GorError in library code
- Add doc comments with # Examples on public items
- Add unit tests in #[cfg(test)] mod tests
- Do NOT run cargo test or cargo clippy — just write the code
- Do NOT commit

Output: Write the file(s) and report what you created.
```

### Step 3: Validate

After all workers finish, run the validation gate:
```bash
cargo fmt
cargo clippy --all-targets
cargo test
```

If anything fails, fix it yourself (the wrangler). Don't send workers back to fix their own work — it's faster to fix small issues directly.

### Step 4: Integrate & Commit

Once the gate is green:
1. Update the story file status to `done`
2. Add implementation notes if the design diverged from the spec
3. Commit with a conventional commit message: `feat(<scope>): <description>`
4. Groom any backlog stories that need updates based on what was learned

### Step 5: Report

Summarize what was done: files created/modified, test count, any design decisions.

## Validation Gate (non-negotiable)

Every story MUST pass before committing:
```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
```

## Commit Convention

```
feat(<scope>): <imperative description>

- bullet points for notable changes
```

Scopes: `auth`, `repo`, `pr`, `issue`, `release`, `label`, `search`, `gist`, `workflow`, `run`, `client`, `output`, `config`

## Anti-Patterns

- **Don't** implement code yourself — delegate to workers
- **Don't** send workers back to fix their mistakes — fix small issues yourself
- **Don't** skip the validation gate
- **Don't** commit until all tests pass and clippy is clean
- **Don't** delegate architecture decisions — those are the wrangler's job
- **Don't** run more than 4 workers in parallel (keep it manageable)
