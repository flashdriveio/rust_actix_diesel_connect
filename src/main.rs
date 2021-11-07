use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn home_get() -> impl Responder {
    HttpResponse::Ok().body("<p style='background: #565656'><img src='https://actix.rs/img/logo-large.png' width='250'></p>")
}

#[post("/")]
async fn home_post(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(home_get)
            .service(home_post)
    })
    .bind("127.0.0.1:5003")?
    .run()
    .await
}
