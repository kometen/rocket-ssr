use rocket_dyn_templates::{context, Template};

pub struct AuthContext {
    pub passwordless_api_key: String,
    pub passwordless_api_url: String,
}

impl AuthContext {
    pub fn new() -> Self {
        let passwordless_api_key =
            std::env::var("PASSWORDLESS_API_KEY").expect("PASSWORDLESS_API_KEY must be set.");
        let passwordless_api_url =
            std::env::var("PASSWORDLESS_API_URL").expect("PASSWORDLESS_API_URL must be set.");

        Self {
            passwordless_api_key,
            passwordless_api_url,
        }
    }

    pub fn render_template(&self, template_name: &str) -> Template {
        let template_name_owned = template_name.to_string();
        Template::render(
            template_name_owned,
            context! {
                passwordless_api_url: self.passwordless_api_url.clone(),
                passwordless_api_key: self.passwordless_api_key.clone()
            },
        )
    }
}
