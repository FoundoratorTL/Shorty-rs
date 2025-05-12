use uuid::Uuid;

/// Generate a version-4 UUID string for short codes
pub fn generate_short_code() -> String {
    Uuid::new_v4().to_string()
}
