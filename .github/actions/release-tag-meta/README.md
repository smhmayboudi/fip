# Release tag meta action

Extract release metadata from the tag.

## Inputs

## `git-ref`

**Required** The git ref (i.e. starting with refs/tags/).

## Outputs

## `name`

The release name (without the refs/tags/v prefix).

## `tag`

The git tag (without the refs/tags/ prefix).

## Example usage

uses: actions/release-tag-meta@v1
with:
  git-ref: 'refs/tags/v0.1.0'
