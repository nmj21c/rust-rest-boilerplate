use axum::extract::Json;
use axum::routing::{get, post, put};
use axum::{Extension, Router};
use axum_extra::extract::cookie::{Cookie, CookieJar};
use tracing::info;

use crate::extractors::{SessionExtractor, UserAgentExtractor};
use crate::server::dtos::user_dto::{
    SignInUserDto, SignUpUserDto, UpdateUserDto, UserAuthenicationResponse,
};
use crate::server::error::AppResult;
use crate::server::extractors::{RequiredAuthentication, ValidationExtractor};
use crate::server::services::Services;

pub struct UserController;

// 유저 컨트롤러
impl UserController {

    // 라우터 정의 /api/v1/users 이하 정의
    pub fn app() -> Router {
        Router::new()
            .route("/signup", post(Self::signup_user_endpoint))
            .route("/signin", post(Self::signin_user_endpoint))
            .route("/signout", post(Self::signout_user_endpoint))
            .route("/whoami", get(Self::get_current_user_endpoint))
            .route("/refresh", get(Self::refresh_user_endpoint))
            .route("/", put(Self::update_user_endpoint))
    }

    // 가입
    pub async fn signup_user_endpoint(
        Extension(services): Extension<Services>,
        ValidationExtractor(request): ValidationExtractor<SignUpUserDto>,
    ) -> AppResult<Json<UserAuthenicationResponse>> {
        info!(
            "recieved request to create user {:?}/{:?}",
            request.email.as_ref().unwrap(),
            request.name.as_ref().unwrap()
        );

        let created_user = services.users.signup_user(request).await?;

        Ok(Json(UserAuthenicationResponse { user: created_user }))
    }

    // 로그인
    pub async fn signin_user_endpoint(
        jar: CookieJar,
        Extension(services): Extension<Services>,
        UserAgentExtractor(user_agent): UserAgentExtractor,
        ValidationExtractor(request): ValidationExtractor<SignInUserDto>,
    ) -> AppResult<(CookieJar, Json<UserAuthenicationResponse>)> {
        info!(
            "recieved request to login user {:?}",
            request.email.as_ref().unwrap()
        );

        let (user, refresh_token) = services.users.signin_user(request, user_agent).await?;

        let cookie = jar.add(Cookie::new("refresh_token", refresh_token.to_string()));

        Ok((cookie, Json(UserAuthenicationResponse { user })))
    }

    // 토큰으로 내 정보 조회
    pub async fn get_current_user_endpoint(
        RequiredAuthentication(user_id, services): RequiredAuthentication,
    ) -> AppResult<Json<UserAuthenicationResponse>> {
        info!("recieved request to retrieve current user");

        let current_user = services.users.get_current_user(user_id).await?;

        Ok(Json(UserAuthenicationResponse { user: current_user }))
    }

    // 내 정보 업데이트
    pub async fn update_user_endpoint(
        RequiredAuthentication(user_id, services): RequiredAuthentication,
        Json(request): Json<UpdateUserDto>,
    ) -> AppResult<Json<UserAuthenicationResponse>> {
        info!("recieved request to update user {:?}", user_id);

        let updated_user = services.users.updated_user(user_id, request).await?;

        Ok(Json(UserAuthenicationResponse { user: updated_user }))
    }

    // 토큰 갱신
    pub async fn refresh_user_endpoint(
        jar: CookieJar,
        Extension(services): Extension<Services>,
        SessionExtractor(session_id, refresh_token): SessionExtractor,
    ) -> AppResult<(CookieJar, Json<UserAuthenicationResponse>)> {
        info!("recieved request to refresh access token {:?}", session_id);

        let user = services.sessions.refresh_access_token(session_id).await?;

        let cookie = jar.add(Cookie::new("refresh_token", refresh_token));

        Ok((cookie, Json(UserAuthenicationResponse { user })))
    }

    // 로그아웃
    pub async fn signout_user_endpoint(
        jar: CookieJar,
        Extension(services): Extension<Services>,
        SessionExtractor(session_id, _refresh_token): SessionExtractor,
    ) -> AppResult<CookieJar> {
        info!("recieved request to signout session {:?}", session_id);

        services.sessions.refresh_access_token(session_id).await?;

        let cookie = jar.remove(Cookie::from("refresh_token"));

        Ok(cookie)
    }
}
