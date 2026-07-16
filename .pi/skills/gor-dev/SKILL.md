---
name: gor-dev
description: "Use for implementing gor feature stories. When the user says 'work on story X', 'implement X', 'next story', 'work on the next story', or references a story from docs/issues/, use this skill. If no specific story is named, a cheap scout agent determines the next ready story from the backlog. Coordinates cheap worker subagents for rote implementation while the wrangler (you) plans, validates, integrates, and commits. Never implement a story directly — always delegate the bulk coding to workers."
---

# gor-dev — Story Implementation Workflow

Coordinate cheap worker subagents to implement `gor` feature stories. You (the wrangler) plan, validate, integrate, and commit. Workers do the rote coding.

## Model Assignment

| Role | Model | Purpose |
|------|-------|---------|
| **Wrangler** | Your current model | Plan, review, validate, commit |
| **Worker** | `ollama-cloud/deepseek-v4-flash` | Write code for one small, isolated task |

Use `ollama-cloud/deepseek-v4-flash` for all worker subagents unless the user overrides.

## Workflow (per story)

### Step 0: Pre-flight CI Check

**Before starting any story work**, check that CI is green on `main`. This ensures any future failures are caused by the story work, not pre-existing issues.

1. Check the latest CI run on `main`:
   ```bash
   gh run list --limit 1 --branch main --workflow ci.yml
   ```

2. If the latest run is **not** `completed` with `success`, investigate and fix:
   ```bash
   gh run view <run-id> --log-failed
   ```

3. Common CI failures and fixes:
   - **`dtolnay/rust-toolchain` fails with `'toolchain' is a required input`**: Add `toolchain: stable` to the step's `with:` block in `.github/workflows/ci.yml` or `.github/workflows/release.yml`.
   - **Action can't be resolved (invalid commit hash)**: Look up the correct commit hash via `gh api repos/<owner>/<repo>/git/ref/tags/<tag> --jq '.object.sha'` and update the pinned hash.
   - **Clippy / fmt / test failures**: These are code issues — fix the source, not the workflow.

4. Fix any failures, commit with `ci:` scope, push, and wait for the run to go green before proceeding.

5. Also check the Release workflow:
   ```bash
   gh run list --limit 1 --branch main --workflow release.yml
   ```

### Step 1: Find Next Story (if no story specified)

When the user says "next story" or "work on the next story" without naming a specific story, delegate to a cheap scout agent to determine what to work on.

Launch a **scout** subagent with:
- `context: "fork"`
- `model: "ollama-cloud/deepseek-v4-flash"`
- `reads: ["docs/issues/"]`

**Scout prompt:**
```
Analyze the story backlog in docs/issues/ to find the next story to implement.

For each story, extract from its YAML frontmatter:
- status (todo, in_progress, done)
- priority (P0, P1, P2, P3, P4)
- phase (0-4)
- blockedBy (list of story names that must be done first)

Rules:
1. Only consider stories with status: todo
2. A story is ready if ALL stories in its blockedBy list have status: done
3. Among ready stories, pick the highest priority (P0 > P1 > P2 > P3 > P4)
4. If multiple at the same priority, pick the lowest phase first
5. If still tied, pick the one that blocks the most other stories

Output ONLY this JSON (no other text):
{"story": "<filename without .md>", "priority": "<P0-P4>", "phase": <N>, "blocks": <count>, "reason": "<one-line explanation>"}
```

Parse the scout's JSON output. That's the story to implement. Proceed to Step 2.

### Step 2: Read & Plan

Read the story file from `docs/issues/<story>.md`. Break it into 2-5 small, independent tasks. Each task should touch at most 2-3 files and have zero dependencies on other tasks.

Create a todo list for the story tasks.

### Step 3: Delegate (Parallel)

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

### Step 4: Validate

After all workers finish, run the validation gate:
```bash
cargo fmt
cargo clippy --all-targets
cargo test
```

If anything fails, fix it yourself (the wrangler). Don't send workers back to fix their own work — it's faster to fix small issues directly.

### Step 5: Integrate & Commit

Once the gate is green:
1. Update the story file status to `done`
2. Add implementation notes if the design diverged from the spec
3. Commit with a conventional commit message: `feat(<scope>): <description>`
4. Groom any backlog stories that need updates based on what was learned

### Step 6: Push & Watch CI

After committing, push and monitor CI for failures caused by the story work:

1. Push to `main`:
   ```bash
   git push
   ```

2. Wait for the CI run to start, then monitor it:
   ```bash
   gh run list --limit 1 --branch main --workflow ci.yml
   ```

3. Poll the run status until all jobs complete. Use:
   ```bash
   gh run view <run-id> --json status,conclusion,jobs --jq '{status, conclusion, jobs: [.jobs[] | {name, status, conclusion}]}'
   ```

4. If any job fails:
   - Inspect the failure: `gh run view <run-id> --log-failed --job <job-id>`
   - Fix the issue (code or workflow)
   - Commit with appropriate scope (`fix:` for code, `ci:` for workflow)
   - Push and re-check

5. Only report the story as complete once CI is fully green.

### Step 7: Report

Summarize what was done: files created/modified, test count, any design decisions, and CI status.

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

CI-only changes use `ci:` scope.

## Anti-Patterns

- **Don't** implement code yourself — delegate to workers
- **Don't** send workers back to fix their mistakes — fix small issues yourself
- **Don't** skip the validation gate
- **Don't** skip the pre-flight CI check — always verify CI is green before starting
- **Don't** commit until all tests pass and clippy is clean
- **Don't** consider a story done until CI is green on the pushed commit
- **Don't** delegate architecture decisions — those are the wrangler's job
- **Don't** run more than 4 workers in parallel (keep it manageable)
