#[cfg(test)]
mod tests {
    use newsletter::run;

    const MOCK_SERVER_HOST: &str = "127.0.0.1:8080";

    async fn mock_app(
    ) -> Result<tokio::task::JoinHandle<Result<(), tokio::io::Error>>, std::io::Error> {
        let server = run(MOCK_SERVER_HOST).await?;
        Ok(tokio::spawn(server))
    }

    #[tokio::test]
    async fn test_health_check() {
        let handle = mock_app().await.unwrap();

        let client = reqwest::Client::new();

        let resp = client
            .get(format!("http://{}/{}", MOCK_SERVER_HOST, "health_check"))
            .send()
            .await
            .unwrap();

        assert!(resp.status().is_success());

        handle.abort();

        let _ = handle.await;
    }
}
