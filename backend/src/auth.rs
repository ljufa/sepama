use crate::errors::ServiceError;
use actix_web::client::Client;
use jsonwebtoken::jwk::{AlgorithmParameters, JwkSet};
use jsonwebtoken::{decode, decode_header, DecodingKey, TokenData, Validation};
use serde_json;
use std::collections::HashMap;
use std::error::Error;

pub async fn get_token_data(
    token: &str,
) -> Result<TokenData<HashMap<String, serde_json::Value>>, ServiceError> {
    let authority = std::env::var("AUTHORITY").expect("AUTHORITY must be set");
    let jwks = fetch_jwks(&format!(
        "{}{}",
        authority.as_str(),
        ".well-known/jwks.json"
    ))
    .await
    .expect("Can't get JWKS!");
    let header = decode_header(token)
        .map_err(|_| ServiceError::BadRequest("Invalid token header".to_string()))?;

    let kid = match header.kid {
        Some(k) => k,
        None => {
            return Err(ServiceError::BadRequest(
                "Token doesn't have a `kid` header field".into(),
            ))
        }
    };
    if let Some(j) = jwks.find(&kid) {
        match j.algorithm {
            AlgorithmParameters::RSA(ref rsa) => {
                let decoding_key = DecodingKey::from_rsa_components(&rsa.n, &rsa.e).unwrap();
                let mut validation = Validation::new(j.common.algorithm.unwrap());
                validation.validate_exp = false;
                let decoded_token =
                    decode::<HashMap<String, serde_json::Value>>(token, &decoding_key, &validation)
                        .unwrap();
                return Ok(decoded_token);
            }
            _ => unreachable!("this should be a RSA"),
        }
    } else {
        return Err(ServiceError::JWKSFetchError);
    }
}

async fn fetch_jwks(uri: &str) -> Result<JwkSet, Box<dyn Error>> {
    let client = Client::default();
    let mut res = client.get(uri).send().await?;
    let val = res.json::<JwkSet>().await?;
    return Ok(val);
}
