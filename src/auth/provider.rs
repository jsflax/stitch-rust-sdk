extern crate serde_json;

use self::serde_json::Value;

// An AuthProvider is responsible for providing the necessary information for a specific
// authentication request.
pub trait AuthProvider {
    fn auth_type(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn payload(&self) -> Value;
}
