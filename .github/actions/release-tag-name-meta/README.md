# Release tag name meta action

Extract release metadata from the tag name.

## Inputs

## `git-ref-name`

**Required** The git ref name.

## Outputs

## `name`

The prerelease/release name.

## `prerelease`

Is it release candidate?

## `version`

The prerelease/release version.

## Example usage

uses: actions/release-tag-name-meta@v1
with:
  git-ref-name: 'v0.1.0'
