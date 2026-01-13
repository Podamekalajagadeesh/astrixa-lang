#!/bin/bash
set -e

# ASTRIXA Language Installer
# One-command installation: curl -fsSL https://astrixa.org/install | sh

ASTRIXA_VERSION="${ASTRIXA_VERSION:-latest}"
INSTALL_DIR="${ASTRIXA_INSTALL_DIR:-/usr/local/bin}"
TMP_DIR=$(mktemp -d)
REPO="Podamekalajagadeesh/astrixa-lang"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Helper functions
info() {
    echo -e "${BLUE}ℹ${NC} $1"
}

success() {
    echo -e "${GREEN}✓${NC} $1"
}

error() {
    echo -e "${RED}✗${NC} $1"
    exit 1
}

warning() {
    echo -e "${YELLOW}⚠${NC} $1"
}

# Detect OS and architecture
detect_platform() {
    OS=$(uname -s | tr '[:upper:]' '[:lower:]')
    ARCH=$(uname -m)
    
    case "$OS" in
        linux*)
            OS="linux"
            ;;
        darwin*)
            OS="macos"
            ;;
        msys*|mingw*|cygwin*)
            OS="windows"
            ;;
        *)
            error "Unsupported operating system: $OS"
            ;;
    esac
    
    case "$ARCH" in
        x86_64|amd64)
            ARCH="x86_64"
            ;;
        aarch64|arm64)
            ARCH="aarch64"
            ;;
        *)
            error "Unsupported architecture: $ARCH"
            ;;
    esac
    
    info "Detected platform: $OS-$ARCH"
}

# Check if running as root (needed for /usr/local/bin)
check_permissions() {
    if [ ! -w "$INSTALL_DIR" ]; then
        warning "No write permission to $INSTALL_DIR"
        info "Will attempt to install with sudo..."
        SUDO="sudo"
    else
        SUDO=""
    fi
}

# Download ASTRIXA binary
download_astrixa() {
    info "Downloading ASTRIXA $ASTRIXA_VERSION..."
    
    # For now, build from source (later: download pre-built binaries)
    if command -v cargo >/dev/null 2>&1; then
        info "Building from source..."
        cd "$TMP_DIR"
        git clone --depth 1 https://github.com/$REPO.git
        cd astrixa-lang/compiler
        cargo build --release
        
        BINARY_PATH="target/release/astrixa"
        if [ ! -f "$BINARY_PATH" ]; then
            error "Build failed"
        fi
        
        success "Built successfully"
    else
        error "Rust toolchain not found. Please install Rust first: https://rustup.rs"
    fi
}

# Install binary
install_binary() {
    info "Installing to $INSTALL_DIR..."
    
    $SUDO cp "$TMP_DIR/astrixa-lang/compiler/target/release/astrixa" "$INSTALL_DIR/astrixa"
    $SUDO chmod +x "$INSTALL_DIR/astrixa"
    
    success "Installed astrixa to $INSTALL_DIR"
}

# Install LSP server (optional)
install_lsp() {
    info "Installing ASTRIXA Language Server..."
    
    cd "$TMP_DIR/astrixa-lang/lsp"
    cargo build --release
    
    if [ -f "target/release/astrixa-lsp" ]; then
        $SUDO cp target/release/astrixa-lsp "$INSTALL_DIR/astrixa-lsp"
        $SUDO chmod +x "$INSTALL_DIR/astrixa-lsp"
        success "Installed astrixa-lsp to $INSTALL_DIR"
    else
        warning "LSP build failed (optional)"
    fi
}

# Verify installation
verify_installation() {
    info "Verifying installation..."
    
    if command -v astrixa >/dev/null 2>&1; then
        VERSION=$(astrixa --version 2>/dev/null || echo "unknown")
        success "ASTRIXA installed successfully!"
        echo
        echo "  Version: $VERSION"
        echo "  Path: $(which astrixa)"
        echo
    else
        error "Installation failed. 'astrixa' command not found in PATH"
    fi
}

# Setup shell completion (optional)
setup_completion() {
    info "Setting up shell completion..."
    
    # TODO: Add completion scripts for bash/zsh/fish
    # For now, skip
}

# Cleanup temporary files
cleanup() {
    if [ -d "$TMP_DIR" ]; then
        rm -rf "$TMP_DIR"
    fi
}

# Display post-install message
post_install() {
    echo
    echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${GREEN}  ASTRIXA installed successfully!${NC}"
    echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo
    echo "  Quick start:"
    echo
    echo "    # Create your first program"
    echo "    echo 'print(\"Hello, ASTRIXA!\")' > hello.ax"
    echo "    astrixa hello.ax"
    echo
    echo "    # Initialize a new project"
    echo "    astrixa init my-project"
    echo "    cd my-project"
    echo
    echo "  Documentation:"
    echo "    https://github.com/$REPO/blob/main/docs/intro.md"
    echo
    echo "  Need help?"
    echo "    https://github.com/$REPO/discussions"
    echo
    echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo
}

# Main installation flow
main() {
    echo
    echo "╔════════════════════════════════════════════════╗"
    echo "║                                                ║"
    echo "║        ASTRIXA Language Installer              ║"
    echo "║        Web, Web3, AI - One Language            ║"
    echo "║                                                ║"
    echo "╚════════════════════════════════════════════════╝"
    echo
    
    detect_platform
    check_permissions
    download_astrixa
    install_binary
    install_lsp || true  # LSP is optional
    verify_installation
    cleanup
    post_install
}

# Run installer
trap cleanup EXIT
main "$@"
