# Design Specification: Core Library Split (Ref #2)

## Project Goals
Separate the core logic and infrastructure of `gor` into a dedicated library crate (`gor-core`) to improve modularity, testability, and clear separation of concerns between business logic and CLI presentation.

## Proposed Structure
1. **gor-core**: A library crate containing:
    - Network communication (HTTP/REST) via `reqwest`.
    - Core data types and error handling (using `thiserror`).
    - Keyring management for authentication tokens.
    - Configuration logic and validation.
2. **gor** (CLI): The command-line interface wrapper. It will:
    - Parse CLI arguments with `clap`.
    - Provide progress reporting (`indicatif`) and pretty formatting (`console`/`eyre`).
    - Act as a thin layer that maps input to `gor-core` calls and decodes results for display.

## Design Decisions & Constraints (Ref #2)
- **Stability**: The CLI must remain behaviorally identical. No new flags or changes to existing output formatting are permitted during this refactor.
- **Core Clarity**: Core logic in `gor-core` must not depend on any UI/CLI libraries (`clap`, `indicatif`, etc.).
- **Type Safety**: Replace raw `serde_json::Value` usage inside the core with strongly typed models where possible.

## Roadmap
1. **Phase 1: Infrastructure Migration**. Move common files (`client`, `host`, `error`, `config`) to `gor-core`.
2. **Phase 2: Feature Extraction**. Iteratively move command logic (e.g., `label`, `issue`, `pr`) into the core library, ensuring they are fully testable in isolation.
3. **Phase 3: Final Cleanup**. Refine internal helper utilities and publish both crates.
