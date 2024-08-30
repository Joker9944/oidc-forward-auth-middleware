use openidconnect::url::Url;
use salvo::http::header::{
    REFERRER_POLICY, STRICT_TRANSPORT_SECURITY, X_CONTENT_TYPE_OPTIONS, X_FRAME_OPTIONS,
    X_XSS_PROTECTION,
};
use salvo::http::HeaderValue;

use salvo::writing::Redirect;
use salvo::Response;
use salvo::{handler, prelude::Request};
use std::collections::HashMap;
use tracing::debug;
pub fn get_header(req: &Request, key: &str) -> String {
    req.headers()
        .get(key)
        .and_then(|value| value.to_str().ok())
        .unwrap_or("")
        .to_string()
}

pub fn get_query_param(querystring: &str, key: &str) -> String {
    let hash_query: HashMap<String, String> =
        Url::parse(&format!("https://whatever{}", querystring))
            .unwrap()
            .query_pairs()
            .into_owned()
            .collect();

    hash_query.get(key).unwrap_or(&"".to_string()).to_owned()
}

pub fn get_cookie(req: &Request, key: &str) -> String {
    req.cookie(key)
        .map(|cookie| cookie.value().to_string())
        .unwrap_or_else(|| "".to_string())
}

#[handler]
pub async fn security_middleware(req: &mut Request, res: &mut Response) {
    let headers = res.headers_mut();

    headers.insert(X_FRAME_OPTIONS, HeaderValue::from_static("SAMEORIGIN"));
    headers.insert(
        STRICT_TRANSPORT_SECURITY,
        HeaderValue::from_static("max-age=63072000; includeSubDomains"),
    );
    headers.insert(X_CONTENT_TYPE_OPTIONS, HeaderValue::from_static("noopen"));
    headers.insert(X_XSS_PROTECTION, HeaderValue::from_static("1; mode=block"));
    headers.insert(REFERRER_POLICY, HeaderValue::from_static("no-referrer"));

    if !get_header(req, "x-forwarded-proto")
        .to_lowercase()
        .eq("https")
    {
        debug!("Redirecting client to HTTPS.");

        res.render(Redirect::temporary(format!(
            "https://{}/{}",
            get_header(req, "x-forwarded-host"),
            get_header(req, "x-forwarded-uri").trim_start_matches("/")
        )));
    }
}
