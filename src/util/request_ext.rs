use std::net::SocketAddr;

use axum::extract::Request;
use local_ip_address::local_ip;

pub trait RequestExt {
    fn get_ip_addr(&self, sokect_addr: Option<SocketAddr>) -> String;
}

const UNKNOWN_IP: &str = "unknown";

macro_rules! header_or_empty {
    ($request:expr, $header:expr) => {
        $request
            .headers()
            .get($header)
            .and_then(|header| header.to_str().ok())
            .unwrap_or("")
    };
}

impl RequestExt for Request {
    fn get_ip_addr(&self, socket_addr: Option<SocketAddr>) -> String {
        let mut ip = header_or_empty!(self, "x-forwarded-for");

        if ip.is_empty() || ip.eq_ignore_ascii_case(UNKNOWN_IP) {
            ip = header_or_empty!(self, "Proxy-Client-IP");
        }

        if ip.is_empty() || ip.eq_ignore_ascii_case(UNKNOWN_IP) {
            ip = header_or_empty!(self, "WL-Proxy-Client-IP");
        }

        let mut ip = if ip.is_empty() || ip.eq_ignore_ascii_case(UNKNOWN_IP) {
            let mut ip_str = socket_addr
                .as_ref()
                .map(|addr| addr.ip().to_string())
                .unwrap_or_default();
            if ip_str.eq("127.0.0.1") {
                if let Ok(local_ip) = local_ip() {
                    ip_str = local_ip.to_string();
                }
            }
            ip_str
        } else {
            ip.into()
        };

        // 对于通过多个代理的情况，第一个IP为客户端真实IP,多个IP按照','分割
        if ip.len() > 15 {
            if let Some(index) = ip.find(',') {
                ip = ip.split_at(index).0.to_string();
            }
        }

        ip
    }
}
