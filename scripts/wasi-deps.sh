#!/usr/bin/env bash
set -e

# Sanitize VERSION: remove leading "v" if present
RAW_VERSION="$1"
VERSION="${RAW_VERSION#v}"

# Check that VERSION matches semver (e.g., 1.2.3)
if ! [[ "$VERSION" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
  echo "Error: VERSION must be in semver format (e.g., 1.2.3 or v1.2.3)"
  exit 1
fi

echo "io = \"https://github.com/WebAssembly/wasi-io/archive/v${VERSION}.tar.gz\"
random = \"https://github.com/WebAssembly/wasi-random/archive/v${VERSION}.tar.gz\"
clocks = \"https://github.com/WebAssembly/wasi-clocks/archive/v${VERSION}.tar.gz\"
filesystem = \"https://github.com/WebAssembly/wasi-filesystem/archive/v${VERSION}.tar.gz\"
sockets = \"https://github.com/WebAssembly/wasi-sockets/archive/v${VERSION}.tar.gz\"
cli = \"https://github.com/WebAssembly/wasi-cli/archive/v${VERSION}.tar.gz\"
http = \"https://github.com/WebAssembly/wasi-http/archive/v${VERSION}.tar.gz\"" > deps.toml

wit-deps -d wasip2 -m deps.toml -l deps.lock
rm deps.toml
rm deps.lock

# Update wasip2/README.md with the new version
if [ -f wasip2/README.md ]; then
    echo "Updating wasip2/README.md with version ${VERSION}"
  sed -i.bak -E "s/[0-9]+\.[0-9]+\.[0-9]+/${VERSION}/g" wasip2/README.md
  rm wasip2/README.md.bak
fi
