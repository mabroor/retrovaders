#!/bin/bash

# RetroVaders - Session Initialization Script
# Run this at the start of every development session

set -e

echo "🚀 RetroVaders Session Initialization"
echo "======================================"
echo ""

# 1. Confirm working directory
echo "📂 Working Directory:"
pwd
echo ""

# 2. Show recent progress
echo "📋 Recent Progress (last 20 lines):"
echo "------------------------------------"
if [ -f "claude-progress.txt" ]; then
    tail -20 claude-progress.txt
else
    echo "No progress file found. Creating..."
    touch claude-progress.txt
fi
echo ""

# 3. Check feature status
echo "📊 Feature Status:"
echo "------------------"
if [ -f "features.json" ] && command -v jq &> /dev/null; then
    echo "Pending:     $(cat features.json | jq '[.features[] | select(.status == "pending")] | length')"
    echo "In Progress: $(cat features.json | jq '[.features[] | select(.status == "in_progress")] | length')"
    echo "Completed:   $(cat features.json | jq '[.features[] | select(.status == "completed")] | length')"
    echo "Blocked:     $(cat features.json | jq '[.features[] | select(.status == "blocked")] | length')"
    echo ""
    echo "Next Features to Work On:"
    cat features.json | jq -r '.features[] | select(.status == "pending") | "  - \(.id): \(.name)"' | head -5
else
    echo "features.json not found or jq not installed"
fi
echo ""

# 4. Run tests if project exists
echo "🧪 Test Status:"
echo "---------------"
if [ -f "Cargo.toml" ]; then
    cargo test --lib 2>&1 | tail -10 || echo "Tests not yet configured"
else
    echo "Cargo.toml not found - project not yet initialized"
fi
echo ""

# 5. Git status
echo "📝 Git Status:"
echo "--------------"
if [ -d ".git" ]; then
    git status --short
    echo ""
    echo "Recent Commits:"
    git log --oneline -5 2>/dev/null || echo "No commits yet"
else
    echo "Git not initialized"
fi
echo ""

# 6. Clippy check (if project exists)
if [ -f "Cargo.toml" ]; then
    echo "🔍 Clippy Check:"
    echo "----------------"
    cargo clippy 2>&1 | tail -5 || echo "Clippy check skipped"
    echo ""
fi

echo "======================================"
echo "✅ Session initialization complete!"
echo ""
echo "Remember:"
echo "  1. Read RALPH_PROMPT.md for development guidelines"
echo "  2. Use subagents for research before implementing"
echo "  3. Focus on 1-3 features per session"
echo "  4. Commit after each completed feature"
echo "  5. Update claude-progress.txt at session end"
echo ""
