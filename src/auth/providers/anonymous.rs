use auth::provider::AuthProvider;
use auth::providers::Value;

pub struct AnonymousAuthProvider;

impl AuthProvider for AnonymousAuthProvider {
    fn auth_type(&self) -> &'static str { "anon" }
    fn name(&self) -> &'static str { "user" }
    fn payload(&self) -> Value { json!({}) }
}
