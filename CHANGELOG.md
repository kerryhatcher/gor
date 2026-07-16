# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.1](https://github.com/kerryhatcher/gor/compare/v0.1.0...v0.1.1) - 2026-07-16

### Added

- *(pr)* add pr view command
- *(pr)* add pr list command
- *(repo)* add repo view command
- *(auth)* implement OAuth device flow login
- implement config get/set/list command
- add user stories for all previously-uncovered gh command families
- add user stories for cache list and delete
- add user stories for ruleset list and view
- add user story for attestation verify
- add user stories for gpg-key list and add
- add user stories for ssh-key list and add
- add user stories for variable list, set, and delete
- add user stories for project list, view, and item-add
- add user stories for codespace list, create, delete, ssh, logs, and stop
- add user stories for extension list, install, remove, and upgrade
- add user stories for alias set, list, and delete
- add user story for secret delete
- add user stories for org view, secret list, and secret set
- add user stories for search commits, config, and org list
- add user stories for release delete, workflow view, and workflow enable/disable
- add user stories for pr edit, pr checks, and release edit
- add user stories for issue comment, repo edit, and repo sync
- add user story for repo sync
- add user stories for api graphql, auth setup-git, and pr comment
- add remaining user stories for label delete, label clone, repo transfer, and all initial stories
- add user stories for label delete, label clone, and repo transfer
- add user stories for gist edit, gist delete, and label edit
- add user stories for run cancel, run rerun, and run download
- add user stories for release upload, release download, and run watch
- add minimal Rust CLI skeleton
- add Trivy and Kingfisher security scanning
- initialize project infrastructure with Rust best practices

### Fixed

- resolve critical defects in api-call and repo-create user stories
- remove unused serde and serde_json dependencies
- remove remaining unused dev-dependencies
- remove unused dependencies flagged by cargo-shear
- update deny.toml for cargo-deny v3 compatibility
- replace cargo-deny-action with direct cargo-deny install
- bump MSRV to 1.86, fix doc links, add libdbus to CI
- install libdbus-1-dev in CI for keyring feature
- use valid kingfisher output format (pretty, not table)
- correct kingfisher.yaml config schema

### Other

- add pre-flight CI check and post-commit CI watch to gor-dev skill
- fix trivy-action commit hash
- add explicit toolchain input to dtolnay/rust-toolchain steps
- backfill ADRs for all major architectural decisions
- add ADR instructions to AGENTS.md and adrs skill
- switch adrs to NextGen mode with MADR format
- configure adrs tool to use docs/adr directory
- *(wrangler)* add Step 0 scout for 'next story' auto-selection
- add wrangler skill for cheap-worker delegation workflow
- mark auth-login/logout/status as done, groom backlog
- mark auth-login story as in_progress
- mark config user story as done with implementation notes
- add CONTRIBUTING.md for human developers
- add AGENTS.md with AI agent development guide
- gitignore pi subagent artifacts
- Initial commit
