fn main() {
    let version = openssl::version::version();
    println!("OpenSSL version: {}", version);
}
