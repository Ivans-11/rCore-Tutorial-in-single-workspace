#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
CHECKER_DIR="${ROOT_DIR}/checker"
CHECKER_REPO_URL="https://github.com/LearningOS/rCore-Tutorial-Checker-2025S.git"

if ! command -v cargo >/dev/null 2>&1; then
  echo "cargo not found; install Rust toolchain first." >&2
  exit 1
fi

if ! command -v python3 >/dev/null 2>&1; then
  echo "python3 not found; install python3 first." >&2
  exit 1
fi

if [ ! -d "${CHECKER_DIR}/.git" ]; then
  echo "Cloning checker repo into ${CHECKER_DIR}..."
  git clone --depth 1 "${CHECKER_REPO_URL}" "${CHECKER_DIR}"
fi

if command -v rustup >/dev/null 2>&1; then
  rustup target add riscv64gc-unknown-none-elf
  rustup component add rust-src llvm-tools-preview
else
  echo "rustup not found; skipping target/component setup." >&2
fi

RUN_CMD=(cargo qemu --ch 5 --exercise --nobios)
if command -v timeout >/dev/null 2>&1; then
  timeout 10m "${RUN_CMD[@]}" 2>&1 | python3 "${CHECKER_DIR}/check/ch5.py"
else
  "${RUN_CMD[@]}" 2>&1 | python3 "${CHECKER_DIR}/check/ch5.py"
fi
