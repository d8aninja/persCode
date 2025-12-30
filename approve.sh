# Navigate to your repo directory
# cd /path/to/your/repo

# List all open PRs
gh pr list

# Approve all open PRs
gh pr list --json number --jq '.[].number' | xargs -I {} gh pr review {} --approve

# Merge all open PRs (after approving)
gh pr list --json number --jq '.[].number' | xargs -I {} gh pr merge {} --merge

# Or do both at once (approve and merge)
gh pr list --json number --jq '.[].number' | xargs -I {} sh -c 'gh pr review {} --approve && gh pr merge {} --merge'

# If you want to squash merge instead:
gh pr list --json number --jq '.[].number' | xargs -I {} sh -c 'gh pr review {} --approve && gh pr merge {} --squash'

# If you want to rebase merge:
gh pr list --json number --jq '.[].number' | xargs -I {} sh -c 'gh pr review {} --approve && gh pr merge {} --rebase'