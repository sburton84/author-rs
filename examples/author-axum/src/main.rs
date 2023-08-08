use author_axum::SessionManagerLayer;
use axum::debug_handler;
use axum::routing::get;
use axum::{Extension, Router};
use std::net::{Ipv4Addr, SocketAddr};
use tracing::debug;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialise tracing
    tracing_subscriber::fmt::init();

    // Build our application
    let app = Router::new().route("/", get(unprotected));

    // Add protected admin routes
    let app = app.nest(
        "/admin",
        Router::new()
            .route("/", get(protected))
            .layer(SessionManagerLayer::new()),
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
async fn unprotected() {}

// #[Protect(Resource, Read)]
#[debug_handler]
async fn protected() {}
