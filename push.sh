#!/bin/bash

# MCP Sentinel v1.5.0 - Push to GitHub Script
# This script pushes MCP Sentinel v1.5.0 to GitHub

set -e  # Exit on error

echo "🚀 MCP Sentinel v1.5.0 - GitHub Push Script"
echo "=========================================="
echo ""

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    echo "❌ Error: Not in MCP Sentinel directory!"
    echo "Please run this script from the MCP_Scanner directory"
    exit 1
fi

# Check git status
echo "📋 Checking git status..."
git status

echo ""
echo "📦 Repository Information:"
echo "   Remote: $(git remote get-url origin)"
echo "   Branch: $(git branch --show-current)"
echo "   Commits ahead: $(git rev-list --count origin/main..HEAD 2>/dev/null || echo 'N/A')"
echo ""

# Confirm push
read -p "🤔 Do you want to push to GitHub? (yes/no): " -r
echo
if [[ ! $REPLY =~ ^[Yy]es$ ]]; then
    echo "❌ Push cancelled"
    exit 0
fi

echo "🔄 Pushing main branch..."
if git push -u origin main; then
    echo "✅ Main branch pushed successfully!"
else
    echo "❌ Failed to push main branch"
    echo ""
    echo "💡 Troubleshooting:"
    echo "   1. Make sure you have GitHub access configured"
    echo "   2. Check if the repository exists: https://github.com/beejak/MCP_Sentinel"
    echo "   3. Verify your GitHub token has 'repo' permissions"
    echo ""
    echo "Alternative: Create the repo first on GitHub"
    echo "   Go to: https://github.com/new"
    echo "   Name: MCP_Sentinel"
    echo "   Then run: git push -u origin main"
    exit 1
fi

echo ""
echo "🏷️  Pushing v1.5.0 tag..."
if git push origin v1.5.0; then
    echo "✅ Tag v1.5.0 pushed successfully!"
else
    echo "⚠️  Warning: Failed to push tag (but main branch is pushed)"
fi

echo ""
echo "✨ Success! MCP Sentinel v1.5.0 is now on GitHub!"
echo ""
echo "📍 Repository URL: https://github.com/beejak/MCP_Sentinel"
echo ""
echo "📋 Next Steps:"
echo "   1. Visit: https://github.com/beejak/MCP_Sentinel"
echo "   2. Create a release from tag v1.5.0"
echo "   3. Add repository description and topics"
echo ""
echo "🎉 Congratulations! Your scanner is live!"
