use reqwest::header::{HeaderMap, HeaderName};
use tracing::{debug, error, info};
use tracing_subscriber::EnvFilter;

fn main() {
    init_tracing();
    show_openssl_version();
    if let Err(err) = request_to_example_com() {
        error!(%err, "request to example.com failed");
    }
    let digest = get_digest();
    info!(digest, "computed digest");
    match tokio::runtime::Runtime::new() {
        Ok(rt) => {
            if let Err(err) = rt.block_on(tokio_example()) {
                error!(%err, "tokio example failed");
            }
        }
        Err(err) => {
            error!(%err, "failed to create tokio runtime");
        }
    }
}

fn init_tracing() {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    tracing_subscriber::fmt()
        .with_env_filter(env_filter)
        .with_writer(std::io::stderr)
        .init();
}

fn show_openssl_version() {
    let version = openssl::version::version();
    info!(version, "detected OpenSSL version");
}

fn get_digest() -> String {
    use ring::digest;
    let bin = digest::digest(&digest::SHA256, b"hello, world");
    hex::encode(bin)
}

async fn tokio_example() -> Result<(), Box<dyn std::error::Error>> {
    info!("hello from async example");
    Ok(())
}

fn request_to_example_com() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::new();
    let res = client.get("https://example.com").send()?;
    info!(status = %res.status(), "received response");
    debug!(headers = ?redacted_headers(res.headers()), "received response headers");
    let body = res.text()?;
    debug!(body_bytes = body.len(), "received response body");
    Ok(())
}

fn redacted_headers(headers: &HeaderMap) -> Vec<(String, String)> {
    headers
        .iter()
        .map(|(name, value)| {
            let value = if is_sensitive_header(name) {
                "<redacted>".to_owned()
            } else {
                value
                    .to_str()
                    .map_or_else(|_| "<non-utf8>".to_owned(), str::to_owned)
            };
            (name.as_str().to_owned(), value)
        })
        .collect()
}

fn is_sensitive_header(name: &HeaderName) -> bool {
    matches!(
        name.as_str(),
        "authorization"
            | "cookie"
            | "set-cookie"
            | "proxy-authorization"
            | "x-api-key"
            | "x-auth-token"
            | "access-token"
            | "refresh-token"
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::show_openssl_version;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn openssl_version() {
        show_openssl_version();
    }

    #[test]
    fn test_digest() {
        let expected_hex = "09ca7e4eaa6e8ae9c7d261167129184883644d07dfba7cbfbc4c8a2e08360d5b";
        let actual = get_digest();
        assert_eq!(expected_hex, actual);
    }

    #[test]
    fn redacts_sensitive_header_values() {
        let mut headers = HeaderMap::new();
        headers.insert("authorization", "Bearer secret".parse().unwrap());
        headers.insert("content-type", "text/html".parse().unwrap());

        let redacted = redacted_headers(&headers);

        assert_eq!(
            redacted
                .iter()
                .find(|(name, _)| name == "authorization")
                .map(|(_, value)| value),
            Some(&"<redacted>".to_owned())
        );
        assert_eq!(
            redacted
                .iter()
                .find(|(name, _)| name == "content-type")
                .map(|(_, value)| value),
            Some(&"text/html".to_owned())
        );
    }
}
