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

echo -e "${BOLD}${CYAN}"
echo "┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓"
echo "┃           mdwatch Installer           ┃"
echo "┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛"
echo -e "${RESET}"

echo -e "${INFO} Checking for Rust toolchain..."
if ! command -v cargo >/dev/null 2>&1; then
    echo -e "${YELLOW}Rust is not installed. Installing via rustup...${RESET}"
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    export PATH="$HOME/.cargo/bin:$PATH"
    echo -e "${CHECK} Rust installed!"
else
    echo -e "${CHECK} Rust is already installed."
fi

echo -e "${INFO} Cloning mdwatch repository..."
if [ -d "$HOME/mdwatch" ]; then
    echo -e "${YELLOW}A previous mdwatch directory exists. Updating repository...${RESET}"
    cd "$HOME/mdwatch"
    git pull
else
    git clone --depth 1 --branch main https://github.com/santoshxshrestha/mdwatch.git "$HOME/mdwatch"
fi

echo -e "${INFO} Building mdwatch in release mode..."
cd "$HOME/mdwatch"
cargo build --release

BINARY_PATH="$HOME/mdwatch/target/release/mdwatch"
INSTALL_DIR="/usr/local/bin"
if [ ! -f "$BINARY_PATH" ]; then
    echo -e "${FAIL} Error: Release binary not found at $BINARY_PATH."
    exit 1
fi

echo -e "${INFO} Installing mdwatch to ${INSTALL_DIR} (may need sudo)..."
sudo cp "$BINARY_PATH" "$INSTALL_DIR/mdwatch"
sudo chmod +x "$INSTALL_DIR/mdwatch"

echo -e "${CHECK} mdwatch installed to ${INSTALL_DIR} and available in your PATH."

echo -e "${CHECK} You can now run '${BOLD}mdwatch${RESET}' from anywhere in your terminal."
