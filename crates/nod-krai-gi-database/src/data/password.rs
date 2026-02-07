use crate::util;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("password pair mismatch")]
    PairMismatch,
    #[error("password length mismatch")]
    LengthMismatch,
    #[error("failed to generate password hash: {0}")]
    HashFailed(pbkdf2::password_hash::Error),
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Password(String);

impl Password {
    const PASSWORD_LENGTH: std::ops::Range<usize> = 8..30;

    pub fn new(password: String, password_v2: String) -> Result<Self, Error> {
        if password != password_v2 {
            return Err(Error::PairMismatch);
        }

        if !Self::PASSWORD_LENGTH.contains(&password.len()) {
            return Err(Error::LengthMismatch);
        }

        let hash = util::hash_string(&password).map_err(Error::HashFailed)?;
        Ok(Self(hash))
    }

    pub fn verify(&self, password: &str) -> bool {
        util::verify_hash(password, &self.0).is_some()
    }

    pub fn hash_str(&self) -> &str {
        self.0.as_str()
    }
}
