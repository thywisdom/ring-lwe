# Ring-LWE Security Module Deployment Script
# This script helps you deploy your ring-LWE security module for free

Write-Host "🚀 Ring-LWE Security Module Deployment Script" -ForegroundColor Green
Write-Host "================================================" -ForegroundColor Green

# Check if git is installed
try {
    git --version | Out-Null
    Write-Host "✅ Git is installed" -ForegroundColor Green
} catch {
    Write-Host "❌ Git is not installed. Please install Git first." -ForegroundColor Red
    Write-Host "Download from: https://git-scm.com/download/win" -ForegroundColor Yellow
    exit 1
}

# Check if we're in the right directory
if (-not (Test-Path "Cargo.toml")) {
    Write-Host "❌ Please run this script from the ring-lwe project directory" -ForegroundColor Red
    exit 1
}

Write-Host "📋 Prerequisites Check:" -ForegroundColor Cyan
Write-Host "1. GitHub Account (free) - https://github.com" -ForegroundColor White
Write-Host "2. Railway Account (free) - https://railway.app" -ForegroundColor White
Write-Host "3. Your ring-lwe project (ready)" -ForegroundColor White

Write-Host "`n🎯 Deployment Options:" -ForegroundColor Cyan
Write-Host "1. Railway (Recommended - Easiest)" -ForegroundColor White
Write-Host "2. Render (Alternative)" -ForegroundColor White
Write-Host "3. Fly.io (Advanced)" -ForegroundColor White

$choice = Read-Host "`nSelect deployment option (1-3)"

switch ($choice) {
    "1" {
        Write-Host "`n🚂 Deploying to Railway..." -ForegroundColor Green
        Write-Host "`nStep 1: Initialize Git repository" -ForegroundColor Yellow
        
        # Initialize git if not already done
        if (-not (Test-Path ".git")) {
            git init
            Write-Host "✅ Git repository initialized" -ForegroundColor Green
        }
        
        # Add all files
        git add .
        git commit -m "Initial commit: Ring-LWE Security Module"
        Write-Host "✅ Files committed to git" -ForegroundColor Green
        
        # Get GitHub repository URL
        $repoUrl = Read-Host "`nEnter your GitHub repository URL (e.g., https://github.com/username/ring-lwe)"
        
        # Set remote origin
        git remote add origin $repoUrl
        git branch -M main
        git push -u origin main
        
        Write-Host "`n✅ Code pushed to GitHub!" -ForegroundColor Green
        Write-Host "`nStep 2: Deploy on Railway" -ForegroundColor Yellow
        Write-Host "1. Go to https://railway.app" -ForegroundColor White
        Write-Host "2. Sign up with GitHub" -ForegroundColor White
        Write-Host "3. Click 'New Project'" -ForegroundColor White
        Write-Host "4. Select 'Deploy from GitHub repo'" -ForegroundColor White
        Write-Host "5. Choose your ring-lwe repository" -ForegroundColor White
        Write-Host "6. Railway will automatically detect it's a Rust project" -ForegroundColor White
        
        Write-Host "`nStep 3: Configure Environment Variables" -ForegroundColor Yellow
        Write-Host "In Railway dashboard, add these variables:" -ForegroundColor White
        Write-Host "HOST=0.0.0.0" -ForegroundColor Cyan
        Write-Host "PORT=8080" -ForegroundColor Cyan
        Write-Host "RUST_LOG=info" -ForegroundColor Cyan
        
        Write-Host "`nStep 4: Test Your API" -ForegroundColor Yellow
        Write-Host "Once deployed, test with:" -ForegroundColor White
        Write-Host "curl https://your-app-name.railway.app/api/v1/health" -ForegroundColor Cyan
    }
    
    "2" {
        Write-Host "`n🎨 Deploying to Render..." -ForegroundColor Green
        Write-Host "`nStep 1: Push to GitHub (same as Railway)" -ForegroundColor Yellow
        
        if (-not (Test-Path ".git")) {
            git init
        }
        git add .
        git commit -m "Initial commit: Ring-LWE Security Module"
        
        $repoUrl = Read-Host "`nEnter your GitHub repository URL"
        git remote add origin $repoUrl
        git branch -M main
        git push -u origin main
        
        Write-Host "`nStep 2: Deploy on Render" -ForegroundColor Yellow
        Write-Host "1. Go to https://render.com" -ForegroundColor White
        Write-Host "2. Sign up with GitHub" -ForegroundColor White
        Write-Host "3. Click 'New +' - 'Web Service'" -ForegroundColor White
        Write-Host "4. Connect your GitHub account" -ForegroundColor White
        Write-Host "5. Select your ring-lwe repository" -ForegroundColor White
        Write-Host "6. Configure:" -ForegroundColor White
        Write-Host "   - Name: ring-lwe-security" -ForegroundColor Cyan
        Write-Host "   - Environment: Rust" -ForegroundColor Cyan
        Write-Host "   - Build Command: cargo build --release --bin ring-lwe-server" -ForegroundColor Cyan
        Write-Host "   - Start Command: ./target/release/ring-lwe-server" -ForegroundColor Cyan
    }
    
    "3" {
        Write-Host "`n✈️ Deploying to Fly.io..." -ForegroundColor Green
        Write-Host "`nStep 1: Install Fly CLI" -ForegroundColor Yellow
        Write-Host "Run this command:" -ForegroundColor White
        Write-Host "iwr https://fly.io/install.ps1 -useb | iex" -ForegroundColor Cyan
        
        Write-Host "`nStep 2: Deploy" -ForegroundColor Yellow
        Write-Host "Run these commands:" -ForegroundColor White
        Write-Host "fly auth signup" -ForegroundColor Cyan
        Write-Host "fly launch" -ForegroundColor Cyan
        Write-Host "fly deploy" -ForegroundColor Cyan
    }
    
    default {
        Write-Host "❌ Invalid choice. Please select 1, 2, or 3." -ForegroundColor Red
        exit 1
    }
}

Write-Host "`n🎉 Deployment instructions completed!" -ForegroundColor Green
Write-Host "`n📚 Next Steps:" -ForegroundColor Cyan
Write-Host "1. Follow the deployment steps above" -ForegroundColor White
Write-Host "2. Test your API endpoints" -ForegroundColor White
Write-Host "3. Integrate with your Qmail application" -ForegroundColor White
Write-Host "4. Check DEPLOYMENT_GUIDE.md for detailed instructions" -ForegroundColor White

Write-Host "`n🔗 Useful Links:" -ForegroundColor Cyan
Write-Host "• Railway: https://railway.app" -ForegroundColor White
Write-Host "• Render: https://render.com" -ForegroundColor White
Write-Host "• Fly.io: https://fly.io" -ForegroundColor White
Write-Host "• GitHub: https://github.com" -ForegroundColor White

Read-Host "`nPress Enter to exit" 