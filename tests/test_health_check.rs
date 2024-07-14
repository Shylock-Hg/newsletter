#[cfg(test)]
mod tests {
    use newsletter::run;

    async fn mock_app() -> Result<(), std::io::Error> {
        let server = run().await?;
        tokio::spawn(server);
        Ok(())
    }

    const MOCK_SERVER_HOST: &str = "http://127.0.0.1:8080";

    #[tokio::test]
    async fn test_health_check() {
        mock_app().await.unwrap();

        let client = reqwest::Client::new();

        let resp = client
            .get(format!("{}/{}", MOCK_SERVER_HOST, "health_check"))
            .send()
            .await
            .unwrap();

        assert!(resp.status().is_success());
    }
}
