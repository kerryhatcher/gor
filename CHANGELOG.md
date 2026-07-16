# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.1](https://github.com/kerryhatcher/gor/compare/v0.1.0...v0.1.1) - 2026-07-16

### Added

- *(copilot)* add copilot status and usage commands
- *(classroom)* add classroom list and assignments commands
- *(project)* add Projects V2 support via GraphQL
- *(variable)* add environment-scoped variables support
- *(secret)* add environment-scoped secrets support
- *(ssh-key)* add ssh-key delete command
- *(run)* add run delete command
- *(release)* add release delete-asset command
- *(pr)* add pr ready command
- *(issue)* add issue transfer command
- *(gpg-key)* add gpg-key delete command
- *(completion)* add shell completion command
- *(codespace)* add codespace rebuild command
- *(codespace)* add codespace ports command
- *(codespace)* add codespace cp command
- *(auth)* add auth token command
- *(project)* add project view and item-add commands
- add project list and attestation verify commands
- add run cancel/download/rerun/watch and extension upgrade commands
- add codespace logs/ssh/stop and extension remove commands
- add cache, secret, variable, and codespace delete commands
- *(run)* add run view command
- add secret, variable, run, cache, ruleset, extension, codespace, workflow enable/disable/run, and org view commands
- *(cli)* add remaining command group definitions
- *(keys)* add ssh-key and gpg-key list/add commands
- *(org)* add org list command
- *(workflow)* add workflow view command
- *(gist)* add gist edit and delete commands
- *(gist)* add gist view command
- *(api)* add api graphql subcommand
- *(alias)* add alias delete command
- add browse, gist, search, workflow, and alias commands
- *(pr)* add pr diff, edit, review, and checks commands
- *(label)* add label create, edit, delete, and clone commands
- *(release)* add release create and download commands
- *(release)* add release upload command
- *(release)* add release edit command
- *(release)* add release delete command
- *(release)* add release view command
- *(repo)* add repo create, fork, delete, edit, transfer commands
- *(label)* add label list command
- *(issue)* add issue create command
- *(pr)* add pr checkout command
- *(pr)* add pr merge command
- *(pr)* add pr comment command
- *(pr)* add pr close and pr reopen commands
- *(pr)* add pr create command
- *(repo)* add repo clone command
- *(repo)* add repo list command
- *(issue)* add issue comment command
- *(issue)* add issue close and reopen commands
- *(auth)* add setup-git command to configure git credential helper
- *(api)* add gor api command for arbitrary REST API calls
- *(issue)* add issue view command
- *(issue)* add issue list command
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

- add 39 user stories for missing gh feature parity
- add user stories for remaining feature gaps
- mark repo-create, repo-fork, repo-delete, repo-edit, repo-transfer stories as done
- mark label-list and release-list stories as done
- mark issue-create story as done
- mark pr-checkout story as done
- mark pr-merge story as done
- mark pr-comment story as done
- mark pr-close story as done
- mark pr-create story as done
- mark repo-clone story as done
- mark repo-list story as done
- mark issue-comment story as done
- mark issue-close story as done
- mark auth-setup-git story as done
- mark api-call story as done
- mark issue-view story as done
- ignore RUSTSEC-2025-0119 (unmaintained number_prefix crate)
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
