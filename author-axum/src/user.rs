use crate::session::Session;
use author_web::user::UserSession;
use axum::async_trait;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::http::{Request, StatusCode};
use std::marker::PhantomData;
use tower_layer::Layer;
use tower_service::Service;

#[derive(Clone)]
pub struct User<U: Clone, Sess>(pub U, pub PhantomData<Sess>);

#[derive(Clone)]
pub struct UserWithRole<U: Clone>(pub U);

#[async_trait]
impl<S, U, Sess> FromRequestParts<S> for User<U, Sess>
where
    Sess: UserSession<U> + Clone + Send + Sync + 'static,
    U: Clone,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let Session(session) = parts
            .extensions
            .get::<Session<Sess>>()
            .cloned()
            .ok_or((StatusCode::FORBIDDEN, "Forbidden"))?;

        let user = session
            .current_user()
            .ok_or((StatusCode::FORBIDDEN, "Forbidden"))?;

        Ok(User(user, PhantomData::default()))
    }
}

// pub struct UserService<Inner> {
//     inner: Inner,
// }
//
// impl<Inner> UserService<Inner> {
//     pub fn new(inner: Inner) -> Self {
//         UserService { inner }
//     }
// }
//
// impl<Inner, S, K, B, ResBody> Service<Request<B>> for UserService<Inner> {
//     // type Response = (
//     //     Option<PrivateCookieJar>,
//     //     Result<Inner::Response, StatusCode>,
//     // );
//     // type Error = Infallible;
//     // type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;
//     //
//     // fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
//     //     self.inner.poll_ready(cx)
//     // }
//     //
//     // fn call(&mut self, req: Request<B>) -> Self::Future {
//     // }
// }
//
// pub struct UserLayer {}
//
// impl UserLayer {
//     pub fn new() -> Self {
//         UserLayer {}
//     }
// }
//
// impl<Inner> Layer<Inner> for UserLayer {
//     type Service = UserService<Inner>;
//
//     fn layer(&self, inner: Inner) -> Self::Service {
//         UserService::new(inner)
//     }
// }
