use axum::{RequestPartsExt, extract::FromRequestParts};
use axum_extra::{TypedHeader, headers::{Authorization, authorization::Bearer}};
use jsonwebtoken::{Validation, decode};
use serde::{Deserialize, Serialize};

use crate::modules::public::auth::infrastructure::{constants::JWT_TOKEN, errors::AuthError};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Claims {
    pub sub: String,
    pub email: String,
    pub exp: u64
}

impl<S> FromRequestParts<S> for Claims
where 
    S: Send + Sync
{
    type Rejection = AuthError;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        _: &S,
    ) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AuthError::InvalidToken)?;
        
        let token_data = decode::<Claims>(bearer.token(), &JWT_TOKEN.decoding, &Validation::default())
            .map_err(|_| AuthError::InvalidToken)?;
        
        Ok(token_data.claims)
    }
}