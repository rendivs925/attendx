use serde_json::Value;
use shared::utils::locale_utils::Lang;
use std::fs;
use std::path::Path;

pub fn get_lang(req: &actix_web::HttpRequest) -> Lang {
    req.headers()
        .get("Accept-Language")
        .and_then(|value| value.to_str().ok())
        .and_then(|header| {
            header
                .split(',')
                .next()
                .and_then(|tag| tag.split('-').next())
        })
        .map(Lang::from_code)
        .unwrap_or(Lang::De)
}
