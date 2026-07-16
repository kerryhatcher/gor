---
number: 3
title: REST-first API strategy with GraphQL deferred
status: accepted
date: 2026-07-16
tags: [api, architecture]
deciders: [kwhatcher]
---

# REST-first API strategy with GraphQL deferred

## Context and Problem Statement

gor interacts with the GitHub API for all operations. GitHub offers two API paradigms: REST (v3) and GraphQL (v4). Each has different strengths, and gor must decide which to use as its primary integration surface. Should gor use REST, GraphQL, or a hybrid approach?

## Decision Drivers

* **Coverage** — all P0/P1 features must be implementable with the chosen API
* **Simplicity** — REST is simpler to implement, debug, and test
* **Performance** — GraphQL can reduce over-fetching and round-trips for complex queries
* **Rate limits** — REST and GraphQL have different rate limit models
* **Ecosystem** — REST has better tooling support (e.g., `wiremock` for testing)

## Considered Options

* REST-only — use GitHub REST API v3 for all operations
* GraphQL-only — use GitHub GraphQL API v4 for all operations
* Hybrid — use REST for simple operations, GraphQL for complex queries
* **REST-first, GraphQL deferred** — start with REST; add GraphQL later if needed

## Decision Outcome

Chosen option: **REST-first, GraphQL deferred**, because REST covers all P0/P1 features with simpler implementation, debugging, and testing. GraphQL is deferred to a future phase when specific use cases (e.g., complex cross-entity queries) demonstrate a clear need.

### Consequences

* Good, because REST endpoints are well-documented and stable
* Good, because REST responses are easier to deserialize with `serde`
* Good, because REST is simpler to mock in tests with `wiremock`
* Good, because REST pagination (Link headers) is straightforward to implement
* Good, because faster time-to-market for v1 — no GraphQL query builder needed
* Bad, because some operations may require multiple REST calls where GraphQL could do one
* Bad, because REST can over-fetch data compared to GraphQL's field selection
* Bad, because GraphQL will need to be added later if complex query patterns emerge

### Confirmation

All current command implementations (`repo view`, `repo list`, `pr list`, `auth login`, etc.) use REST endpoints exclusively. The `client.rs` module is built around REST patterns (URL building, pagination, JSON deserialization). No GraphQL client code exists in the codebase.

## Pros and Cons of the Options

### REST-only

Use GitHub REST API v3 for all operations.

* Good, because well-documented with comprehensive endpoint coverage
* Good, because simple HTTP semantics (GET, POST, PATCH, DELETE)
* Good, because easy to test with standard HTTP mocking tools
* Good, because pagination via Link headers is standardized
* Neutral, because rate limits are per-endpoint (5000 requests/hour for authenticated users)
* Bad, because can over-fetch data (no field selection)
* Bad, because complex queries may require multiple round-trips

### GraphQL-only

Use GitHub GraphQL API v4 for all operations.

* Good, because precise field selection eliminates over-fetching
* Good, because complex cross-entity queries in a single request
* Good, because strongly typed schema enables code generation
* Bad, because steeper learning curve for contributors
* Bad, because harder to mock in tests (single endpoint, POST-only)
* Bad, because rate limits are calculated differently (points-based) and harder to predict
* Bad, because some REST endpoints have no GraphQL equivalent

### Hybrid

Use REST for simple CRUD, GraphQL for complex queries.

* Good, because best tool for each job
* Bad, because two code paths to maintain, test, and debug
* Bad, because inconsistent error handling and pagination patterns
* Bad, because increases cognitive load for contributors

### REST-first, GraphQL deferred

Start with REST; add GraphQL later if needed.

* Good, because fastest path to v1 with full feature coverage
* Good, because GraphQL can be added incrementally without breaking changes
* Good, because defers complexity until concrete use cases justify it
* Neutral, because requires designing the client abstraction to accommodate future GraphQL
* Bad, because may require refactoring if GraphQL is added later

## More Information

This decision should be revisited when:
- A specific feature requires data that would take 3+ REST calls but 1 GraphQL query
- Rate limit pressure from REST becomes a bottleneck
- Users request GraphQL-specific features (e.g., `gor api graphql`)
