#!/bin/bash
# Pre-push validation script for portfolio
# Run this before pushing to catch syntax errors early

set -e

echo "Running pre-push validation..."

# Frontend check
echo "Checking frontend syntax..."
cd frontend
if command -v cargo >/dev/null 2>&1; then
    cargo check 2>&1 | head -50 || {
        echo "ERROR: Frontend cargo check failed"
        exit 1
    }
    echo "✓ Frontend syntax OK"
else
    echo "⚠ cargo not available (expected outside container), skipping frontend check"
fi

# Backend check
cd ../backend
if command -v cargo >/dev/null 2>&1; then
    cargo check 2>&1 | head -50 || {
        echo "ERROR: Backend cargo check failed"
        exit 1
    }
    echo "✓ Backend syntax OK"
else
    echo "⚠ cargo not available (expected outside container), skipping backend check"
fi

cd ..
echo "Pre-push validation complete"
