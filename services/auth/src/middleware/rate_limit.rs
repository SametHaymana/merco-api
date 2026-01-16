use axum::{
    extract::Request,
    http::HeaderMap,
    middleware::Next,
    response::Response,
};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use crate::error::AuthError;

struct RateLimitEntry {
    count: u32,
    reset_at: Instant,
}

pub struct RateLimitMiddleware {
    limits: Arc<Mutex<HashMap<String, RateLimitEntry>>>,
    max_requests: u32,
    window_seconds: u64,
}

impl RateLimitMiddleware {
    pub fn new(max_requests: u32, window_seconds: u64) -> Self {
        Self {
            limits: Arc::new(Mutex::new(HashMap::new())),
            max_requests,
            window_seconds,
        }
    }

    pub async fn rate_limit_middleware(
        &self,
        headers: HeaderMap,
        request: Request,
        next: Next,
    ) -> Result<Response, AuthError> {
        let key = self.get_rate_limit_key(&headers);
        
        let mut limits = self.limits.lock().unwrap();
        
        let entry = limits.entry(key.clone()).or_insert_with(|| {
            RateLimitEntry {
                count: 0,
                reset_at: Instant::now() + Duration::from_secs(self.window_seconds),
            }
        });

        if Instant::now() > entry.reset_at {
            entry.count = 0;
            entry.reset_at = Instant::now() + Duration::from_secs(self.window_seconds);
        }

        if entry.count >= self.max_requests {
            return Err(AuthError::RateLimitExceeded);
        }

        entry.count += 1;

        drop(limits);
        Ok(next.run(request).await)
    }

    fn get_rate_limit_key(&self, headers: &HeaderMap) -> String {
        // Use IP address or API key for rate limiting
        if let Some(api_key) = headers.get("x-api-key").and_then(|h| h.to_str().ok()) {
            format!("api_key:{}", api_key)
        } else if let Some(ip) = headers.get("x-forwarded-for").and_then(|h| h.to_str().ok()) {
            format!("ip:{}", ip.split(',').next().unwrap_or(ip))
        } else {
            "unknown".to_string()
        }
    }
}
