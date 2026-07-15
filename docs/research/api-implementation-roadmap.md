# API Implementation Roadmap

Research compiled 2026-07 from official GitHub Enterprise Server 3.20 REST API documentation.
Maps the ~1,005 REST endpoints to gor's planned command surface, prioritized by user value.

---

## Phase 0: Foundation (proven starting point)

These commands are the baseline — already proven and should be the starting point for gor:

| Command | Endpoints Used | Status |
|---------|---------------|--------|
| `auth login/logout/status/token` | OAuth device flow, `GET /user` | ✅ Baseline |
| `api` (arbitrary REST) | Any endpoint | ✅ Baseline |
| `repo view/list/clone` | `GET /repos/{owner}/{repo}`, `GET /user/repos`, `GET /users/{username}/repos`, `GET /orgs/{org}/repos` | ✅ Baseline |
| `pr list/view/checkout/create/comment/close/reopen/merge` | `GET /repos/{o}/{r}/pulls`, `GET /repos/{o}/{r}/pulls/{n}`, `POST /repos/{o}/{r}/pulls`, `POST /repos/{o}/{r}/issues/{n}/comments`, `PATCH /repos/{o}/{r}/pulls/{n}`, `PUT /repos/{o}/{r}/pulls/{n}/merge` | ✅ Baseline |
| `issue list/view/create/comment/close/reopen` | `GET /repos/{o}/{r}/issues`, `GET /repos/{o}/{r}/issues/{n}`, `POST /repos/{o}/{r}/issues`, `POST /repos/{o}/{r}/issues/{n}/comments`, `PATCH /repos/{o}/{r}/issues/{n}` | ✅ Baseline |

---

## Phase 1: Complete the core (parity with `gh` daily-use surface)

### `repo` — lifecycle operations
| Subcommand | Endpoints | Notes |
|-----------|-----------|-------|
| `repo create` | `POST /user/repos`, `POST /orgs/{org}/repos` | Name, description, private/public, template |
| `repo fork` | `POST /repos/{o}/{r}/forks` | |
| `repo delete` | `DELETE /repos/{o}/{r}` | Needs confirmation prompt |
| `repo edit` | `PATCH /repos/{o}/{r}` | Description, visibility, topics, etc. |
| `repo archive` | `PATCH /repos/{o}/{r}` (archived=true) | |
| `repo rename` | `PATCH /repos/{o}/{r}` (name field) | |
| `repo sync` | `POST /repos/{o}/{r}/merge-upstream` | Sync fork from upstream |

### `release` — full command group (top missing feature)
| Subcommand | Endpoints | Notes |
|-----------|-----------|-------|
| `release list` | `GET /repos/{o}/{r}/releases` | |
| `release view` | `GET /repos/{o}/{r}/releases/{id}` | By tag or ID |
| `release create` | `POST /repos/{o}/{r}/releases` | Tag, name, body, draft, prerelease, assets |
| `release edit` | `PATCH /repos/{o}/{r}/releases/{id}` | |
| `release delete` | `DELETE /repos/{o}/{r}/releases/{id}` | |
| `release upload` | `POST /repos/{o}/{r}/releases/{id}/assets` | Upload binary assets |
| `release download` | `GET /repos/{o}/{r}/releases/assets/{id}` | Download assets |

### `pr` — complete the surface
| Subcommand | Endpoints | Notes |
|-----------|-----------|-------|
| `pr diff` | `GET /repos/{o}/{r}/pulls/{n}` (diff media type) | `Accept: application/vnd.github.diff` |
| `pr review` | `POST /repos/{o}/{r}/pulls/{n}/reviews` | Approve, comment, request changes |
| `pr ready` | `PATCH /repos/{o}/{r}/pulls/{n}` (draft=false) | |
| `pr edit` | `PATCH /repos/{o}/{r}/pulls/{n}` | Title, body, base, state |
| `pr checks` | `GET /repos/{o}/{r}/commits/{ref}/check-runs` | CI status |

### `issue` — complete the surface
| Subcommand | Endpoints | Notes |
|-----------|-----------|-------|
| `issue edit` | `PATCH /repos/{o}/{r}/issues/{n}` | Title, body, state, milestone, labels, assignees |
| `issue delete` | `DELETE /repos/{o}/{r}/issues/{n}` | Needs confirmation |
| `issue lock/unlock` | `PUT/DELETE /repos/{o}/{r}/issues/{n}/lock` | |
| `issue pin/unpin` | `POST/DELETE /repos/{o}/{r}/issues/{n}/pin` | |
| `issue transfer` | `POST /repos/{o}/{r}/issues/{n}/transfer` | |

### `label` — new command group
| Subcommand | Endpoints | Notes |
|-----------|-----------|-------|
| `label list` | `GET /repos/{o}/{r}/labels` | |
| `label create` | `POST /repos/{o}/{r}/labels` | Name, color, description |
| `label edit` | `PATCH /repos/{o}/{r}/labels/{name}` | |
| `label delete` | `DELETE /repos/{o}/{r}/labels/{name}` | |
| `label clone` | GET + POST | Copy labels between repos |

### Cross-cutting improvements
- `--json <fields>` with field selection (not just all-or-nothing)
- `--jq` / `--template` output formatting
- `--web` flag on `pr view`, `issue view`, `repo view`
- `--assignee`, `--label`, `--milestone`, `--project` on `pr create` / `issue create`
- Auto-detect head branch for `pr create` (currently requires `--head`)

---

## Phase 2: Search, gists, and CI visibility

