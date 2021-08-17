#!/bin/bash

PROBLEM=${1?}
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"

cargo new $PROBLEM --lib
sed -i '1s#^#//! https://leetcode.com/problems/shuffle-string/\n\npub struct Solution {}\n\nimpl Solution {\n}\n\n#' $SCRIPT_DIR/$PROBLEM/src/lib.rs
echo "vim ${PROBLEM}/src/lib.rs"
echo "git add ${PROBLEM} && git commit -m \"Add solution for problem ${PROBLEM}.\" && git push"
