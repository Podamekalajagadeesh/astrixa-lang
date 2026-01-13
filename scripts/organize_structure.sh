#!/bin/bash
# Astrixa Lang Project Structure Reorganization
# This script moves files to create an intentional, professional project structure

set -e

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$PROJECT_ROOT"

echo "🏗️  Reorganizing Astrixa Lang project structure..."
echo ""

# Move markdown documentation files to docs/
echo "📄 Moving markdown documentation to docs/..."
MD_FILES=(
    "AI_PRIMITIVES.md"
    "CHANGELOG.md"
    "CLI_REFERENCE.md"
    "CODE_OF_CONDUCT.md"
    "CONTRIBUTING.md"
    "DOCUMENTATION_INDEX.md"
    "GAS_MODEL.md"
    "GOVERNANCE.md"
    "LSP_QUICKSTART.md"
    "PACKAGE_MANAGER.md"
    "PACKAGE_MANAGER_TUTORIAL.md"
    "RELEASE_NOTES_v0.1.0.md"
    "ROADMAP.md"
    "SECURITY.md"
    "STDLIB_QUICKSTART.md"
)

for md_file in "${MD_FILES[@]}"; do
    if [ -f "$md_file" ]; then
        mv "$md_file" "docs/$md_file" && echo "  ✓ Moved $md_file"
    fi
done

# Move cleanup script to scripts/
echo ""
echo "🔧 Moving utility scripts to scripts/..."
if [ -f "cleanup_project.sh" ]; then
    mv "cleanup_project.sh" "scripts/" && echo "  ✓ Moved cleanup_project.sh"
fi

# Remove temporary backup directory
echo ""
echo "🗑️  Removing temporary files..."
if [ -d ".cleanup_backup" ]; then
    rm -rf ".cleanup_backup" && echo "  ✓ Removed .cleanup_backup/"
fi

echo ""
echo "✅ Project structure reorganization complete!"
echo ""
echo "📊 Final Structure:"
echo "   astrixa-lang/"
echo "   ├── docs/                 (all documentation)"
echo "   ├── scripts/              (build and utility scripts)"
echo "   ├── compiler/             (language compiler)"
echo "   ├── runtime/              (runtime/WASM)"
echo "   ├── examples/             (example programs)"
echo "   ├── tests/                (test files)"
echo "   ├── stdlib/               (standard library)"
echo "   ├── lsp/                  (language server)"
echo "   ├── astrixa-cli/          (CLI tools)"
echo "   ├── astrixa-vscode/       (VS Code extension)"
echo "   ├── rfcs/                 (RFCs)"
echo "   ├── design/               (design docs)"
echo "   ├── README.md             (main readme)"
echo "   ├── LICENSE               (license)"
echo "   └── .gitignore"
echo ""
echo "✨ Repository is now clean and intentional!"
