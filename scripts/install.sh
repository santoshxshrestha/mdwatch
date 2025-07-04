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
echo "┃           mdserve Installer           ┃"
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

echo -e "${INFO} Cloning mdserve repository..."
if [ -d "$HOME/mdserve" ]; then
    echo -e "${YELLOW}A previous mdserve directory exists. Updating repository...${RESET}"
    cd "$HOME/mdserve"
    git pull
else
    git clone --depth 1 --branch main https://github.com/santoshxshrestha/mdserve.git "$HOME/mdserve"
fi

echo -e "${INFO} Building mdserve in release mode..."
cd "$HOME/mdserve"
cargo build --release

BINARY_PATH="$HOME/mdserve/target/release/mdserve"
INSTALL_DIR="/usr/local/bin"
if [ ! -f "$BINARY_PATH" ]; then
    echo -e "${FAIL} Error: Release binary not found at $BINARY_PATH."
    exit 1
fi

echo -e "${INFO} Installing mdserve to ${INSTALL_DIR} (may need sudo)..."
sudo cp "$BINARY_PATH" "$INSTALL_DIR/mdserve"
sudo chmod +x "$INSTALL_DIR/mdserve"

echo -e "${CHECK} mdserve installed to ${INSTALL_DIR} and available in your PATH."

echo -e "\n${BOLD}${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${RESET}"
echo -e "${BOLD}  Nerd Font Required for Best Icon Support  ${RESET}"
echo -e "  ${YELLOW}For best visual experience, set your terminal font to a Nerd Font."
echo -e "  Download: ${CYAN}https://www.nerdfonts.com/font-downloads${RESET}"
echo -e "${BOLD}${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${RESET}\n"

echo -e "${CHECK} You can now run '${BOLD}mdserve${RESET}' from anywhere in your terminal."
