fn main() {
    show_openssl_version();
}

fn show_openssl_version() {
    let version = openssl::version::version();
    println!("OpenSSL version: {}", version);
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
