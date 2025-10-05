use dink_server::{Logger, RouterOptions, start_default_server_with_options};
use std::env;

/// Custom logger implementation for the API
#[derive(Clone)]
pub struct ApiLogger {
    pub service_name: String,
}

impl ApiLogger {
    pub fn new(service_name: String) -> Self {
        Self { service_name }
    }
}

impl Logger for ApiLogger {
    fn info(&self, message: &str) {
        println!("🔔 [{}] INFO: {}", self.service_name, message);
    }

    fn warn(&self, message: &str) {
        eprintln!("⚠️ [{}] WARN: {}", self.service_name, message);
    }

    fn error(&self, message: &str) {
        eprintln!("❌ [{}] ERROR: {}", self.service_name, message);
    }

    fn debug(&self, message: &str) {
        println!("🐛 [{}] DEBUG: {}", self.service_name, message);
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get port from environment variable or use default
    let port = env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse::<u16>()
        .unwrap_or(3000);

    // Get notification route from environment variable or use root route
    let notify_route = env::var("NOTIFY_ROUTE").unwrap_or_else(|_| "/".to_string());

    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], port));

    println!("🚀 Starting Dink API Server on {}", addr);
    println!(
        "📡 Health check: http://{}:{}/health",
        addr.ip(),
        addr.port()
    );
    println!(
        "📨 Notifications: http://{}:{}{}",
        addr.ip(),
        addr.port(),
        notify_route
    );

    // Create custom logger and options
    let logger = ApiLogger::new("Dink-API".to_string());
    let options = RouterOptions::new(logger).with_route(&notify_route);

    // Start server with options
    start_default_server_with_options(options).await?;

    Ok(())
}
