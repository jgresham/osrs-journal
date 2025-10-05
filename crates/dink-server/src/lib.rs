use axum::{
    Router,
    extract::Multipart,
    http::StatusCode,
    response::Json,
    routing::{get, post},
};
use serde_json::{Value, json};
use std::net::SocketAddr;
use tower::ServiceBuilder;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
use tracing::info;

/// Generic logger trait for general-purpose logging
pub trait Logger: Send + Sync + Clone {
    fn info(&self, message: &str);
    fn warn(&self, message: &str);
    fn error(&self, message: &str);
    fn debug(&self, message: &str);
}

/// Default logger implementation using tracing
#[derive(Clone)]
pub struct DefaultLogger;

impl Logger for DefaultLogger {
    fn info(&self, message: &str) {
        tracing::info!("{}", message);
    }

    fn warn(&self, message: &str) {
        tracing::warn!("{}", message);
    }

    fn error(&self, message: &str) {
        tracing::error!("{}", message);
    }

    fn debug(&self, message: &str) {
        tracing::debug!("{}", message);
    }
}

/// No-op logger for when logging is disabled
#[derive(Clone)]
pub struct NoOpLogger;

impl Logger for NoOpLogger {
    fn info(&self, _message: &str) {}
    fn warn(&self, _message: &str) {}
    fn error(&self, _message: &str) {}
    fn debug(&self, _message: &str) {}
}

/// Console logger that prints to stdout/stderr
#[derive(Clone)]
pub struct ConsoleLogger {
    pub prefix: String,
}

impl ConsoleLogger {
    pub fn new(prefix: &str) -> Self {
        Self {
            prefix: prefix.to_string(),
        }
    }
}

impl Logger for ConsoleLogger {
    fn info(&self, message: &str) {
        println!("[INFO] {}: {}", self.prefix, message);
    }

    fn warn(&self, message: &str) {
        eprintln!("[WARN] {}: {}", self.prefix, message);
    }

    fn error(&self, message: &str) {
        eprintln!("[ERROR] {}: {}", self.prefix, message);
    }

    fn debug(&self, message: &str) {
        println!("[DEBUG] {}: {}", self.prefix, message);
    }
}

/// Options for configuring the router
#[derive(Clone)]
pub struct RouterOptions<L: Logger + 'static> {
    /// The route path for handling notifications (default: "/notify")
    pub notify_route: String,
    /// The logger to use for general logging (default: DefaultLogger)
    pub logger: L,
}

impl<L: Logger + 'static> RouterOptions<L> {
    /// Create new router options with custom logger
    pub fn new(logger: L) -> Self {
        Self {
            notify_route: "/notify".to_string(),
            logger,
        }
    }

    /// Set a custom notification route
    pub fn with_route(mut self, route: &str) -> Self {
        self.notify_route = route.to_string();
        self
    }
}

impl RouterOptions<DefaultLogger> {
    /// Create router options with default logger
    pub fn default() -> Self {
        Self {
            notify_route: "/notify".to_string(),
            logger: DefaultLogger,
        }
    }
}

/// Health check endpoint
async fn health() -> Json<Value> {
    Json(json!({
        "status": "healthy",
        "service": "dink-server"
    }))
}

/// Handle Dink Plugin notifications
async fn handle_notification<L: Logger>(
    mut multipart: Multipart,
    logger: L,
) -> Result<Json<Value>, StatusCode> {
    let mut payload_json = None;
    let mut file_data = None;

    // Parse multipart form data
    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?
    {
        match field.name() {
            Some("payload_json") => {
                let data = field.bytes().await.map_err(|_| StatusCode::BAD_REQUEST)?;
                payload_json =
                    Some(String::from_utf8(data.to_vec()).map_err(|_| StatusCode::BAD_REQUEST)?);
            }
            Some("file") => {
                let data = field.bytes().await.map_err(|_| StatusCode::BAD_REQUEST)?;
                file_data = Some(data.to_vec());
            }
            _ => {
                // Ignore unknown fields
            }
        }
    }

    // Parse the JSON payload
    let payload: Value = match payload_json {
        Some(json_str) => serde_json::from_str(&json_str).map_err(|_| StatusCode::BAD_REQUEST)?,
        None => return Err(StatusCode::BAD_REQUEST),
    };

    // Extract notification data
    let notification_type = payload
        .get("type")
        .and_then(|v| v.as_str())
        .ok_or(StatusCode::BAD_REQUEST)?;

    let player_name = payload
        .get("playerName")
        .and_then(|v| v.as_str())
        .unwrap_or("Unknown");

    let content = payload
        .get("content")
        .and_then(|v| v.as_str())
        .unwrap_or("");

    let dink_account_hash = payload
        .get("dinkAccountHash")
        .and_then(|v| v.as_str())
        .unwrap_or("");

    // Log the notification using the injected logger
    logger.info(&format!(
        "📨 NOTIFICATION: {} from {} - {} - (dinkAccountHash: {})",
        notification_type, player_name, content, dink_account_hash
    ));

    // If we have file data (screenshot), log that too
    let has_screenshot = file_data.is_some();
    if let Some(file) = &file_data {
        logger.info(&format!(
            "Received screenshot attachment ({} bytes)",
            file.len()
        ));
    }

    // Create a response
    Ok(Json(json!({
        "status": "success",
        "message": "Notification processed",
        "notification_type": notification_type,
        "player_name": player_name,
        "has_screenshot": has_screenshot
    })))
}

/// Create and configure the Axum router with default options
pub fn create_router() -> Router {
    create_router_with_options(RouterOptions::default())
}

/// Create and configure the Axum router with custom options
pub fn create_router_with_options<L: Logger + 'static>(options: RouterOptions<L>) -> Router {
    Router::new()
        .route("/health", get(health))
        .route(
            &options.notify_route,
            post(move |multipart| {
                let logger = options.logger.clone();
                async move { handle_notification(multipart, logger).await }
            }),
        )
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(
                    CorsLayer::new()
                        .allow_origin(Any)
                        .allow_methods(Any)
                        .allow_headers(Any),
                ),
        )
}

/// Start the server with default options
pub async fn start_server(addr: SocketAddr) -> Result<(), Box<dyn std::error::Error>> {
    start_server_with_options(addr, RouterOptions::default()).await
}

/// Start the server with custom options
pub async fn start_server_with_options<L: Logger + 'static>(
    addr: SocketAddr,
    options: RouterOptions<L>,
) -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    let app = create_router_with_options(options);

    info!("Starting dink-server on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

/// Start the server with default configuration (localhost:3000)
pub async fn start_default_server() -> Result<(), Box<dyn std::error::Error>> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    start_server(addr).await
}

/// Start the server with default configuration and custom options
pub async fn start_default_server_with_options<L: Logger + 'static>(
    options: RouterOptions<L>,
) -> Result<(), Box<dyn std::error::Error>> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    start_server_with_options(addr, options).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use tower::util::ServiceExt;

    #[tokio::test]
    async fn test_health_endpoint() {
        let app = create_router();
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/health")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }
}
