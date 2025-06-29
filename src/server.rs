use actix_web::{web, App, HttpServer, middleware::Logger};
use ring_lwe::api::configure_routes;
use std::env;
use dotenv::dotenv;
use log::info;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables
    dotenv().ok();
    
    // Initialize logging
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    
    // Get server configuration from environment
    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .expect("PORT must be a valid port number");
    
    info!("Starting Ring-LWE Security Module Server");
    info!("Server will be available at http://{}:{}", host, port);
    
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .configure(configure_routes)
            .service(
                web::scope("/")
                    .route("", web::get().to(|| async { 
                        "Ring-LWE Security Module API\n\nAvailable endpoints:\n- POST /api/v1/keys - Generate key pair\n- POST /api/v1/encrypt - Encrypt message\n- POST /api/v1/decrypt - Decrypt message\n- GET /api/v1/health - Health check"
                    }))
            )
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
} 