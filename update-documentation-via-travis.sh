#!/bin/bash

set -o errexit -o nounset

if [ "$TRAVIS_BRANCH" != "master" ]
then
  echo "This commit was made against the $TRAVIS_BRANCH and not the master! No deploy!"
  exit 0
fi

rev=$(git rev-parse --short HEAD)

mkdir page

cd page

git init
git config user.name "Stefan Müller"
git config user.email "s.mueller@it.kls-glt.de"

git remote add upstream "https://$GH_TOKEN@github.com/Kliemann-Service-GmbH/xMZ-Mod-Touch-Server.git"
git fetch upstream
git reset upstream/gh-pages

git checkout upstream/master README.md
pandoc --css https://gist.githubusercontent.com/tleonardi/b04fde6538b4a3fe16a6/raw/5c9aaff140f79a76c39a3abcd368004119933612/github-pandoc.css --self-contained --highlight-style=tango -s -f markdown -t html5 -o index.html README.md
git reset HEAD README.md
rm README.md

touch .

git add -A .
git commit -m "Auto rebuild pages at ${rev}"
git push -q upstream HEAD:gh-pages
