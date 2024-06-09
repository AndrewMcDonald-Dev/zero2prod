use std::net::TcpListener;

use zero2prod::startup::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Could not bind on random port.");

    let port = listener.local_addr().unwrap().port();
    println!("Listening on address: http://127.0.0.1:{}", port);
    run(listener)?.await
}
