#!/bin/bash

# Quick Deployment Setup Script
# Run this before final deployment

echo "=== Portfolio Deployment Checklist ==="
echo ""

# 1. Frontend
echo "✓ FRONTEND CHECKS"
cd /home/rustyradon/PORT1
npm run build && echo "  ✓ Frontend builds successfully" || echo "  ✗ Frontend build failed"
npm run lint && echo "  ✓ Lint passes" || echo "  ✗ Lint has errors"
echo ""

# 2. Backend
echo "✓ BACKEND CHECKS"
cd port-back
cargo build --release && echo "  ✓ Backend builds successfully" || echo "  ✗ Backend build failed"
echo ""

# 3. Git
echo "✓ GIT CHECKS"
cd /home/rustyradon/PORT1
git status
echo ""

echo "=== NEXT STEPS ==="
echo ""
echo "1. VERCEL (Frontend):"
echo "   - Go to https://vercel.com"
echo "   - Import this GitHub repo"
echo "   - Set VITE_API_URL environment variable"
echo ""
echo "2. MONGODB ATLAS:"
echo "   - Go to https://mongodb.com/cloud"
echo "   - Create free cluster"
echo "   - Copy connection string"
echo ""
echo "3. RENDER (Backend):"
echo "   - Go to https://render.com"
echo "   - Create Web Service from GitHub"
echo "   - Set MONGODB_URI environment variable"
echo "   - Note your backend URL (e.g., https://portfolio-backend.onrender.com)"
echo ""
echo "4. UPDATE VERCEL:"
echo "   - Set VITE_API_URL to your Render backend URL"
echo "   - Redeploy frontend"
echo ""
echo "=== DONE ==="
