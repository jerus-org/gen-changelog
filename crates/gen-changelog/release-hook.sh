#!/bin/bash
set -exo pipefail

# Release hook for gen-changelog crate
# Called by cargo release via pre-release-hook
# Runs from: crates/gen-changelog/

NAME="CHANGELOG.md"
PACKAGE="gen-changelog"
REPO_DIR="../.."

# NEW_VERSION is set by cargo release
VERSION="${NEW_VERSION:-${1}}"

if [[ -z "${VERSION}" ]]; then
    echo "Error: No version specified (set NEW_VERSION or pass as argument)" >&2
    exit 1
fi

# Build README from docs components
cat docs/readme/head.md > README.md
# shellcheck disable=SC2129
cat docs/readme/lib.md >> README.md
cat docs/main.md >> README.md
cat docs/readme/tail.md >> README.md

echo "Rebuilt README.md from docs components"

# Generate CHANGELOG.md
gen-changelog generate \
    --display-summaries \
    --name "${NAME}" \
    --repository-dir "${REPO_DIR}" \
    --next-version "${VERSION}"

echo "Generated ${NAME} for ${PACKAGE}@${VERSION}"

# Inject ephemeral signing pubkey into Cargo.toml if provided by CI
# Key generation is handled by the CI generate_signing_key command
if [[ -n "${BINSTALL_SIGNING_PUBKEY}" ]]; then
    sed -i "s/pubkey = \".*\"/pubkey = \"$BINSTALL_SIGNING_PUBKEY\"/" Cargo.toml
    echo "Cargo.toml updated with ephemeral signing key for ${PACKAGE}@${VERSION}"
fi
