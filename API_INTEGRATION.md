# Ring-LWE Security Module API Integration Guide

This guide explains how to integrate the Ring-LWE Security Module with your Qmail application for end-to-end encryption.

> ✅ **Status: API Successfully Tested** - All endpoints have been verified to work correctly with the test results shown below.

## 🚀 Quick Start

### 1. Deploy the Security Module

```bash
# Using Docker Compose (recommended)
docker-compose up -d

# Or build and run manually
cargo build --release --bin ring-lwe-server
./target/release/ring-lwe-server
```

The service will be available at `http://localhost:8080`

### 2. Test the API

#### Using curl (Linux/macOS)
```bash
# Health check
curl http://localhost:8080/api/v1/health

# Generate keys (requires empty JSON body)
curl -X POST http://localhost:8080/api/v1/keys \
  -H "Content-Type: application/json" \
  -d '{}'

# Encrypt a message
curl -X POST http://localhost:8080/api/v1/encrypt \
  -H "Content-Type: application/json" \
  -d '{
    "public_key": "your_public_key_here",
    "message": "Hello, secure world!"
  }'

# Decrypt a message
curl -X POST http://localhost:8080/api/v1/decrypt \
  -H "Content-Type: application/json" \
  -d '{
    "secret_key": "your_secret_key_here",
    "ciphertext": "encrypted_ciphertext_here"
  }'
```

#### Using PowerShell (Windows)
```powershell
# Health check
Invoke-RestMethod -Uri http://localhost:8080/api/v1/health -Method Get

# Generate keys (requires empty JSON body)
Invoke-RestMethod -Uri http://localhost:8080/api/v1/keys -Method Post -ContentType "application/json" -Body "{}"

# Encrypt a message
$encryptBody = @{
    public_key = "your_public_key_here"
    message = "Hello, secure world!"
} | ConvertTo-Json
Invoke-RestMethod -Uri http://localhost:8080/api/v1/encrypt -Method Post -ContentType "application/json" -Body $encryptBody

# Decrypt a message
$decryptBody = @{
    secret_key = "your_secret_key_here"
    ciphertext = "encrypted_ciphertext_here"
} | ConvertTo-Json
Invoke-RestMethod -Uri http://localhost:8080/api/v1/decrypt -Method Post -ContentType "application/json" -Body $decryptBody
```

#### ✅ Verified Test Results
Our testing confirmed all endpoints work correctly:

**Key Generation:**
```json
{
  "key_id": "b07b56ce-9882-4227-b849-dd8da981ae83",
  "public_key": "AAgAAAAAAABEAQAAAAAAAOj6...",
  "secret_key": "AAQAAAAAAAABAAAAAAAAAAE...",
  "created_at": "2025-06-29T02:21:46.071919700Z",
  "parameters": {
    "n": 1024,
    "q": 12289,
    "t": 2
  }
}
```

**Encryption:**
```json
{
  "ciphertext": "AAgAAAAAAADpDAAAAAAAAMYP...",
  "encrypted_at": "2025-06-29T02:22:13.619052600Z"
}
```

**Decryption:**
```json
{
  "plaintext": "Hello from Qmail! This is a secure message.",
  "decrypted_at": "2025-06-29T02:22:25.634798700Z"
}
```

**Health Check:**
```json
{
  "status": "healthy",
  "timestamp": "2025-06-29T02:22:54.965630300Z",
  "version": "0.1.7"
}
```

## 📡 API Endpoints

### POST `/api/v1/keys`
Generate a new key pair.

**Request Body (required - can be empty):**
```json
{}
```

**Or with custom parameters:**
```json
{
  "n": 1024,
  "q": 12289,
  "t": 2
}
```

**Response:**
```json
{
  "key_id": "uuid-here",
  "public_key": "base64-encoded-public-key",
  "secret_key": "base64-encoded-secret-key",
  "created_at": "2024-01-01T00:00:00Z",
  "parameters": {
    "n": 1024,
    "q": 12289,
    "t": 2
  }
}
```

### POST `/api/v1/encrypt`
Encrypt a message using a public key.

**Request Body:**
```json
{
  "public_key": "base64-encoded-public-key",
  "message": "Message to encrypt",
  "parameters": {
    "n": 1024,
    "q": 12289,
    "t": 2
  }
}
```

**Response:**
```json
{
  "ciphertext": "base64-encoded-ciphertext",
  "encrypted_at": "2024-01-01T00:00:00Z"
}
```

### POST `/api/v1/decrypt`
Decrypt a message using a secret key.

