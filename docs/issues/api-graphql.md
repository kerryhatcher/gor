---
tags: [api, read, write]
priority: P2
phase: 4
endpoints:
  - POST /graphql
status: done
blockedBy: [api-call]
blocks: []
---

# API GraphQL

## As a

power user who needs to call GitHub's GraphQL API for data that REST does not expose efficiently

## I want

to make arbitrary authenticated GraphQL queries from the command line

## Acceptance criteria

1. Running `gor api graphql --query 'query { viewer { login } }'` executes the query and prints the JSON response
2. `--query` / `-q` flag supplies the GraphQL query string directly
3. When `--query` is omitted, the query is read from stdin (one-shot, non-interactive)
4. `--field` / `-F` flag supplies GraphQL variables as `key=value` pairs (repeatable)
5. `--hostname` flag targets a specific host (github.com or GHES)
6. `--jq` flag filters the JSON response through a jq expression
7. `--template` flag formats the response via Handlebars templates
8. The JSON response is pretty-printed with 2-space indentation by default
9. GraphQL errors in the `errors` array are surfaced clearly with the error message and path, and the command exits non-zero
10. The endpoint path is automatically targeted at the host's GraphQL base URL (`/graphql`)

## Out of scope

- Schema introspection queries (users can supply these via `--query`)
- Interactive query editor / REPL
- Automatic pagination of connections (users supply cursor variables via `--field`)
