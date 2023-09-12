use axum::async_trait;
use axum::extract::{FromRequestParts, State};
use axum::http::request::Parts;
use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;
use axum::RequestPartsExt;
use lazy_regex::regex_captures;
use tower_cookies::{Cookie, Cookies};

use crate::ctx::Ctx;
use crate::model::ModelControler;
use crate::web::AUTH_TOKEN;
use crate::{Error, Result};

pub async fn mw_ctx_resolver<B>(
    _mc: State<ModelControler>,
    cookies: Cookies,
    mut req: Request<B>,
    next: Next<B>,
) -> Result<Response> {
    println!("->> {:<12} - mw_ctx_resolver", "MIDDLEWARE");

    let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());

    let result_ctx = match auth_token
        .ok_or(Error::AuthFailNoAuthTokenCookie)
        .and_then(parse_token)
    {
        Ok((user_id, exp, sig)) => Ok(Ctx::new(user_id)),
        Err(e) => Err(e),
    };

    // Remove cookie

    if result_ctx.is_err() && matches!(result_ctx, Err(Error::AuthFailNoAuthTokenCookie)) {
        cookies.remove(Cookie::named(AUTH_TOKEN));
    }

    req.extensions_mut().insert(result_ctx);

    Ok(next.run(req).await)
}

#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for Ctx {
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self> {
        println!("->> {:<12} - Ctx", "EXTRACTOR");

        parts
            .extensions
            .get::<Result<Ctx>>()
            .ok_or(Error::AuthFailCtxNotInExtension)?
            .clone()
    }
}

pub async fn require_auth<B>(ctx: Result<Ctx>, req: Request<B>, next: Next<B>) -> Result<Response> {
    println!("->> {:<12} - reqire_auth", "MIDDLEWARE");

    ctx?;

    Ok(next.run(req).await)
}

fn parse_token(token: String) -> Result<(u64, String, String)> {
    let (_, id, exp, sign) = regex_captures!(r#"^user-(\d+)\.(.+)\.(.+)"#, &token)
        .ok_or(Error::AuthFailTokenWrongFormat)?;

    let user_id: u64 = id.parse().map_err(|_| Error::AuthFailTokenWrongFormat)?;

    Ok((user_id, exp.to_string(), sign.to_string()))
}