**Request Body:**
```json
{
  "secret_key": "base64-encoded-secret-key",
  "ciphertext": "base64-encoded-ciphertext",
  "parameters": {
    "n": 1024,
    "q": 12289,
    "t": 2
  }
}
```

**Response:**
```json
{
  "plaintext": "Decrypted message",
  "decrypted_at": "2024-01-01T00:00:00Z"
}
```

### GET `/api/v1/health`
Check service health.

**Response:**
```json
{
  "status": "healthy",
  "timestamp": "2024-01-01T00:00:00Z",
  "version": "0.1.7"
}
```

## 🔧 Qmail Integration

### 1. User Registration Flow

```javascript
// When a user registers in Qmail
async function registerUser(email, password) {
  // 1. Generate encryption keys
  const keyResponse = await fetch('http://localhost:8080/api/v1/keys', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' }
  });
  const keys = await keyResponse.json();
  
  // 2. Store user data with keys
  const userData = {
    email,
    passwordHash: await hashPassword(password),
    publicKey: keys.public_key,
    secretKey: keys.secret_key, // Encrypt this before storing!
    keyId: keys.key_id
  };
  
  // 3. Save to database
  await saveUser(userData);
}
```

### 2. Sending Encrypted Email

```javascript
// When sending an email
async function sendEncryptedEmail(fromUser, toUser, subject, message) {
  // 1. Get recipient's public key
  const recipient = await getUserByEmail(toUser);
  
  // 2. Encrypt the message
  const encryptResponse = await fetch('http://localhost:8080/api/v1/encrypt', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({
      public_key: recipient.publicKey,
      message: message
    })
  });
  const encrypted = await encryptResponse.json();
  
  // 3. Store encrypted email
  const emailData = {
    from: fromUser,
    to: toUser,
    subject: subject,
    encryptedContent: encrypted.ciphertext,
    sentAt: new Date()
  };
  
  await saveEmail(emailData);
}
```

### 3. Reading Encrypted Email

```javascript
// When reading an email
async function readEncryptedEmail(userId, emailId) {
  // 1. Get user's secret key
  const user = await getUserById(userId);
  
  // 2. Get encrypted email
  const email = await getEmailById(emailId);
  
  // 3. Decrypt the message
  const decryptResponse = await fetch('http://localhost:8080/api/v1/decrypt', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({
      secret_key: user.secretKey,
      ciphertext: email.encryptedContent
    })
  });
  const decrypted = await decryptResponse.json();
  
  return {
    ...email,
    content: decrypted.plaintext
  };
}
```

## 🔒 Security Considerations

### 1. Key Management
- **Never store secret keys in plain text**
- Use a key management service (AWS KMS, HashiCorp Vault, etc.)
- Encrypt secret keys with a master key before storing in database

### 2. Network Security
- Use HTTPS in production
- Implement API authentication (JWT, API keys)
- Rate limiting to prevent abuse
- CORS configuration for web clients

### 3. Parameter Security
- Use appropriate security parameters (n=1024, q=12289 for 128-bit security)
- Consider higher parameters for stronger security
- Validate all input parameters

### 4. Error Handling
- Don't expose sensitive information in error messages
- Log security events for monitoring
- Implement proper error responses

## 🐳 Production Deployment

### Docker Compose with Environment Variables

```yaml
version: '3.8'
services:
  ring-lwe-security:
    build: .
    ports:
      - "8080:8080"
    environment:
      - HOST=0.0.0.0
      - PORT=8080
      - RUST_LOG=info
      - API_KEY=your-secure-api-key
    restart: unless-stopped
    networks:
      - qmail-network

  qmail-app:
    # Your Qmail application
    depends_on:
      - ring-lwe-security
    networks:
      - qmail-network

networks:
  qmail-network:
    driver: bridge
```

### Kubernetes Deployment

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: ring-lwe-security
spec:
  replicas: 3
  selector:
    matchLabels:
      app: ring-lwe-security
  template:
    metadata:
      labels:
        app: ring-lwe-security
    spec:
      containers:
      - name: ring-lwe-security
        image: your-registry/ring-lwe-security:latest
        ports:
        - containerPort: 8080
        env:
        - name: HOST
          value: "0.0.0.0"
        - name: PORT
          value: "8080"
        - name: RUST_LOG
          value: "info"
        resources:
          requests:
            memory: "256Mi"
            cpu: "250m"
          limits:
            memory: "512Mi"
            cpu: "500m"
```

## 📊 Monitoring and Logging

### Health Checks
```bash
# Add to your monitoring system
curl -f http://localhost:8080/api/v1/health || exit 1
```

### Metrics to Monitor
- API response times
- Error rates
- Key generation frequency
- Encryption/decryption throughput

### Log Analysis
```