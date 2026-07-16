---
tags: [completion, shell]
priority: P2
phase: 4
endpoints: []
status: done
blockedBy: []
blocks: []
---

# Completion

## As a

developer who wants fast tab-completion in my shell

## I want

to generate shell completion scripts for bash, zsh, fish, and PowerShell

## Acceptance criteria

1. Running `gor completion bash` outputs a bash completion script to stdout
2. Running `gor completion zsh` outputs a zsh completion script to stdout
3. Running `gor completion fish` outputs a fish completion script to stdout
4. Running `gor completion powershell` outputs a PowerShell completion script to stdout
5. The shell argument is a required positional (one of: `bash`, `zsh`, `fish`, `powershell`)
6. Completions cover all subcommands, flags, and positional arguments
7. Dynamic completions are provided where applicable (e.g., repository names, branch names)
8. The generated script can be sourced directly (e.g., `source <(gor completion bash)`)

## Out of scope

- Installing the completion script automatically (users source or place it themselves)
- Custom completion handlers for third-party extensions
