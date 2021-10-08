#!/bin/bash

set -eux
set -o errexit
set -o pipefail
set -o nounset

# for FILE in $(git diff-index --cached --name-only --diff-filter=AM HEAD | grep .rs | grep -v 'D' | grep -v 'R'); do
# 	test "$(cat $FILE | make hunspell | sort | uniq -c)" == "" || {
# 		echo >&2 "hunspell found (a) misspelled word(s) on \"$FILE\""
# 		exit 1
# 	}
# done

make clippy fmt-check

for LINE in $(git diff --staged --name-status | grep .rs | grep -v 'D' | grep -v 'R'); do
	FILE=$(echo $LINE | awk 'match($0, /.*/) {print $2}')
	git add $FILE
done
