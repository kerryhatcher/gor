# GitHub API Overview — Fundamentals for gor

Target: **GHES 3.20** (GA 2026-03-17, EOL 2027-03-17) with full **github.com** compatibility.
Research compiled 2026-07 from official GitHub Enterprise Server 3.20 REST API documentation.

---

## Base URLs

| Target | REST Base | GraphQL Base |
|--------|-----------|--------------|
| github.com | `https://api.github.com` | `https://api.github.com/graphql` |
| GHES | `https://HOSTNAME/api/v3` | `https://HOSTNAME/api/graphql` |
| Manage-GHES | `https://HOSTNAME:8443/manage/v1` | — |

Manage-GHES uses **HTTP Basic** with the Management Console password, not bearer tokens.

---

## API Versioning

- Header: `X-GitHub-Api-Version: 2022-11-28` (default if omitted)
- GHES 3.20 supports only this one version, through **March 10, 2028**
- **Breaking changes** (trigger a new version): removing operations, renaming fields/params, adding required params, changing types or auth requirements
- **Non-breaking changes** (roll out to all versions): new operations, new optional params, new response fields, new enum values
- Deprecated versions return `Deprecation` + `Sunset` headers, then `410 Gone` after the window closes

**Implication for gor:** Hardcode `2022-11-28` as the API version. No need for version negotiation until GitHub ships a new version.

---

## Authentication

### Methods (in preference order)

1. **Fine-grained PAT** — GitHub's recommended replacement for classic PATs; per-endpoint permission docs
2. **GitHub App tokens** — Best for automation; user-access tokens expire after 60 min, need client ID + private key
3. **Classic PAT** — Legacy but still widely used; scoped at creation
4. **OAuth App tokens** — Web/device flow; less preferred than GitHub Apps
5. **`GITHUB_TOKEN`** — Auto-available inside Actions workflows
6. **Basic auth** — Only for specific App/OAuth endpoints (client ID as username, secret as password)

### Header format

```
Authorization: Bearer <token>
Accept: application/vnd.github+json
X-GitHub-Api-Version: 2022-11-28
```

- PATs and OAuth tokens accept both `Bearer` and `token` prefixes
- JWTs (GitHub App auth as the app itself) **must** use `Bearer`
- Always pair with `Accept: application/vnd.github+json`

### Token resolution chain

1. Environment variables (`GH_TOKEN`/`GITHUB_TOKEN` for github.com, `GH_ENTERPRISE_TOKEN`/`GITHUB_ENTERPRISE_TOKEN` for GHES)
2. OS keyring (via `keyring` crate)
3. Config file (`~/.config/gor/hosts.yml`, mode 0600)

---

## Rate Limits

### github.com
- **Primary:** 5,000 requests/hour (authenticated), 60/hour (unauthenticated)
- **Secondary:** 100 concurrent requests, 900 points/min, content creation limits
- Headers: `x-ratelimit-limit`, `x-ratelimit-remaining`, `x-ratelimit-reset`, `x-ratelimit-used`

### GHES
- **Disabled by default** — site admins must explicitly enable
- When enabled, limits are admin-configured (not fixed defaults)
- Check `/rate_limit` endpoint for current values
- GraphQL node limits **always apply** regardless of rate-limit config

### GraphQL node limits (always enforced)
- `first`/`last` must be 1–100 on every connection
- Max 500,000 total nodes per call
- Node counts multiply across nested levels (50 repos × 10 issues = 550 nodes)
- Mitigation: smaller page sizes, shallower nesting, split large queries

**Implication for gor:** Always check `x-ratelimit-remaining` on responses. Implement exponential backoff with retry-after for 429s. For list commands, respect pagination rather than trying to fetch everything at once.

---

## Pagination

### REST
- `Link` header with `rel="next"`, `rel="last"`, `rel="prev"`, `rel="first"`
- Query params: `per_page` (max 100) and `page`
- **Direction:** Offset-based pagination is being deprecated on Dependabot endpoints in favor of cursor-based (`before`/`after`/`per_page`) — this is the direction all list endpoints are heading

### GraphQL
- Cursor-based only: `first`/`last` + `after`/`before` on connection fields
- Page info object: `endCursor`, `hasNextPage`, `hasPreviousPage`

**Implication for gor:** Implement `Link` header parsing for REST pagination. For `--paginate` flag, follow `rel="next"` until exhausted. Cap `--limit` at 100 per page and paginate transparently.

---

## Media Types

- Default: `Accept: application/vnd.github+json`
- Custom: `application/vnd.github.PARAM+json` (e.g., `application/vnd.github.diff` for diffs)
- Multiple types comma-separated: `Accept: application/vnd.github+json,application/vnd.github.diff`
- Responses include `*_url` fields as RFC 6570 URI templates for related resources

---

## REST API Surface Summary

~1,005 endpoints across these categories:

| Area | Endpoints | Priority for gor |
|------|-----------|-------------------|
| Repos, branches, commits, releases | 188 | **P0** — core CLI surface |
| Issues, PRs, checks, reactions | 98 | **P0** — core CLI surface |
| Git database, gists, search, meta | 60 | **P1** — search + gists |
| Actions (workflows, runs, artifacts) | 202 | **P1** — CI visibility |
| Orgs, teams, users, projects | 129 | **P2** — org management |
| Apps, webhooks, activity | 73 | **P2** — automation |
| Security (code/secret scanning, Dependabot) | 88 | **P3** — GHAS features |
| Packages, pages, migrations | 49 | **P3** — niche |
| Enterprise admin, SCIM, Manage-GHES | ~118 | **P3** — GHES-only |

---

## GraphQL API

- Single endpoint: `POST https://HOSTNAME/api/graphql`
- Schema-driven: types, queries, mutations, introspection
- Key advantage: single call replaces multiple REST requests; client specifies exact fields needed
- gor should support `gor api graphql` for arbitrary GraphQL queries (like `gh api graphql`)

---

## Open Items

- [ ] Confirm OpenAPI spec location for GHES 3.20 (check `github/rest-api-description` `ghes-3.20` branch)
- [ ] Verify fine-grained PAT availability on GHES 3.20
- [ ] Determine if gor should support Manage-GHES endpoints (separate port 8443, Basic auth)
- [ ] Evaluate whether to implement GraphQL support in v1 or defer
