use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("world");
    format!("Hello {}.", name)
}

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            // match this first instead of greet
            .route("/health_check", web::get().to(health_check))
            .route("/{name}", web::get().to(greet))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_health_check() {
        let resp = health_check().await;
        assert!(resp.status().is_success());
    }
}
