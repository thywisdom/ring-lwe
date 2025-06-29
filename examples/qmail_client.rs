use reqwest;
use serde_json::{json, Value};
use std::collections::HashMap;

pub struct RingLweClient {
    base_url: String,
    client: reqwest::Client,
}

impl RingLweClient {
    pub fn new(base_url: String) -> Self {
        Self {
            base_url,
            client: reqwest::Client::new(),
        }
    }

    /// Generate a new key pair
    pub async fn generate_keys(&self, parameters: Option<HashMap<String, Value>>) -> Result<Value, Box<dyn std::error::Error>> {
        let url = format!("{}/api/v1/keys", self.base_url);
        
        let response = self.client
            .post(&url)
            .json(&parameters.unwrap_or_default())
            .send()
            .await?;
        
        let result: Value = response.json().await?;
        Ok(result)
    }

    /// Encrypt a message
    pub async fn encrypt_message(&self, public_key: &str, message: &str, parameters: Option<HashMap<String, Value>>) -> Result<Value, Box<dyn std::error::Error>> {
        let url = format!("{}/api/v1/encrypt", self.base_url);
        
        let request_body = json!({
            "public_key": public_key,
            "message": message,
            "parameters": parameters
        });
        
        let response = self.client
            .post(&url)
            .json(&request_body)
            .send()
            .await?;
        
        let result: Value = response.json().await?;
        Ok(result)
    }

    /// Decrypt a message
    pub async fn decrypt_message(&self, secret_key: &str, ciphertext: &str, parameters: Option<HashMap<String, Value>>) -> Result<Value, Box<dyn std::error::Error>> {
        let url = format!("{}/api/v1/decrypt", self.base_url);
        
        let request_body = json!({
            "secret_key": secret_key,
            "ciphertext": ciphertext,
            "parameters": parameters
        });
        
        let response = self.client
            .post(&url)
            .json(&request_body)
            .send()
            .await?;
        
        let result: Value = response.json().await?;
        Ok(result)
    }

    /// Check service health
    pub async fn health_check(&self) -> Result<Value, Box<dyn std::error::Error>> {
        let url = format!("{}/api/v1/health", self.base_url);
        
        let response = self.client
            .get(&url)
            .send()
            .await?;
        
        let result: Value = response.json().await?;
        Ok(result)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the client
    let client = RingLweClient::new("http://localhost:8080".to_string());
    
    println!("🔐 Ring-LWE Security Module Client Example");
    println!("===========================================");
    
    // Health check
    println!("\n1. Checking service health...");
    match client.health_check().await {
        Ok(health) => println!("✅ Health check: {:?}", health),
        Err(e) => println!("❌ Health check failed: {}", e),
    }
    
    // Generate keys
    println!("\n2. Generating key pair...");
    match client.generate_keys(None).await {
        Ok(keys) => {
            println!("✅ Keys generated successfully!");
            let public_key = keys["public_key"].as_str().unwrap();
            let secret_key = keys["secret_key"].as_str().unwrap();
            
            // Encrypt a message
            println!("\n3. Encrypting message...");
            let message = "Hello from Qmail! This is a secure message.";
            match client.encrypt_message(public_key, message, None).await {
                Ok(encrypted) => {
                    println!("✅ Message encrypted successfully!");
                    let ciphertext = encrypted["ciphertext"].as_str().unwrap();
                    
                    // Decrypt the message
                    println!("\n4. Decrypting message...");
                    match client.decrypt_message(secret_key, ciphertext, None).await {
                        Ok(decrypted) => {
                            println!("✅ Message decrypted successfully!");
                            let plaintext = decrypted["plaintext"].as_str().unwrap();
                            println!("📧 Original message: {}", message);
                            println!("🔓 Decrypted message: {}", plaintext);
                        },
                        Err(e) => println!("❌ Decryption failed: {}", e),
                    }
                },
                Err(e) => println!("❌ Encryption failed: {}", e),
            }
        },
        Err(e) => println!("❌ Key generation failed: {}", e),
    }
    
    Ok(())
} 