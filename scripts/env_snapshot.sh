#!/usr/bin/env bash

set -euo pipefail

echo "# Environment snapshot"
echo "timestamp: $(date -u +%Y-%m-%dT%H:%M:%SZ)"
echo "kernel: $(uname -a)"
echo "distro: $(lsb_release -ds 2>/dev/null || cat /etc/os-release | grep PRETTY_NAME)"
echo

echo "## Rust toolchain"
rustc --version --verbose
cargo --version
echo

echo "## C toolchain"
cc --version | head -n1
ld --version | head -n1
echo

echo "## bindgens's libclang"
clang --version | head -n1
find /usr/lib -name 'libclang*.so*' 2>/dev/null | head -n1 || echo "libclang not found"
echo

echo "## glibc"
ldd --version | head -n1
echo

echo "## Git state"
git rev-parse HEAD 2>/dev/null || echo "not a git repo"
git status --porcelain 2>/dev/null | head -n20
echo

echo "## Cargo.lock hash (detects dependency drift without diffing it)"
sha256sum  Cargo.lock 2>/dev/null || echo "no Cargo.lock"

