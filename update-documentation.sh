#!/bin/bash

set -o errexit -o nounset

git checkout gh-pages
git checkout master README.md
pandoc --css https://gist.githubusercontent.com/tleonardi/b04fde6538b4a3fe16a6/raw/5c9aaff140f79a76c39a3abcd368004119933612/github-pandoc.css --self-contained --highlight-style=tango -s -f markdown -t html5 -o index.html README.md
git reset HEAD README.md
rm README.md
git commit index.html -m "Update index.html"
git push
