use author_axum::{Session, SessionConfig, SessionManagerLayer};
use axum::debug_handler;
use axum::routing::get;
use axum::Router;
use std::net::{Ipv4Addr, SocketAddr};
use tracing::debug;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialise tracing
    tracing_subscriber::fmt::init();

    // Create the session config
    let session_config = SessionConfig::default();

    // Build our application
    let app = Router::new().route("/", get(no_session_handler));

    // Add protected admin routes
    let app = app.nest(
        "/admin",
        Router::new()
            .route("/", get(session_handler))
            .layer(SessionManagerLayer::new(session_config)),
    );

    // Run our app
    let addr = SocketAddr::from(("127.0.0.1".parse::<Ipv4Addr>()?, 3000));
    debug!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

#[debug_handler]
async fn no_session_handler() -> String {
    "Hello world".to_string()
}

// #[Protect(Resource, Read)]
#[debug_handler]
async fn session_handler(session: Session) -> String {
    format!("Session found with ID {}", session.0.uuid.to_string())
}
