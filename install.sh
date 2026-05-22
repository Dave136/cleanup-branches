#!/bin/bash

set -e

# allow specifying different destination directory
DIR="${DIR:-"$HOME/.local/bin"}"

# map different architecture variations to the available binaries
ARCH=$(uname -m)
case $ARCH in
    i386|i686) ARCH=x86_64 ;;
    aarch64*) ARCH=aarch64 ;;
esac

# prepare the download URL
GITHUB_LATEST_VERSION=$(curl -L -s -H 'Accept: application/json' https://github.com/Dave136/cleanup-branches/releases/latest | sed -e 's/.*"tag_name":"\([^"]*\)".*/\1/')
GITHUB_FILE="cleanup-branches-${ARCH}-unknown-linux-gnu.tar.gz"
GITHUB_URL="https://github.com/Dave136/cleanup-branches/releases/download/${GITHUB_LATEST_VERSION}/${GITHUB_FILE}"

echo "Installing cleanup-branches ${GITHUB_LATEST_VERSION} for ${ARCH}..."

# download the release tarball
if ! curl -L -f -o cleanup-branches.tar.gz "$GITHUB_URL"; then
    echo "The requested file '${GITHUB_FILE}' for version '${GITHUB_LATEST_VERSION}' and architecture '${ARCH}' may not exist." >&2
    exit 1
fi

# extract and install
tar xzvf cleanup-branches.tar.gz cleanup-branches
install -Dm 755 cleanup-branches -t "$DIR"
rm cleanup-branches cleanup-branches.tar.gz

echo "cleanup-branches installed to ${DIR}/cleanup-branches"
