use std::{future::Future, net::SocketAddr, pin::Pin, sync::Arc};

use axum::{
    body::Body,
    extract::{ConnectInfo, Request},
    response::{IntoResponse, Response},
};
use http::StatusCode;
use tower::{Layer, Service};

use crate::{util::request_ext::RequestExt, AppState};

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
    type Future = Pin<
        Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>,
    >;

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
        let state = request
            .extensions()
            .get::<Arc<AppState>>()
            .cloned()
            .unwrap();

        let limits = self.limits;
        let seconds = self.seconds;

        let inner = self.inner.call(request);
        Box::pin(async move {
            let count: Result<Option<u32>, _> =
                state.redis_cache.get(&key).await;
            let count = match count {
                Ok(c) => c,
                Err(e) => {
                    tracing::error!("Failed to get count from redis: {}", e);
                    return Ok(
                        StatusCode::INTERNAL_SERVER_ERROR.into_response()
                    );
                }
            };

            let count = count.unwrap_or(0);

            tracing::debug!("Count: {}, limits: {}", count, limits);

            if count >= limits {
                return Ok(StatusCode::TOO_MANY_REQUESTS.into_response());
            }

            let _: Result<(), _> =
                state.redis_cache.set_ex(&key, count + 1, seconds).await;
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
