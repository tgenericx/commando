#!/bin/sh
# Commando installation script for Linux and macOS
# Usage: curl -sSfL https://raw.githubusercontent.com/tgenericx/commando/main/install.sh | sh

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Detect OS and architecture
detect_platform() {
    OS="$(uname -s)"
    ARCH="$(uname -m)"
    
    case "$OS" in
        Linux*)
            PLATFORM="linux"
            ;;
        Darwin*)
            PLATFORM="macos"
            ;;
        *)
            echo "${RED}Error: Unsupported operating system: $OS${NC}"
            exit 1
            ;;
    esac
    
    case "$ARCH" in
        x86_64)
            ARCH="x86_64"
            ;;
        aarch64|arm64)
            ARCH="aarch64"
            ;;
        *)
            echo "${RED}Error: Unsupported architecture: $ARCH${NC}"
            exit 1
            ;;
    esac
    
    echo "${GREEN}Detected platform: $PLATFORM-$ARCH${NC}"
}

# Get latest version from GitHub
get_latest_version() {
    echo "${YELLOW}Fetching latest version...${NC}"
    VERSION=$(curl -sSf https://api.github.com/repos/tgenericx/commando/releases/latest | grep '"tag_name"' | sed -E 's/.*"v([^"]+)".*/\1/')
    
    if [ -z "$VERSION" ]; then
        echo "${RED}Error: Could not determine latest version${NC}"
        exit 1
    fi
    
    echo "${GREEN}Latest version: v$VERSION${NC}"
}

# Download binary
download_binary() {
    BINARY_NAME="commando-${PLATFORM}-${ARCH}"
    DOWNLOAD_URL="https://github.com/tgenericx/commando/releases/download/v${VERSION}/${BINARY_NAME}"
    
    echo "${YELLOW}Downloading from: $DOWNLOAD_URL${NC}"
    
    TMPDIR=$(mktemp -d)
    cd "$TMPDIR"
    
    if ! curl -sSfLO "$DOWNLOAD_URL"; then
        echo "${RED}Error: Failed to download binary${NC}"
        exit 1
    fi
    
    chmod +x "$BINARY_NAME"
    echo "${GREEN}Download complete${NC}"
}

# Install binary
install_binary() {
    # Try to install to /usr/local/bin first (requires sudo)
    INSTALL_DIR="/usr/local/bin"
    
    if [ -w "$INSTALL_DIR" ]; then
        mv "$BINARY_NAME" "$INSTALL_DIR/commando"
        echo "${GREEN}Installed to $INSTALL_DIR/commando${NC}"
    else
        echo "${YELLOW}Need sudo to install to $INSTALL_DIR${NC}"
        if sudo -v 2>/dev/null; then
            sudo mv "$BINARY_NAME" "$INSTALL_DIR/commando"
            echo "${GREEN}Installed to $INSTALL_DIR/commando${NC}"
        else
            # Fallback to ~/.local/bin
            INSTALL_DIR="$HOME/.local/bin"
            mkdir -p "$INSTALL_DIR"
            mv "$BINARY_NAME" "$INSTALL_DIR/commando"
            echo "${GREEN}Installed to $INSTALL_DIR/commando${NC}"
            echo "${YELLOW}Make sure $INSTALL_DIR is in your PATH${NC}"
            
            # Check if in PATH
            if ! echo "$PATH" | grep -q "$INSTALL_DIR"; then
                echo "${YELLOW}Add this to your ~/.bashrc or ~/.zshrc:${NC}"
                echo "  export PATH=\"\$HOME/.local/bin:\$PATH\""
            fi
        fi
    fi
    
    # Cleanup
    cd - > /dev/null
    rm -rf "$TMPDIR"
}

# Verify installation
verify_installation() {
    if command -v commando >/dev/null 2>&1; then
        VERSION_OUTPUT=$(commando --version 2>&1 || echo "unknown")
        echo "${GREEN}✓ Installation successful!${NC}"
        echo "Version: $VERSION_OUTPUT"
        echo ""
        echo "Try it out:"
        echo "  cd /path/to/your/git/repo"
        echo "  git add <files>"
        echo "  commando"
    else
        echo "${YELLOW}Warning: commando not found in PATH${NC}"
        echo "You may need to restart your shell or add the installation directory to PATH"
    fi
}

# Main installation flow
main() {
    echo "${GREEN}=== Commando Installer ===${NC}"
    echo ""
    
    detect_platform
    get_latest_version
    download_binary
    install_binary
    verify_installation
    
    echo ""
    echo "${GREEN}Installation complete!${NC}"
}

main
