#[allow(unused)]
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use std::sync::Mutex;
struct AppState {
    counter: Mutex<i32>,
}

#[get("/")]
async fn index(data: web::Data<AppState>) -> impl Responder {
    let mut counter = data.counter.lock().unwrap();
    *counter += 1;
    format!("Ne value: {}", counter)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let counter = web::Data::new(AppState {
        counter: Mutex::new(0),
    });
    HttpServer::new(move || {
            App::new().app_data(counter.clone()).service(index)
        })
        .bind(("127.0.0.1", 5000))?
        .run()
        .await
}
