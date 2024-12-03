use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey, errors::Error};
use serde::{Serialize, Deserialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
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
    };
    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref()))

}

pub fn validate_jwt(token: &str, secret: &str) -> Result<Claims, Error> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )
    .map(|data| data.claims)
}
