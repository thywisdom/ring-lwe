# 🚀 Free Hosting Deployment Guide

This guide will help you deploy your ring-LWE Security Module for free and integrate it with your Qmail application.

## 📋 Prerequisites

1. **GitHub Account** (free)
2. **Railway Account** (free tier available)
3. **Your ring-lwe project** (already ready)

## 🎯 Option 1: Railway (Recommended)

### Step 1: Prepare Your Repository

1. **Push your code to GitHub:**
```bash
git init
git add .
git commit -m "Initial commit: Ring-LWE Security Module"
git branch -M main
git remote add origin https://github.com/YOUR_USERNAME/ring-lwe.git
git push -u origin main
```

### Step 2: Deploy on Railway

1. **Go to [Railway.app](https://railway.app)**
2. **Sign up with GitHub**
3. **Click "New Project"**
4. **Select "Deploy from GitHub repo"**
5. **Choose your `ring-lwe` repository**
6. **Railway will automatically detect it's a Rust project**

### Step 3: Configure Environment Variables

In Railway dashboard:
1. Go to your project
2. Click "Variables" tab
3. Add these environment variables:
```
HOST=0.0.0.0
PORT=8080
RUST_LOG=info
```

### Step 4: Deploy

1. **Railway will automatically build and deploy**
2. **Wait for the build to complete** (usually 2-3 minutes)
3. **Your API will be available at:** `https://your-app-name.railway.app`

### Step 5: Test Your Deployed API

```bash
# Test health check
curl https://your-app-name.railway.app/api/v1/health

# Generate keys
curl -X POST https://your-app-name.railway.app/api/v1/keys \
  -H "Content-Type: application/json" \
  -d '{}'
```

---

## 🎯 Option 2: Render

### Step 1: Create Render Account

1. Go to [Render.com](https://render.com)
2. Sign up with GitHub
3. Click "New +" → "Web Service"

### Step 2: Connect Repository

1. Connect your GitHub account
2. Select your `ring-lwe` repository
3. Configure the service:
   - **Name:** `ring-lwe-security`
   - **Environment:** `Rust`
   - **Build Command:** `cargo build --release --bin ring-lwe-server`
   - **Start Command:** `./target/release/ring-lwe-server`

### Step 3: Set Environment Variables

Add these in Render dashboard:
```
HOST=0.0.0.0
PORT=10000
RUST_LOG=info
```

### Step 4: Deploy

1. Click "Create Web Service"
2. Wait for deployment (3-5 minutes)
3. Your API will be at: `https://your-app-name.onrender.com`

---

## 🎯 Option 3: Fly.io

### Step 1: Install Fly CLI

```bash
# Windows (PowerShell)
iwr https://fly.io/install.ps1 -useb | iex

# Or download from: https://fly.io/docs/hands-on/install-flyctl/
```

### Step 2: Create Fly App

```bash
fly auth signup
fly launch
```

### Step 3: Deploy

```bash
fly deploy
```

Your API will be at: `https://your-app-name.fly.dev`

---

## 🔧 Integration with Qmail Application

### Step 1: Update Your Qmail App Configuration

```javascript
// config.js
const RING_LWE_API_URL = 'https://your-app-name.railway.app';

// Or use environment variable
const RING_LWE_API_URL = process.env.RING_LWE_API_URL || 'https://your-app-name.railway.app';
```

### Step 2: Create API Client

```javascript
// ringLweClient.js
class RingLweClient {
  constructor(baseUrl) {
    this.baseUrl = baseUrl;
  }

  async generateKeys() {
    const response = await fetch(`${this.baseUrl}/api/v1/keys`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({})
    });
    return response.json();
  }

  async encryptMessage(publicKey, message) {
    const response = await fetch(`${this.baseUrl}/api/v1/encrypt`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ public_key: publicKey, message })
    });
    return response.json();
  }

  async decryptMessage(secretKey, ciphertext) {
    const response = await fetch(`${this.baseUrl}/api/v1/decrypt`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ secret_key: secretKey, ciphertext })
    });
    return response.json();
  }
}

module.exports = RingLweClient;
```

### Step 3: Use in Qmail App

```javascript
// emailService.js
const RingLweClient = require('./ringLweClient');

const securityClient = new RingLweClient(process.env.RING_LWE_API_URL);

// When user registers
async function registerUser(email, password) {
  // Generate encryption keys
  const keys = await securityClient.generateKeys();
  
  // Store user with keys
  const user = {
    email,
    passwordHash: await hashPassword(password),
    publicKey: keys.public_key,
    secretKey: keys.secret_key, // Encrypt this before storing!
    keyId: keys.key_id
  };
  
  await saveUser(user);
}

// When sending email
async function sendEncryptedEmail(fromUser, toUser, message) {
  const recipient = await getUserByEmail(toUser);
  const encrypted = await securityClient.encryptMessage(recipient.publicKey, message);
  
  // Store encrypted email
  await saveEmail({
    from: fromUser,
    to: toUser,
    encryptedContent: encrypted.ciphertext,
    sentAt: new Date()
  });
}

// When reading email
async function readEncryptedEmail(userId, emailId) {
  const user = await getUserById(userId);
  const email = await getEmailById(emailId);
  
  const decrypted = await securityClient.decryptMessage(user.secretKey, email.encryptedContent);
  
  return {
    ...email,
    content: decrypted.plaintext
  };
}
```

---

## 🔒 Security Considerations

### 1. **HTTPS Only**
- All free hosting platforms provide HTTPS
- Always use HTTPS in production

### 2. **API Rate Limiting**
```javascript
// Add rate limiting to your Qmail app
const rateLimit = require('express-rate-limit');

const apiLimiter = rateLimit({
  windowMs: 15 * 60 * 1000, // 15 minutes
  max: 100 // limit each IP to 100 requests per windowMs
});

app.use('/api/', apiLimiter);
```

### 3. **Key Storage Security**
```javascript
// Encrypt secret keys before storing
const crypto = require('crypto');

function encryptSecretKey(secretKey, masterKey) {
  const cipher = crypto.createCipher('aes-256-cbc', masterKey);
  let encrypted = cipher.update(secretKey, 'utf8', 'hex');
  encrypted += cipher.final('hex');
  return encrypted;
}

function decryptSecretKey(encryptedKey, masterKey) {
  const decipher = crypto.createDecipher('aes-256-cbc', masterKey);
  let decrypted = decipher.update(encryptedKey, 'hex', 'utf8');
  decrypted += decipher.final('utf8');
  return decrypted;
}
```

---

## 📊 Monitoring

### 1. **Health Checks**
```bash
# Set up monitoring
curl https://your-app-name.railway.app/api/v1/health
```

### 2. **Logs**
- Railway: View logs in dashboard
- Render: View logs in dashboard
- Fly.io: `fly logs`

### 3. **Uptime Monitoring**
Use free services like:
- [UptimeRobot](https://uptimerobot.com) (free tier)
- [Pingdom](https://pingdom.com) (free tier)

---

## 🚨 Troubleshooting

### Common Issues:

1. **Build Fails**
   - Check Rust version compatibility
   - Ensure all dependencies are in Cargo.toml

2. **Port Issues**
   - Make sure HOST=0.0.0.0 and PORT is set correctly
   - Some platforms use different default ports

3. **Memory Issues**
   - Railway free tier: 512MB RAM
   - Render free tier: 512MB RAM
   - Consider optimizing if you hit limits

4. **Cold Starts**
   - Free tiers may have cold start delays
   - Consider upgrading to paid tier for production

---

## 🎉 Success!

Once deployed, your ring-LWE Security Module will be:
- ✅ **Free to host**
- ✅ **Automatically scaled**
- ✅ **HTTPS secured**
- ✅ **Ready for Qmail integration**

Your Qmail application can now use the deployed API endpoints for secure end-to-end encryption! 