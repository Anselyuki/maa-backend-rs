use std::sync::Arc;

use uuid::Uuid;
use validator::Validate;

use crate::{
    envs::max_login_count,
    repository::user_repository::UserRepository,
    route::{
        request::user::{LoginRequest, RegisterRequest},
        response::user::{MaaLoginResponse, MaaUserInfo},
    },
    util::password_encoder::PasswordEncoder,
    MaaError, MaaResult,
};

use super::jwt_service::JwtService;

pub struct UserService {
    user_repository: UserRepository,
    password_encoder: PasswordEncoder,
    jwt_service: Arc<JwtService>,
    max_login: usize,
}

impl UserService {
    pub fn new(
        user_repository: UserRepository,
        jwt_service: Arc<JwtService>,
    ) -> Self {
        let password_encoder = PasswordEncoder::new();
        let max_login = max_login_count().unwrap_or(1);
        Self {
            user_repository,
            password_encoder,
            max_login,
            jwt_service,
        }
    }

    pub async fn login(
        &self,
        req: LoginRequest,
    ) -> MaaResult<MaaLoginResponse> {
        req.validate()?;

        let mut user = self
            .user_repository
            .find_by_email(&req.email)
            .await?
            .ok_or(MaaError::LoginFail)?;

        let user_id = match user.user_id {
            Some(ref id) => id.clone(),
            None => return Err(MaaError::NoneUserId),
        };

        if !self
            .password_encoder
            .matches(&req.password, &user.password)?
        {
            return Err(MaaError::LoginFail);
        }

        if user.status == 0 {
            return Err(MaaError::UserNotEnabled);
        }

        let jwt_id = Uuid::new_v4().to_string();
        user.refresh_jwt_ids.push(jwt_id.clone());
        while user.refresh_jwt_ids.len() > self.max_login {
            user.refresh_jwt_ids.remove(0);
        }

        let authorities: Vec<String> =
            (0..user.status).map(|i| i.to_string()).collect();

        let auth_token = self.jwt_service.issue_auth_token(
            user_id.clone(),
            None,
            authorities,
        )?;

        let refresh_token = self
            .jwt_service
            .issue_refresh_token(user_id, Some(jwt_id))?;

        let resp = MaaLoginResponse {
            token: auth_token.token,
            valid_before: auth_token.expires_at,
            valid_after: auth_token.not_before,
            refresh_token: refresh_token.token,
            refresh_token_valid_before: auth_token.expires_at,
            refresh_token_valid_after: auth_token.not_before,
            user_info: user.into(),
        };

        Ok(resp)
    }

    pub async fn register(req: RegisterRequest) -> MaaResult<MaaUserInfo> {
        req.validate()?;

        todo!()
    }
}
