use std::{future::Future, net::SocketAddr, pin::Pin};

use axum::{
    body::Body,
    extract::{ConnectInfo, Request},
    response::{IntoResponse, Response},
};
use http::StatusCode;
use redis::AsyncCommands;
use tower::{Layer, Service};

use crate::{util::request_ext::RequestExt, MaaAppState};

#[derive(Clone)]
pub struct AccessLimitLayer {
    limit: u32,
    seconds: u64,
}

impl AccessLimitLayer {
    pub fn new(limit: u32, seconds: u64) -> Self {
        Self { limit, seconds }
    }
}

#[derive(Clone)]
pub struct AccessLimitService<S> {
    inner: S,
    limits: u32,
    seconds: u64,
}

impl<S> Service<Request> for AccessLimitService<S>
where
    S: Service<Request, Response = Response<Body>> + Send + 'static,
    S::Future: Send,
    S::Error: IntoResponse,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, request: Request) -> Self::Future {
        let remote_addr = request
            .extensions()
            .get::<ConnectInfo<SocketAddr>>()
            .map(|c| c.0);

        let ip = request.get_ip_addr(remote_addr);
        let request_uri = request.uri().to_string();
        let key = format!("{}{}", ip, request_uri);
        let state = request.extensions().get::<MaaAppState>().cloned().unwrap();

        let limits = self.limits;
        let seconds = self.seconds;

        let inner = self.inner.call(request);
        Box::pin(async move {
            let redis_con = state.redis_pool.get().await;

            let mut redis_con = match redis_con {
                Ok(con) => con,
                Err(e) => {
                    tracing::error!("Failed to get redis connection: {}", e);
                    return Ok(StatusCode::INTERNAL_SERVER_ERROR.into_response());
                }
            };

            let count: Result<Option<u32>, _> = redis_con.get(&key).await;
            let count = match count {
                Ok(c) => c,
                Err(e) => {
                    tracing::error!("Failed to get count from redis: {}", e);
                    return Ok(StatusCode::INTERNAL_SERVER_ERROR.into_response());
                }
            };

            let count = count.unwrap_or(0);

            if count >= limits {
                return Ok(StatusCode::TOO_MANY_REQUESTS.into_response());
            }

            let _: Result<(), _> = redis_con.set_ex(&key, count + 1, seconds).await;
            let response = inner.await?;
            Ok(response)
        })
    }
}

impl<S> Layer<S> for AccessLimitLayer {
    type Service = AccessLimitService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        AccessLimitService {
            inner,
            limits: self.limit,
            seconds: self.seconds,
        }
    }
}
