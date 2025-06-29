use actix_web::{web, HttpResponse, Result};
use serde::{Deserialize, Serialize};
use crate::keygen::keygen_string;
use crate::encrypt::encrypt_string;
use crate::decrypt::decrypt_string;
use crate::utils::Parameters;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use log::info;

#[derive(Debug, Serialize, Deserialize)]
pub struct KeyPairResponse {
    pub key_id: String,
    pub public_key: String,
    pub secret_key: String,
    pub created_at: DateTime<Utc>,
    pub parameters: ParametersInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParametersInfo {
    pub n: usize,
    pub q: i64,
    pub t: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EncryptRequest {
    pub public_key: String,
    pub message: String,
    pub parameters: Option<CustomParameters>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomParameters {
    pub n: Option<usize>,
    pub q: Option<i64>,
    pub t: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EncryptResponse {
    pub ciphertext: String,
    pub encrypted_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DecryptRequest {
    pub secret_key: String,
    pub ciphertext: String,
    pub parameters: Option<CustomParameters>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DecryptResponse {
    pub plaintext: String,
    pub decrypted_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HealthResponse {
    pub status: String,
    pub timestamp: DateTime<Utc>,
    pub version: String,
}

/// Generate a new key pair
async fn generate_keys(req: web::Json<Option<CustomParameters>>) -> Result<HttpResponse> {
    let custom_params = req.into_inner();
    let mut params = Parameters::default();
    
    if let Some(custom) = custom_params {
        if let Some(n) = custom.n {
            params.n = n;
        }
        if let Some(q) = custom.q {
            params.q = q;
        }
        if let Some(t) = custom.t {
            params.t = t;
        }
        // Update polynomial modulus
        let mut poly_vec = vec![0i64; params.n + 1];
        poly_vec[0] = 1;
        poly_vec[params.n] = 1;
        params.f = polynomial_ring::Polynomial::new(poly_vec);
    }

    // Generate keys (these functions return values directly, not Results)
    let keys = keygen_string(&params, None);
    let key_id = Uuid::new_v4().to_string();
    
    let response = KeyPairResponse {
        key_id: key_id.clone(),
        public_key: keys.get("public").unwrap().clone(),
        secret_key: keys.get("secret").unwrap().clone(),
        created_at: Utc::now(),
        parameters: ParametersInfo {
            n: params.n,
            q: params.q,
            t: params.t,
        },
    };
    
    info!("Generated new key pair: {}", key_id);
    Ok(HttpResponse::Ok().json(response))
}

/// Encrypt a message
async fn encrypt_message(req: web::Json<EncryptRequest>) -> Result<HttpResponse> {
    let req_data = req.into_inner();
    let mut params = Parameters::default();
    
    if let Some(custom) = req_data.parameters {
        if let Some(n) = custom.n {
            params.n = n;
        }
        if let Some(q) = custom.q {
            params.q = q;
        }
        if let Some(t) = custom.t {
            params.t = t;
        }
        // Update polynomial modulus
        let mut poly_vec = vec![0i64; params.n + 1];
        poly_vec[0] = 1;
        poly_vec[params.n] = 1;
        params.f = polynomial_ring::Polynomial::new(poly_vec);
    }

    // Encrypt message (function returns value directly, not Result)
    let ciphertext = encrypt_string(&req_data.public_key, &req_data.message, &params, None);
    
    let response = EncryptResponse {
        ciphertext,
        encrypted_at: Utc::now(),
    };
    
    info!("Message encrypted successfully");
    Ok(HttpResponse::Ok().json(response))
}

/// Decrypt a message
async fn decrypt_message(req: web::Json<DecryptRequest>) -> Result<HttpResponse> {
    let req_data = req.into_inner();
    let mut params = Parameters::default();
    
    if let Some(custom) = req_data.parameters {
        if let Some(n) = custom.n {
            params.n = n;
        }
        if let Some(q) = custom.q {
            params.q = q;
        }
        if let Some(t) = custom.t {
            params.t = t;
        }
        // Update polynomial modulus
        let mut poly_vec = vec![0i64; params.n + 1];
        poly_vec[0] = 1;
        poly_vec[params.n] = 1;
        params.f = polynomial_ring::Polynomial::new(poly_vec);
    }

    // Decrypt message (function returns value directly, not Result)
    let plaintext = decrypt_string(&req_data.secret_key, &req_data.ciphertext, &params);
    
    let response = DecryptResponse {
        plaintext,
        decrypted_at: Utc::now(),
    };
    
    info!("Message decrypted successfully");
    Ok(HttpResponse::Ok().json(response))
}

/// Health check endpoint
async fn health_check() -> Result<HttpResponse> {
    let response = HealthResponse {
        status: "healthy".to_string(),
        timestamp: Utc::now(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    };
    Ok(HttpResponse::Ok().json(response))
}

/// Configure the API routes
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .route("/keys", web::post().to(generate_keys))
            .route("/encrypt", web::post().to(encrypt_message))
            .route("/decrypt", web::post().to(decrypt_message))
            .route("/health", web::get().to(health_check))
    );
} 