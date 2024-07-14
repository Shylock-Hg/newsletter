use actix_web::dev::Server;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("world");
    format!("Hello {}.", name)
}

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub async fn run(address: &str) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            // match this first instead of greet
            .route("/health_check", web::get().to(health_check))
            .route("/{name}", web::get().to(greet))
    })
    .bind(address)?
    .run();
    return Ok(server);
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
