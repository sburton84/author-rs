use axum::routing::get;
use axum::{Extension, Router};
use std::net::SocketAddr;
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
            .layer(Extension(auth)),
    );

    // Run our app
    let addr = SocketAddr::from(("127.0.0.1", 3000));
    debug!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn unprotected() {}

#[Protect(Resource, Read)]
fn protected() {}
