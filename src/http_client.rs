#[cfg(feature = "async")]
use core::future::Future;
use std::io::{Error, ErrorKind};

use http::response::Response;

fn io_error<E: std::fmt::Debug>(msg: &str, cause: E) -> Error {
    log::warn!("{msg}\n{cause:#?}");
    Error::new(ErrorKind::Other, msg)
}

#[cfg(feature = "blocking")]
pub fn get_blocking(url: &str) -> Result<Response<String>, Error> {
    match ureq::get(url)
        .config()
        .http_status_as_error(false)
        .build()
        .call()
    {
        Ok(response) => {
            let (parts, mut body) = response.into_parts();
            if parts.status.is_success() {
                log::trace!("successful GET {url}");
            } else {
                log::warn!("failed GET {url} ({:?})", parts.status.canonical_reason())
            }
            let response_str = body
                .read_to_string()
                .map_err(|e| io_error("failed to convert response", e))?;
            let resp = Response::from_parts(parts, response_str);
            Ok(resp)
        }
        Err(e) => Err(io_error("failed http request", e)),
    }
}

#[cfg(feature = "async")]
pub fn get_async(url: &str) -> impl Future<Output = Result<Response<String>, Error>> {
    use bytes::Bytes;
    use http_body_util::{BodyExt, Empty};
    use hyper_util::client::legacy::Client;
    use hyper_util::rt::TokioExecutor;

    let _ = rustls::crypto::ring::default_provider().install_default();
    let url = url.parse::<http::Uri>().unwrap();
    let https = hyper_rustls::HttpsConnectorBuilder::new()
        .with_native_roots()
        .expect("no native root CA certificates found")
        .https_only()
        .enable_http1()
        .build();

    let client: Client<_, Empty<Bytes>> = Client::builder(TokioExecutor::new()).build(https);

    async move {
        let response = client
            .get(url.clone())
            .await
            .map_err(|e| io_error("failed to receive response", e))?;
        let (parts, body) = response.into_parts();
        let body_bytes = body
            .collect()
            .await
            .map_err(|e| io_error("failed to collect response body", e))?
            .to_bytes();
        let response_str = String::from_utf8_lossy(&body_bytes).into_owned();
        if parts.status.is_success() {
            log::trace!("successful GET {url}");
        } else {
            log::warn!("failed GET {url} ({:?})", parts.status.canonical_reason())
        }
        Ok(Response::from_parts(parts, response_str))
    }
}
