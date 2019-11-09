use actix_web::{web, App, Error, HttpResponse, HttpServer};

use futures03::future::{FutureExt, TryFutureExt};

use futures::Future;

pub async fn index() -> Result<HttpResponse, actix_web::Error> {
    Ok(HttpResponse::Ok().body("zdravo svete!"))
}

pub fn index_fut() -> impl Future<Item = HttpResponse, Error = Error> {
    index().boxed().compat()
}

fn main() {
    HttpServer::new(|| App::new().service(web::resource("/").route(web::to_async(index_fut))))
        .bind("127.0.0.1:1331")
        .expect("oups bind")
        .run()
        .expect("oups run");
}
