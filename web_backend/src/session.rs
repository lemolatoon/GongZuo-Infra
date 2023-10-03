use uuid::Uuid;

pub fn create_session_token() -> String {
    Uuid::new_v4().to_string()
}
