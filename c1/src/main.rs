fn main() {
    show_openssl_version();
    request_to_example_com();
    let digest = get_digest();
    println!("Digest: {digest}");
    match tokio::runtime::Runtime::new() {
        Ok(rt) => {
            if let Err(err) = rt.block_on(tokio_example()) {
                println!("Error: {err}");
            }
        }
        Err(err) => {
            println!("Error: {err}");
        }
    }
}

fn show_openssl_version() {
    let version = openssl::version::version();
    println!("OpenSSL version: {version}");
}

fn get_digest() -> String {
    use ring::digest;
    let bin = digest::digest(&digest::SHA256, b"hello, world");
    hex::encode(bin)
}

async fn tokio_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, World!");
    Ok(())
}

fn request_to_example_com() {
    let client = reqwest::blocking::Client::new();
    match client.get("https://example.com").send() {
        Ok(res) => {
            println!("Status: {}", res.status());
            println!("Headers:\n{:#?}", res.headers());
            match res.text() {
                Ok(body) => {
                    println!("Body:\n{body}");
                }
                Err(err) => {
                    println!("Error: {err}");
                }
            }
        }
        Err(err) => {
            println!("Error: {err}");
        }
    }
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
}
