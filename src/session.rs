pub const INITIAL_USERNAME: &str = "Anonymous";
pub const MAX_USERNAME_LENGTH: usize = 25;
pub const MIN_USERNAME_LENGTH: usize = 2;

pub fn normalize_username(username: &str) -> String {
    let mut result = username.to_string();

    if username.len() > MAX_USERNAME_LENGTH {
        result = username[..25].to_string();
    }

    if username.len() < MIN_USERNAME_LENGTH {
        result = String::from(INITIAL_USERNAME);
    }

    result
}
