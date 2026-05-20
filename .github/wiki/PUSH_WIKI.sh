#!/usr/bin/env bash
# Pousser le contenu wiki vers GitHub Wiki
# Prérequis : avoir créé la première page via l'UI GitHub (onglet Wiki > Create the first page)

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
WIKI_DIR="$SCRIPT_DIR"

REPO="MoggleJ/MJQbe_v2"
TMP="/tmp/mjqbe-wiki-push"

rm -rf "$TMP"
git clone "https://github.com/${REPO}.wiki.git" "$TMP"
cp "$WIKI_DIR"/*.md "$TMP/"
cd "$TMP"
git add .
git commit -m "Wiki : mise à jour depuis .github/wiki/" 2>/dev/null || echo "Rien à mettre à jour"
git push origin HEAD:master
echo "Wiki poussé → https://github.com/${REPO}/wiki"
