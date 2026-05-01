#!/usr/bin/env bash
set -e

GREEN="\033[0;32m"
YELLOW="\033[1;33m"
CYAN="\033[0;36m"
RED="\033[0;31m"
BOLD="\033[1m"
RESET="\033[0m"
CHECK="${GREEN}✅${RESET}"
FAIL="${RED}❌${RESET}"
INFO="${CYAN}➜${RESET}"

REPO="vimlinuz/mdwatch"
INSTALL_DIR="${INSTALL_DIR:-$HOME/.local/bin}"

echo -e "${BOLD}${CYAN}"
echo "┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓"
echo "┃           mdwatch Installer           ┃"
echo "┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛"
echo -e "${RESET}"

os=$(uname -s)
arch=$(uname -m)

case "$os" in
    Linux) os="linux" ;;
    Darwin) os="darwin" ;;
    *)
        echo -e "${FAIL} Unsupported OS: $os"
        exit 1
        ;;
esac

case "$arch" in
    x86_64|amd64) arch="x86_64" ;;
    arm64|aarch64) arch="aarch64" ;;
    *)
        echo -e "${FAIL} Unsupported architecture: $arch"
        exit 1
        ;;
esac

target="${arch}-unknown-${os}-gnu"
if [ "$os" = "darwin" ]; then
    target="${arch}-apple-darwin"
fi

echo -e "${INFO} Detecting latest release..."
latest_json=$(curl -sSfL "https://api.github.com/repos/${REPO}/releases/latest")
tag=$(printf "%s" "$latest_json" | grep '"tag_name"' | head -1 | sed -E 's/.*"tag_name":\s*"([^"]+)".*/\1/')

if [ -z "$tag" ]; then
    echo -e "${FAIL} Failed to determine latest release tag."
    exit 1
fi

asset="mdwatch-${tag#v}-${target}.tar.gz"
url="https://github.com/${REPO}/releases/download/${tag}/${asset}"

tmp_dir=$(mktemp -d)
cleanup() { rm -rf "$tmp_dir"; }
trap cleanup EXIT

echo -e "${INFO} Downloading ${asset}..."
if ! curl -sSfL "$url" -o "$tmp_dir/$asset"; then
    echo -e "${YELLOW}No prebuilt binary for ${target}.${RESET}"
    echo -e "${YELLOW}You can install via Cargo instead:${RESET} cargo install mdwatch"
    exit 1
fi

echo -e "${INFO} Extracting archive..."
tar -xzf "$tmp_dir/$asset" -C "$tmp_dir"

if [ ! -f "$tmp_dir/mdwatch" ]; then
    echo -e "${FAIL} Extracted binary not found."
    exit 1
fi

mkdir -p "$INSTALL_DIR"
cp "$tmp_dir/mdwatch" "$INSTALL_DIR/mdwatch"
chmod +x "$INSTALL_DIR/mdwatch"

echo -e "${CHECK} mdwatch installed to ${INSTALL_DIR}."

case ":$PATH:" in
    *":$INSTALL_DIR:"*)
        echo -e "${CHECK} You can now run '${BOLD}mdwatch${RESET}' from anywhere in your terminal."
        ;;
    *)
        echo -e "${YELLOW}Add ${INSTALL_DIR} to your PATH to run 'mdwatch' globally.${RESET}"
        ;;
esac
