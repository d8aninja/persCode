gh pr list --json number --jq '.[].number' | ForEach-Object { gh pr review $_ --approve; gh pr merge $_ --merge }
