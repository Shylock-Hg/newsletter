use newsletter::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let server = run("127.0.0.1:8080").await.unwrap();
    server.await
}
