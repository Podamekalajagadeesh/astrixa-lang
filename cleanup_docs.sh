#!/bin/bash
# Cleanup unnecessary documentation files

echo "Cleaning up duplicate documentation files..."

# STEP_34 duplicates
rm -fv STEP_34_COMPLETION.md
rm -fv STEP_34_FINAL_SUMMARY.md
rm -fv STEP_34_MASTER_SUMMARY.md
rm -fv STEP_34_DOCUMENTATION_INDEX.md
rm -fv STEP_34_MAIN_INDEX.md
rm -fv STEP_34_DOCUMENTATION_OVERVIEW.md
rm -fv STEP_34_FILE_MANIFEST.md
rm -fv STEP_34_FINAL_VERIFICATION_DELIVERY.md
rm -fv STEP_34_VISUAL_SUMMARY.md
rm -fv STEP_34_COMPLETION_CHECKLIST.md
rm -fv STEP_34_VERIFICATION.md

# STEP_36 duplicates
rm -fv STEP_36_DELIVERY_COMPLETE.md
rm -fv STEP_36_DELIVERY_SUMMARY.md
rm -fv STEP_36_FINAL_SUMMARY.md
rm -fv STEP_36_COMPLETION_SUMMARY.md
rm -fv STEP_36_COMPLETION_README.md
rm -fv STEP_36_BANNER.md
rm -fv STEP_36_STATUS.md
rm -fv STEP_36_TRANSFORMATION.md
rm -fv STEP_36_TRANSFORMATION_VISUAL.md
rm -fv STEP_36_BEFORE_AFTER.md
rm -fv STEP_36_VISUAL_SUMMARY.md
rm -fv STEP_36_DOCUMENTATION_MAP.md
rm -fv STEP_36_CHECKLIST.md

# STEP_37 duplicates
rm -fv STEP_37_BANNER.md
rm -fv STEP_37_SUMMARY.md

# STEP_38 duplicates
rm -fv STEP_38_BANNER.md
rm -fv STEP_38_COMPLETION_STATUS.md
rm -fv STEP_38_DELIVERY_COMPLETE.md
rm -fv STEP_38_MASTER_INDEX.md
rm -fv STEP_38_START_HERE.md
rm -fv STEP_38_FINAL_VERIFICATION_CHECKLIST.md

# STEP_39 duplicates
rm -fv STEP_39_BANNER.md
rm -fv STEP_39_COMPLETION_STATUS.md
rm -fv STEP_39_DELIVERY_COMPLETE.md

# STEP_40 duplicates
rm -fv STEP_40_BANNER.md
rm -fv STEP_40_COMPLETION_STATUS.md

# Package Manager duplicates
rm -fv PACKAGE_MANAGER_SUMMARY.md
rm -fv PACKAGE_MANAGER_INDEX.md
rm -fv PACKAGE_MANAGER_IMPLEMENTATION.md
rm -fv PACKAGE_MANAGER_USAGE.md

# Other duplicates
rm -fv LSP_INDEX.md
rm -fv COMPLETION_REPORT.md
rm -fv GOVERNANCE_COMPLETE.md
rm -fv WEB_WEB3_IMPLEMENTATION_COMPLETE.md
rm -fv IMPLEMENTATION_CHECKLIST.md
rm -fv FILE_MANIFEST.md
rm -fv PUSH_INSTRUCTIONS.md
rm -fv COMMIT_MESSAGE.txt
rm -fv ECOSYSTEM_STRATEGY.md

echo ""
echo "✅ Cleanup complete!"
echo ""
echo "Remaining markdown files:"
find . -maxdepth 1 -name "*.md" -type f | wc -l
echo ""
echo "Remaining STEP files:"
ls -1 STEP_*.md 2>/dev/null | wc -l
