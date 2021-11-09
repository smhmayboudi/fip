# Pull request branch meta

Extract branch metadata from the pull request.

## Inputs

## `git-ref`

**Required** The git ref (i.e. starting with refs/heads/).

## Outputs

## `name`

The branch name (without the refs/heads/ prefix).

## Example usage

uses: actions/pull_request-branch-meta@v1
with:
  git-ref: 'refs/heads/0-build'
