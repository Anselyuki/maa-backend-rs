use jsonwebtokens::{Algorithm, AlgorithmID, Verifier};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{
    envs::{jwt_expire_time, jwt_key},
    MaaError, MaaResult,
};

pub struct JwtService {
    algorithm: Algorithm,
    verifier: Verifier,
    expire_time: u64,
}

#[derive(Debug)]
pub struct SignedJwt {
    pub token: String,
    pub expires_at: i64,
    pub not_before: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JwtAuthClaims {
    pub sub: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jti: Option<String>,
    #[serde(rename = "Authorities")]
    pub auth: Vec<String>,
    pub iat: i64,
    pub exp: i64,
    pub nbf: i64,
    pub typ: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JwtRefreshClaims {
    pub sub: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jti: Option<String>,
    pub iat: i64,
    pub exp: i64,
    pub nbf: i64,
    pub typ: String,
}

impl JwtService {
    pub fn new() -> MaaResult<Self> {
        let jwt_key = jwt_key()?.as_bytes().to_vec();

        let expire_time = jwt_expire_time()?;

        let alg = Algorithm::new_hmac(AlgorithmID::ES256, jwt_key)?;

        let verifier = Verifier::create().build()?;

        Ok(Self {
            algorithm: alg,
            expire_time,
            verifier,
        })
    }

    pub fn issue_auth_token(
        &self,
        subject: String,
        jwt_id: Option<String>,
        authoritories: Vec<String>,
    ) -> MaaResult<SignedJwt> {
        let now = chrono::Utc::now().timestamp();
        let expire = now + self.expire_time as i64;
        let claims = JwtAuthClaims {
            sub: subject,
            jti: jwt_id,
            auth: authoritories,
            iat: now,
            exp: expire,
            nbf: now,
            typ: "auth".to_string(),
        };

        let header = json!({
            "alg": self.algorithm.name()
        });

        let token = jsonwebtokens::encode(&header, &claims, &self.algorithm)?;

        Ok(SignedJwt {
            token,
            expires_at: claims.exp,
            not_before: claims.nbf,
        })
    }

    pub fn verify_and_parse_auth_token(
        &self,
        auth_token: &str,
    ) -> MaaResult<JwtAuthClaims> {
        let claims = self
            .verifier
            .verify(auth_token, &self.algorithm)
            .map_err(|_| MaaError::JwtVerifyFailed)?;

        serde_json::from_value(claims).map_err(|_| MaaError::JwtVerifyFailed)
    }

    pub fn issue_refresh_token(
        &self,
        subject: String,
        jwt_id: Option<String>,
    ) -> MaaResult<SignedJwt> {
        let now = chrono::Utc::now().timestamp();
        let expire = now + self.expire_time as i64;
        let claims = JwtRefreshClaims {
            sub: subject,
            jti: jwt_id,
            iat: now,
            exp: expire,
            nbf: now,
            typ: "refresh".to_string(),
        };

        let header = json!({
            "alg": self.algorithm.name()
        });

        let token = jsonwebtokens::encode(&header, &claims, &self.algorithm)?;

        Ok(SignedJwt {
            token,
            expires_at: claims.exp,
            not_before: claims.nbf,
        })
    }

    pub fn new_refresh_token(
        &self,
        old: JwtRefreshClaims,
        jwt_id: Option<String>,
    ) -> MaaResult<SignedJwt> {
        let now = chrono::Utc::now().timestamp();
        let claims = JwtRefreshClaims {
            sub: old.sub,
            jti: jwt_id,
            iat: now,
            exp: old.exp,
            nbf: now,
            typ: "refresh".to_string(),
        };

        let header = json!({
            "alg": self.algorithm.name()
        });

        let token = jsonwebtokens::encode(&header, &claims, &self.algorithm)?;

        Ok(SignedJwt {
            token,
            expires_at: claims.exp,
            not_before: claims.nbf,
        })
    }

    pub fn verify_and_parse_refresh_token(
        &self,
        refresh_token: &str,
    ) -> MaaResult<JwtRefreshClaims> {
        let claims = self
            .verifier
            .verify(refresh_token, &self.algorithm)
            .map_err(|_| MaaError::JwtVerifyFailed)?;

        serde_json::from_value(claims).map_err(|_| MaaError::JwtVerifyFailed)
    }
}