### `search` — new command group
| Subcommand | Endpoints | Notes |
|-----------|-----------|-------|
| `search repos` | `GET /search/repositories` | |
| `search issues` | `GET /search/issues` | Includes PRs |
| `search prs` | `GET /search/issues` (type:pr) | |
| `search code` | `GET /search/code` | |
| `search commits` | `GET /search/commits` | |

### `gist` — new command group
| Subcommand | Endpoints | Notes |
|-----------|-----------|-------|
| `gist list` | `GET /gists`, `GET /users/{u}/gists` | |
| `gist view` | `GET /gists/{id}` | |
| `gist create` | `POST /gists` | Files, description, public/secret |
| `gist edit` | `PATCH /gists/{id}` | |
| `gist delete` | `DELETE /gists/{id}` | |
| `gist clone` | Git clone via gix | |

### `workflow` + `run` — new command groups
| Subcommand | Endpoints | Notes |
|-----------|-----------|-------|
| `workflow list` | `GET /repos/{o}/{r}/actions/workflows` | |
| `workflow view` | `GET /repos/{o}/{r}/actions/workflows/{id}` | |
| `workflow run` | `POST /repos/{o}/{r}/actions/workflows/{id}/dispatches` | |
| `workflow enable/disable` | `PUT /repos/{o}/{r}/actions/workflows/{id}/enable` | |
| `run list` | `GET /repos/{o}/{r}/actions/runs` | |
| `run view` | `GET /repos/{o}/{r}/actions/runs/{id}` | |
| `run watch` | Poll `GET /repos/{o}/{r}/actions/runs/{id}` | |
| `run cancel` | `POST /repos/{o}/{r}/actions/runs/{id}/cancel` | |
| `run rerun` | `POST /repos/{o}/{r}/actions/runs/{id}/rerun` | |
| `run download` | `GET /repos/{o}/{r}/actions/runs/{id}/artifacts` + download | |

### `browse` — new command
| Subcommand | Endpoints | Notes |
|-----------|-----------|-------|
| `browse` | No API — opens `html_url` in `$BROWSER` | Works for repo, issue, PR |

---

## Phase 3: Organization, user, and project management

### `org` — new command group
| Subcommand | Endpoints | Notes |
|-----------|-----------|-------|
| `org list` | `GET /user/orgs` | |
| `org view` | `GET /orgs/{org}` | |

### `project` — new command group
| Subcommand | Endpoints | Notes |
|-----------|-----------|-------|
| `project list` | `GET /orgs/{org}/projects`, `GET /repos/{o}/{r}/projects` | |
| `project view` | `GET /projects/{id}` | |
| `project item-add` | `POST /projects/{id}/items` | |

### `secret` / `variable` — new command groups
| Subcommand | Endpoints | Notes |
|-----------|-----------|-------|
| `secret list/set/delete` | `GET/PUT/DELETE /repos/{o}/{r}/actions/secrets/{name}` | |
| `variable list/set/delete` | `GET/POST/PATCH/DELETE /repos/{o}/{r}/actions/variables/{name}` | |

### `ssh-key` / `gpg-key` — new command groups
| Subcommand | Endpoints | Notes |
|-----------|-----------|-------|
| `ssh-key list/add/delete` | `GET/POST/DELETE /user/keys` | |
| `gpg-key list/add/delete` | `GET/POST/DELETE /user/gpg_keys` | |

---

## Phase 4: Advanced & GHES-specific

### `attestation` — new command group
| Subcommand | Endpoints | Notes |
|-----------|-----------|-------|
| `attestation verify` | `GET /repos/{o}/{r}/attestations` | Build provenance |

### `ruleset` — new command group
| Subcommand | Endpoints | Notes |
|-----------|-----------|-------|
| `ruleset list/view` | `GET /repos/{o}/{r}/rulesets` | |

### `cache` — new command group
| Subcommand | Endpoints | Notes |
|-----------|-----------|-------|
| `cache list/delete` | `GET/DELETE /repos/{o}/{r}/actions/caches` | |

### `codespace` — new command group (github.com only)
| Subcommand | Endpoints | Notes |
|-----------|-----------|-------|
| `codespace list/create/stop/delete/ssh` | Various `/user/codespaces` endpoints | Not available on GHES 3.20 |

### `api graphql` — GraphQL support
| Subcommand | Endpoints | Notes |
|-----------|-----------|-------|
| `api graphql` | `POST /api/graphql` | Query, variables, pagination |

---

## Endpoint Count by Phase

| Phase | New Endpoints | Cumulative |
|-------|--------------|------------|
| 0 (baseline) | ~25 | 25 |
| 1 (core completion) | ~60 | ~85 |
| 2 (search, gists, CI) | ~80 | ~165 |
| 3 (org, user, project) | ~70 | ~235 |
| 4 (advanced) | ~50 | ~285 |

**Total REST surface:** ~1,005 endpoints. gor targets ~285 (28%) — the ones that matter for CLI use. The remaining ~720 are either GHES-admin-only, niche, or better suited to direct `gor api` calls.

---

## Key Design Decisions

1. **REST-first, GraphQL later** — REST covers all P0/P1 needs; GraphQL is a Phase 4 optimization
2. **gix over git2** — Pure Rust, no OpenSSL dependency
3. **Multi-host from day one** — GHES support is not an afterthought; the host resolution chain is built in
4. **`--json` field selection** — Must move from all-or-nothing to field-scoped (like `gh`) for scripting parity
5. **No interactive prompts in v1** — Flag-driven only; interactive TUI is a separate concern
