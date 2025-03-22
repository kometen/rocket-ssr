use crate::session::{SessionStore, UserProfile};
use rocket::{
    request::{FromParam, FromRequest, Outcome},
    tokio::sync::RwLock,
    Request,
};

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

pub struct LimitedId(pub String);

impl<'r> FromParam<'r> for LimitedId {
    type Error = &'static str;

    fn from_param(param: &'r str) -> Result<Self, Self::Error> {
        if param.len() > 30 {
            Err("ID parameter too long")
        } else {
            let sanitized: String = param
                .chars()
                .filter(|c| c.is_alphanumeric() || *c == '-' || *c == '_')
                .collect();

            Ok(LimitedId(sanitized))
        }
    }
}
