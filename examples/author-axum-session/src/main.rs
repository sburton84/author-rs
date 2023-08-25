use author_axum::{Session, SessionManagerLayer};
use author_web::session::store::in_memory::{
    InMemorySession, InMemorySessionData, InMemorySessionStore,
};
use author_web::session::SessionDataValues;
use author_web::SessionConfig;
use axum::debug_handler;
use axum::routing::get;
use axum::Router;
use std::net::{Ipv4Addr, SocketAddr};
use tracing::debug;
use uuid::Uuid;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialise tracing
    tracing_subscriber::fmt::init();

    // Create the session config
    let session_config = SessionConfig::default();

    let session_store = InMemorySessionStore::<InMemorySessionData, Uuid>::new();

    // Build our application
    let app = Router::new().route("/", get(no_session_handler));

    // Add protected admin routes
    let app = app.nest(
        "/admin",
        Router::new()
            .route("/", get(session_handler))
            .layer(SessionManagerLayer::new(session_config, session_store)),
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

#[debug_handler]
async fn session_handler(Session(mut session): Session<InMemorySession>) -> String {
    let value = { session.get_value("test_key").clone() };

    session.set_value("test_key".to_string(), "test_value".to_string());

    format!("Session found with value: {:?}", value)
}

// #[Protect(Resource, Read)]
// async fn protected_handler() -> String {
//
// }
