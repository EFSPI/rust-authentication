use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey, errors::Error};
use serde::{Serialize, Deserialize};
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub scope: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RefreshToken {
    pub token: String,
    pub exp: usize,
}

/// Create a signed JWT
///
/// #Returns
/// - `Ok(String)` containing the JWT if everything is ok
/// - `Err(Error)` if the generation fails
pub fn create_jwt(user_id: &str, secret: &str, expiration: usize) -> Result<String, Error> {
    let exp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| Error::from(jsonwebtoken::errors::ErrorKind::InvalidRsaKey("test")))?
        .as_secs() as usize + expiration;

    let claims = Claims {
        sub: user_id.to_owned(),
        exp,
        scope: Some("read:write".to_string()),
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref()))

}

pub fn create_refresh_token() -> String {
    Uuid::new_v4().to_string()
}

pub fn validate_jwt(token: &str, secret: &str) -> Result<Claims, Error> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )
    .map(|data| data.claims)
}

/// TODO: Réparer cette fonction pour utiliser une db, quitte à faire du sqlite3
pub fn validate_refresh_token(refresh_token: &str) -> bool {
    !refresh_token.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;
    // use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey };

    const SECRET: &str = "super_secret_key";
    const EXPTIME : usize = 3600;

    #[test]
    fn test_generate_jwt() {
        let token = create_jwt("user123", SECRET, EXPTIME);
        assert!(token.is_ok());
    }

    #[test]
    fn test_validate_jwt() {
        let token = create_jwt("user123", SECRET, EXPTIME)
            .expect("Not buggy here");
        let claims = validate_jwt(&token, SECRET).unwrap();
        assert_eq!(claims.sub, "user123");
    }

    #[test]
    fn test_validate_invalid_jwt() {
        let invalid_token = "invalid_token_string";
        let result = validate_jwt(invalid_token, SECRET);
        assert!(result.is_err());
    }
}
