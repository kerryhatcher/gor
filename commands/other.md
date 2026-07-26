---
description: Additional commands — browse, API, config, orgs, projects, rulesets, extensions, attestations, copilot, classroom
---

# Additional Commands via gor

```bash
# Browse — open in browser
gor browse                     # Open current repo
gor browse --issue 123         # Open issue
gor browse --pr 42             # Open PR
gor browse --settings          # Open repo settings

# API — arbitrary REST calls
gor api /repos/owner/repo
gor api /repos/owner/repo/issues --method POST --field title="Bug"
gor api graphql -f query="query { viewer { login } }"

# Config
gor config list
gor config set --host github.com git_protocol ssh

# Organizations
gor org list
gor org view my-org

# Projects (GitHub Projects v2)
gor project list -R owner/repo
gor project view PROJECT_NUMBER -R owner/repo
gor project item-add PROJECT_NUMBER -R owner/repo --title "Task"

# Rulesets
gor ruleset list -R owner/repo
gor ruleset view RULESET_ID -R owner/repo

# Extensions
gor extension list
gor extension install owner/repo

# Attestations
gor attestation verify path/to/artifact

# Copilot
gor copilot status
gor copilot usage --org my-org

# Classroom
gor classroom list
gor classroom view ASSIGNMENT_ID
```
