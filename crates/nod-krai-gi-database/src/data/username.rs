use std::sync::LazyLock;

use regex::Regex;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Username(String);

impl Username {
    pub fn parse(username: String) -> Option<Self> {
        static ALLOWED_PATTERN: LazyLock<Regex> = 
            LazyLock::new(|| Regex::new("^[a-zA-Z0-9._@-]{6,25}$").unwrap());

        ALLOWED_PATTERN
            .is_match(&username)
            .then_some(Self(username))
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}
