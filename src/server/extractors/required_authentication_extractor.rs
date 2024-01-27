use async_trait::async_trait;
use axum::extract::FromRequestParts;
use axum::http::header::AUTHORIZATION;
use axum::http::request::Parts;
use axum::Extension;
use tracing::error;
use uuid::Uuid;

use crate::server::error::Error;
use crate::server::services::Services;

/// Extracts the JWT from the Authorization token header.
/// JWT 토큰 헤더에서 인증 정보 추출
pub struct RequiredAuthentication(pub Uuid, pub Services);

#[async_trait]
impl<S> FromRequestParts<S> for RequiredAuthentication
where
    S: Send + Sync,
{
    type Rejection = Error;
    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let Extension(services): Extension<Services> = Extension::from_request_parts(parts, state)
            .await
            .map_err(|err| Error::InternalServerErrorWithContext(err.to_string()))?;

        // parts 에서 헤더 정보 읽어와서 인증 정보 가져옴
        if let Some(authorization_header) = parts.headers.get(AUTHORIZATION) {
            let header_value = authorization_header
                .to_str()
                .map_err(|_| Error::Unauthorized)?;

            // 토큰 헤더에 Bearer 시작 여부 판단 및 없으면 오류 발생
            if !header_value.contains("Bearer") {
                error!("request does not contain valid 'Bearer' prefix for authorization");
                return Err(Error::Unauthorized);
            }

            let tokenized_value: Vec<_> = header_value.split(' ').collect();

            if tokenized_value.len() != 2 || tokenized_value.get(1).is_none() {
                error!("request does not contain a valid token");
                return Err(Error::Unauthorized);
            }

            let token_value = tokenized_value.into_iter().nth(1).unwrap();
            let user_id = services
                .jwt_util
                .get_user_id_from_token(String::from(token_value))
                .map_err(|err| {
                    error!("could not validate user ID from token: {:?}", err);
                    Error::Unauthorized
                })?;

            let user = services
                .users
                .get_current_user(user_id)
                .await
                .map_err(|err| {
                    error!("invalid user ID from token: {:?}", err);
                    Error::Unauthorized
                })?;

            Ok(RequiredAuthentication(user.id, services))
        } else {
            Err(Error::Unauthorized)
        }
    }
}
