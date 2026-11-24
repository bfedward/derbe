use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,  // user unid
    pub exp: usize, // unix timestamp
}

pub fn make_token(user_unid: Uuid, secret: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let exp = (chrono::Utc::now() + chrono::Duration::days(30)).timestamp() as usize;
    encode(
        &Header::default(),
        &Claims {
            sub: user_unid,
            exp,
        },
        &EncodingKey::from_secret(secret.as_bytes()),
    )
}

pub fn verify_token(token: &str, secret: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )?;

    Ok(data.claims)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_token_verifies_with_the_same_secret() {
        let unid = Uuid::new_v4();
        let token = make_token(unid, "secret").unwrap();
        assert_eq!(verify_token(&token, "secret").unwrap().sub, unid);
    }

    #[test]
    fn a_token_is_rejected_with_another_secret() {
        let token = make_token(Uuid::new_v4(), "secret").unwrap();
        assert!(verify_token(&token, "other").is_err());
    }
}
