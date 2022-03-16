#[allow(unused)]
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

struct AppState {
    name: String
}

#[get("/")]
async fn index (data: web::Data<AppState>) -> impl Responder {
    format!("{}", &data.name)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        .app_data(web::Data::new(
            AppState {
                name: "A simple web app in actix".to_string()
            }
        ))
        .service(index)
    })
    .bind(("127.0.0.1", 5000))?
    .run()
    .await
}