#!/bin/bash
set -e

# Branch from which github create the projects page
DOC_BRANCH="gh-pages"
# Should the projects README.md converted into the index.html file?
## If you which so, make sure the system has a working `pandoc` installation!
CREATE_UPDATE_README=true


[[ "$(git symbolic-ref --short HEAD)" == "master" ]] || exit 0

msg() {
    echo "[1;34m> [1;32m$@[0m"
}

dir="$(pwd)"
last_rev="$(git rev-parse HEAD)"
last_msg="$(git log -1 --pretty=%B)"

unset GIT_WORK_TREE

msg "Cloning into a temporary directory..."
# The second call is to support OSX.
tmp="$(mktemp -d 2>/dev/null || mktemp -d -t 'tmp-rust-docs')"
trap "cd \"$dir\"; rm -rf \"$tmp\"" EXIT
git clone --quiet --branch master "$dir" "$tmp"

cd "$tmp"
ln -s "$dir/target" "$tmp/target"

msg "Generating documentation..."
cargo doc


# Switch to pages
msg "Replacing documentation..."
# Only if $DOC_BRANCH not exists
if ! git checkout --quiet "$DOC_BRANCH" 2>/dev/null; then
    git checkout --quiet --orphan "$DOC_BRANCH"
    git rm --quiet --ignore-unmatch -rf .
    cat > .gitignore <<EOF
target
Cargo.lock
EOF
    git add .gitignore
    git commit -m "Initial commit."
fi


# Clean
git rm --quiet --ignore-unmatch -rf .


# index.html patch.
## If a index.html exist, update it. Here i use `pandoc`, modify this for your needs.
if "$CREATE_UPDATE_README"; then
  msg "Create or update index.html with the content or the projects README.md"
  git checkout master README.md
  git checkout master share/pandoc.css
  pandoc --css share/pandoc.css --self-contained --highlight-style=tango -s -f markdown -t html5 -o index.html README.md
  git add index.html
  rm -r share
  rm README.md
fi

# Restore gitignore
git reset --quiet -- .gitignore
git checkout --quiet -- .gitignore

# Copy documentation into root
cp -a target/doc/* .


# Remove unneeded files
rm target

# Add all (new) files to git and commit them.
git add .
git commit -m "Update docs for $last_rev" -m "$last_msg"

cd $dir
git push --set-upstream origin "$DOC_BRANCH"

msg "Done."
