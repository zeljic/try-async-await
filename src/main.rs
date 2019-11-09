use actix_web::{web, App, Error, HttpResponse, HttpServer};
use futures::Future;
use futures03::future::{FutureExt, TryFutureExt};

pub async fn index_fut() -> Result<HttpResponse, actix_web::Error> {
    Ok(HttpResponse::Ok().body("zdravo svete!"))
}

pub fn index() -> impl Future<Item = HttpResponse, Error = Error> {
    index_fut().boxed().compat()
}

fn main() {
    HttpServer::new(|| App::new().service(web::resource("/").route(web::to_async(index))))
        .bind("127.0.0.1:1337")
        .expect("oups bind")
        .run()
        .expect("oups run");
}
