use crate::db::{self, Role};
use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("sqlite3 error: {0}")]
    Sqlite3Error(#[from] rusqlite::Error),
    #[error("Invalid password hash: {0}")]
    PwdHashError(#[from] argon2::password_hash::Error),
}

pub fn add_user(name: &str, pwd: &str, email: &str, role: Role) -> Result<(), AuthError> {
    let salt = SaltString::generate(&mut OsRng);

    let pwd_hash: String = Argon2::default()
        .hash_password(pwd.as_bytes(), &salt)?
        .to_string();

    db::add_user(name, pwd_hash.as_str(), email, role)?;

    Ok(())
}

pub fn auth_user(name: &str, pwd: &str) -> Result<i64, AuthError> {
    let (id, pwd_hash): (i64, String) = db::get_user_id_pwd_hash(name)?;

    let pwd_hash = PasswordHash::new(&pwd_hash)?;

    Argon2::default().verify_password(pwd.as_bytes(), &pwd_hash)?;

    Ok(id)
}
