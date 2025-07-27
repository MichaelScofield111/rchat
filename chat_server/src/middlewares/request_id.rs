use axum::{extract::Request, http::HeaderValue, middleware::Next, response::Response};
use tracing::warn;

use super::REQUEST_ID_HEADER;

pub async fn set_request_id(mut req: Request, next: Next) -> Response {
    // if x-request-id is set do nothing, otherwise generate a new one
    let id = match req.headers().get(REQUEST_ID_HEADER) {
        Some(v) => v.as_bytes().to_vec(),
        None => {
            let request_id = uuid::Uuid::now_v7().to_string();
            match request_id.parse() {
                Ok(id) => {
                    req.headers_mut().insert(REQUEST_ID_HEADER, id);
                }
                Err(e) => {
                    warn!("Generate request id failed {}", e);
                }
            }
            request_id.as_bytes().to_vec()
        }
    };

    let mut res = next.run(req).await;
    match HeaderValue::from_bytes(&id) {
        Ok(v) => {
            res.headers_mut().insert(REQUEST_ID_HEADER, v);
        }
        Err(e) => {
            warn!("Set request id failed {}", e);
        }
    }

    res
}
