fn main() {
    show_openssl_version();
    reqest_to_example_com();
}

fn show_openssl_version() {
    let version = openssl::version::version();
    println!("OpenSSL version: {}", version);
}

fn reqest_to_example_com() {
    let client = reqwest::blocking::Client::new();
    match client.get("https://example.com").send() {
        Ok(res) => {
            println!("Status: {}", res.status());
            println!("Headers:\n{:#?}", res.headers());
            match res.text() {
                Ok(body) => {
                    println!("Body:\n{}", body);
                }
                Err(err) => {
                    println!("Error: {}", err);
                }
            }
        }
        Err(err) => {
            println!("Error: {}", err);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::show_openssl_version;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn openssl_version() {
        show_openssl_version();
    }
}
