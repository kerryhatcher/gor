---
name: adrs
description: "Use for creating and managing Architecture Decision Records (ADRs) with the adrs CLI tool. Trigger when the user mentions ADRs, architecture decisions, design decisions, or when a major architectural choice is made during implementation that should be recorded. Also use for searching, updating, superseding, linking, or validating existing ADRs."
---

# adrs — Architecture Decision Records

Manage ADRs in this repo using the `adrs` CLI tool. ADRs live in `docs/adr/` and use NextGen mode (YAML frontmatter) with MADR format.

## When to Record an ADR

Create an ADR anytime a **major architectural decision** is made. Examples:

- Choosing between two libraries or frameworks (e.g., `gix` vs `git2`)
- Picking a data format, protocol, or API strategy (e.g., REST-first over GraphQL)
- Deciding on a concurrency model (sync vs async)
- Establishing a cross-cutting pattern (error handling strategy, config format)
- Deprecating or replacing an existing approach

**Do NOT** create ADRs for routine implementation details, bug fixes, or decisions already covered by AGENTS.md conventions.

## Repo Configuration

- **Directory:** `docs/adr/`
- **Mode:** NextGen (YAML frontmatter with tags, deciders, custom fields)
- **Format:** MADR 4.0.0 (structured sections: Context, Decision Drivers, Considered Options, Decision Outcome, Pros and Cons)
- **Config file:** `adrs.toml` at repo root

## Common Commands

### Creating an ADR

```bash
# Create a new ADR (opens editor)
adrs new "Use gix over git2 for git operations"

# Create without opening editor (for scripting/CI)
adrs new "Use gix over git2 for git operations" --no-edit

# Create with tags (NextGen mode is on by default via adrs.toml)
adrs new -t git,dependencies "Use gix over git2 for git operations"

# Create and immediately mark as accepted
adrs new --status accepted "Use gix over git2 for git operations"

# Create and supersede an existing ADR
adrs new --supersedes 3 "Use reqwest with rustls instead of openssl"
```

### Viewing and Searching

```bash
# List all ADRs
adrs list

# Detailed list with status and date
adrs list -l

# Filter by status
adrs list --status accepted

# Filter by tag
adrs list --tag security

# Search all content
adrs search "rustls"

# Search titles only
adrs search -t "error handling"
```

### Editing and Updating

```bash
# Edit an ADR by number
adrs edit 3

# Edit by fuzzy title match
adrs edit "gix over git2"

# Change status
adrs status 3 accepted
adrs status 2 superseded --by 5
adrs status 4 deprecated
```

### Linking ADRs

```bash
# Link two ADRs (reverse link auto-derived)
adrs link 3 Supersedes 1
adrs link 5 Amends 2
adrs link 4 "Relates to" 3
```

### Validation and Health

```bash
# Check ADR repository health
adrs doctor

# Show current configuration
adrs config
```

### Generating Documentation

```bash
# Generate table of contents
adrs generate toc > docs/adr/README.md

# Generate Graphviz graph of ADR relationships
adrs generate graph

# Generate an mdbook
adrs generate book
```

## MADR Template Sections

When creating a MADR-format ADR, fill out these sections:

1. **Context and Problem Statement** — What problem are we solving? Why now?
2. **Decision Drivers** — What forces are shaping this decision? (e.g., performance, security, maintainability)
3. **Considered Options** — List all options evaluated (including "do nothing")
4. **Decision Outcome** — Which option was chosen and why
5. **Consequences** — What becomes easier/harder because of this?
6. **Pros and Cons of the Options** — Detailed comparison of each option

## Status Lifecycle

```
proposed → accepted → deprecated → superseded
                ↓
            rejected
```

- **proposed** — Default for new ADRs; under discussion
- **accepted** — Approved and in effect
- **deprecated** — No longer recommended, but not replaced
- **superseded** — Replaced by a newer ADR (use `--by` to link)
- **rejected** — Decision was not approved

## Workflow for Agents

1. When a major architectural decision is made during implementation, **pause and create an ADR** before committing the code.
2. Use `--no-edit` to create the ADR programmatically, then fill in the content directly in the markdown file.
3. After writing the ADR content, run `adrs doctor` to validate.
4. Commit the ADR alongside the code that implements the decision.
5. If a decision changes, use `adrs status` to mark the old ADR as superseded and create a new one with `--supersedes`.

## Anti-Patterns

- **Don't** create ADRs for trivial choices already covered by AGENTS.md
- **Don't** leave ADRs in `proposed` status after the decision is implemented
- **Don't** edit an accepted ADR's decision — supersede it instead
- **Don't** forget to link related ADRs
- **Don't** skip the MADR sections — the structured format exists for a reason
