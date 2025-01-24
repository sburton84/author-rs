use crate::session::Session;
use author_web::user::UserSession;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::http::StatusCode;
use std::marker::PhantomData;
use tracing::trace;

#[derive(Clone)]
pub struct User<U: Clone, Sess>(pub U, pub PhantomData<Sess>);

#[derive(Clone)]
pub struct UserWithRole<U: Clone>(pub U);

impl<S, U, Sess> FromRequestParts<S> for User<U, Sess>
where
    S: Send + Sync,
    Sess: UserSession<User = U> + Clone + Send + Sync + 'static,
    U: Clone,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        trace!("Loading user for request");

        let Session(session) = parts
            .extensions
            .get::<Session<Sess>>()
            .cloned()
            .ok_or((StatusCode::FORBIDDEN, "Forbidden"))?;

        trace!("Loaded session");

        let user = session
            .current_user()
            .await
            .map_err(|_| (StatusCode::FORBIDDEN, "Forbidden"))?
            .ok_or((StatusCode::FORBIDDEN, "Forbidden"))?;

        trace!("Loaded user");

        Ok(User(user, PhantomData::default()))
    }
}
