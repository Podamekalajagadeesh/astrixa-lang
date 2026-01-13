#!/bin/bash

# Astrixa Lang Project Cleanup Script
# Removes unnecessary development artifacts and redundant documentation

echo "🧹 Starting Astrixa Lang Project Cleanup..."
echo ""

# Navigate to project root
cd /workspaces/astrixa-lang

# Create backup directory for safety
mkdir -p .cleanup_backup
TIMESTAMP=$(date +%Y%m%d_%H%M%S)

echo "📦 Creating backup at .cleanup_backup/backup_$TIMESTAMP/"
tar -czf ".cleanup_backup/backup_$TIMESTAMP.tar.gz" . --exclude=.cleanup_backup --exclude=.git --exclude=target --exclude=node_modules 2>/dev/null

echo ""
echo "🗑️  Deleting unnecessary files..."
echo ""

# Counter
deleted=0

# 1. Delete ALL STEP_XX documentation files
echo "Deleting STEP_XX documentation files..."
for file in STEP_*.md; do
    if [ -f "$file" ]; then
        echo "  - $file"
        rm "$file"
        ((deleted++))
    fi
done

# 2. Delete redundant index files (keep README.md and DOCUMENTATION_INDEX.md)
echo ""
echo "Deleting redundant index files..."
for file in INDEX.md START_HERE.md; do
    if [ -f "$file" ]; then
        echo "  - $file"
        rm "$file"
        ((deleted++))
    fi
done

# 3. Delete "COMPLETE" documentation files
echo ""
echo "Deleting 'COMPLETE' documentation files..."
for file in COMPLETION_SUMMARY.md ASTRIXA_V0.1_COMPLETE.md RELEASE_COMPLETE.md \
            AI_COMPLETE_GUIDE.md WEB3_COMPLETE_GUIDE.md LSP_COMPLETE.md \
            PACKAGE_MANAGER_COMPLETE.md STDLIB_COMPLETE_REFERENCE.md \
            COMPILER_COMPLETE_STRUCTURE.md COMPILER_PIPELINE_COMPLETE.md \
            IDE_SUPPORT_COMPLETE.md AI_IMPLEMENTATION_SUMMARY.md \
            LSP_IMPLEMENTATION_SUMMARY.md; do
    if [ -f "$file" ]; then
        echo "  - $file"
        rm "$file"
        ((deleted++))
    fi
done

# 4. Move test files to tests/ directory
echo ""
echo "Moving test files to tests/ directory..."
mkdir -p tests
for file in *_test.ax test_*.ax; do
    if [ -f "$file" ]; then
        echo "  - $file -> tests/"
        mv "$file" tests/
    fi
done

# 5. Move example/demo files to examples/ directory
echo ""
echo "Moving demo files to examples/ directory..."
for file in main.ax math.ax defi_portfolio_demo.ax contract_with_ai_advanced.ax wallet_contract.ax; do
    if [ -f "$file" ]; then
        echo "  - $file -> examples/"
        mv "$file" examples/
    fi
done

# 6. Move scripts to scripts/ directory
echo ""
echo "Moving scripts to scripts/ directory..."
mkdir -p scripts
for file in build_wasm.sh install.sh test_modules.sh; do
    if [ -f "$file" ]; then
        echo "  - $file -> scripts/"
        mv "$file" scripts/
    fi
done

# 7. Move playground.html to examples/
echo ""
echo "Moving playground.html to examples/..."
if [ -f "playground.html" ]; then
    echo "  - playground.html -> examples/"
    mv playground.html examples/
fi

# 8. Delete corrupted compiler files
echo ""
echo "Deleting corrupted compiler files..."
cd compiler
for file in "Token::Else," "Token::Fn," "Token::If," "Token::Let," "break," "panic!(Condition must be boolean)," "{"; do
    if [ -f "$file" ]; then
        echo "  - compiler/$file"
        rm "$file"
        ((deleted++))
    fi
done
cd ..

# 9. Delete compiler STEP documentation
echo ""
echo "Deleting compiler STEP documentation..."
if [ -f "compiler/STEP_34_README.md" ]; then
    echo "  - compiler/STEP_34_README.md"
    rm "compiler/STEP_34_README.md"
    ((deleted++))
fi

# 10. Clean up redundant guides
echo ""
echo "Deleting redundant guide files..."
for file in LAUNCH_GUIDE.md COMPILER_TEST_GUIDE.md; do
    if [ -f "$file" ]; then
        echo "  - $file"
        rm "$file"
        ((deleted++))
    fi
done

echo ""
echo "✅ Cleanup complete!"
echo ""
echo "📊 Summary:"
echo "  - Files deleted: $deleted"
echo "  - Files moved to tests/: $(ls tests/*.ax 2>/dev/null | wc -l)"
echo "  - Files moved to examples/: $(find examples -name "*.ax" -o -name "*.html" 2>/dev/null | wc -l)"
echo "  - Files moved to scripts/: $(ls scripts/*.sh 2>/dev/null | wc -l)"
echo ""
echo "💾 Backup saved at: .cleanup_backup/backup_$TIMESTAMP.tar.gz"
echo ""
echo "🎉 Project structure cleaned!"
