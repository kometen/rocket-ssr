use crate::session::{SessionStore, UserProfile};
use rocket::{request::FromRequest, request::Outcome, tokio::sync::RwLock, Request};

#[derive(Debug)]
pub struct User(pub Option<UserProfile>);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let session_store = request.rocket().state::<RwLock<SessionStore>>().unwrap();

        let cookies = request.cookies();
        let token = cookies
            .get("session_token")
            .map(|cookie| cookie.value().to_string());

        if let Some(token) = token {
            let store = session_store.read().await;
            let profile = store.get_profile(&token);
            return Outcome::Success(User(profile));
        }

        Outcome::Success(User(None))
    }
}
