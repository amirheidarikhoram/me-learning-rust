#![allow(unused)]
use actix_web::guard;
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

#[get("/list")]
async fn list_users() -> impl Responder {
    format!("Nothing for now")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let counter = web::Data::new(AppState {
        counter: Mutex::new(0),
    });

    // let app = App::new()
    // .app_data(counter.clone())
    // .service(index)
    // .service(users_scope);

    HttpServer::new(move || {
        App::new()
            .configure(first_level_config)
            .app_data(counter.clone())
            .service(index)
            // you can not extract this service as a variable and then use it here becauses scope does not impl clone
            .service(
                web::scope("/users")
                    // // lets add a guard for this scope
                    // .guard(guard::Header("ContentType", "application/json"))
                    .service(list_users)
                    .configure(scoped_config),
            )
    })
    .bind(("127.0.0.1", 5000))?
    .run()
    .await
}

fn scoped_config(config: &mut web::ServiceConfig) {
    config.service(
        web::resource("/resource")
            .route(web::get().to(|| async { HttpResponse::Ok().body("body is ok") })),
    );
}

fn first_level_config(config: &mut web::ServiceConfig) {
    config.service(
        web::resource("/another")
            .route(web::get().to(|| async { HttpResponse::Ok().body(" you got another body") })),
    );
}
