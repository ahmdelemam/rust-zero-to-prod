use actix_web::{web, App, HttpRequest, HttpServer, Responder};

async fn greet(req: HttpRequest) -> impl Responder {
let name = req.match_info().get("name").unwrap_or("World"); format!("Hello {}!", &name)
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
  HttpServer::new(|| { App::new()
        .route("/", web::get().to(greet)) //web::get() is a short-cut for Route::new().guard(guard::Get())
        .route("/{name}", web::get().to(greet))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
