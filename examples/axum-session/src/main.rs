use author_web::session::store::in_memory::{
    InMemorySession, InMemorySessionData, InMemorySessionStore,
};
use author_web::session::{SessionConfig, SessionData};

use crate::schema::{auth_session, auth_user};
use author_axum::session::{Session, SessionManagerLayer};
use author_axum::user::User;
use author_web::session::store::SessionDataValueStorage;
use author_web::user::UserSession;
use axum::debug_handler;
use axum::extract::Path;
use axum::routing::get;
use axum::Router;
use sea_orm::sea_query::Table;
use sea_orm::{ConnectionTrait, Database, DbBackend, DbConn, DbErr, Schema};
use std::net::{Ipv4Addr, SocketAddr};
use tracing::debug;

mod schema;

enum Roles {
    User,
    Admin,
}

// struct ExampleSession {
//
// }
//
// async fn setup_schema(db: &DbConn) -> Result<(), DbErr> {
//     let schema = Schema::new(DbBackend::Sqlite);
//
//     // Derive from Entity
//     let stmt = schema.create_table_from_entity(auth_session::Entity);
//
//     // Execute create table statement
//     db.execute(db.get_database_backend().build(&stmt)).await?;
//
//     // Derive from Entity
//     let stmt = schema.create_table_from_entity(auth_user::Entity);
//
//     // Execute create table statement
//     db.execute(db.get_database_backend().build(&stmt)).await?;
//
//     Ok(())
// }

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialise tracing
    tracing_subscriber::fmt::init();

    // let db: DbConn = Database::connect("sqlite::memory:").await?;
    // setup_schema(&db)?;

    // Create the session config
    let session_config = SessionConfig::default();

    let string_session_store = InMemorySessionStore::<InMemorySessionData>::new();

    // Build our application
    let app = Router::new().route("/", get(no_session_handler));

    // Add protected admin routes
    let app = app.nest(
        "/admin",
        Router::new()
            // .route("/role", get(role_handler))
            // .layer(RoleGuardLayer)
            .route("/session", get(session_handler))
            .route("/user", get(user_handler))
            .route("/set_user/:name", get(set_user_handler))
            .layer(SessionManagerLayer::new(
                session_config.clone(),
                string_session_store,
            )),
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
    let value = { session.get_value("test_key").await.clone() };

    session
        .set_value("test_key".to_string(), "test_value".to_string())
        .await;

    format!("Session found with value: {:?}", value)
}

#[debug_handler]
async fn user_handler(User(user, _): User<String, InMemorySession>) -> String {
    format!("Logged in as user with name: {:?}", user)
}

#[debug_handler]
async fn set_user_handler(
    Session(mut session): Session<InMemorySession>,
    Path(name): Path<String>,
) -> String {
    session.set_user(name.clone()).await;

    format!("User set to: {:?}", name)
}

// #[debug_handler]
// async fn role_handler() -> String {}

// #[debug_handler]
// async fn make_me_admin(Session(mut session): Session<InMemorySession>) -> String {}
//
// #[debug_handler]
// async fn user_with_role_handler(user: UserWithRole<Role>) -> String {}

// #[Protect(Resource, Read)]
// async fn protected_handler() -> String {
//
// }
