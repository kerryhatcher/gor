---
tags: [api, read, write]
priority: P0
phase: 0
endpoints:
  - "ANY /{path}"
status: done
blockedBy: [auth-login]
blocks: [api-graphql]
---

# API — Arbitrary REST Calls

## As a

power user who needs to call GitHub API endpoints that `gor` doesn't wrap

## I want

to make arbitrary authenticated REST API calls and see the raw response

## Acceptance criteria

1. Running `gor api /repos/owner/repo` makes a `GET` request and prints the JSON response
2. `--method` / `-X` flag supports `GET`, `POST`, `PUT`, `PATCH`, `DELETE`
3. `--field` / `-F` flag sends form-encoded body parameters
4. `--raw-field` / `-f` flag sends raw body parameters (for JSON arrays, etc.)
5. `--header` / `-H` flag adds custom request headers
6. `--input` flag reads the request body from a file (`@-` for stdin)
7. `--paginate` flag follows `Link` headers to fetch all pages
8. `--hostname` flag targets a specific host
9. `--jq` flag filters the JSON response through a jq expression
10. `--template` flag formats output via Handlebars templates
11. `--silent` flag suppresses the status output
12. Response headers are shown with `--include` / `-i`
13. The endpoint path is automatically prefixed with the correct API base URL

## Out of scope

- GraphQL API calls (separate story)
- Streaming responses
- WebSocket connections
