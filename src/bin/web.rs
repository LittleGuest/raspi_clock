use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

use raspi_clock::error::Err;
use raspi_clock::store::Store;

#[get("/")]
async fn index() -> impl Responder {
    let html = include_str!("../../max7219.html");
    HttpResponse::Ok().content_type("text/html").body(html)
}

#[get("/api/v1/design")]
async fn get() -> impl Responder {
    match Store::get() {
        Ok(data) => HttpResponse::Ok().json2(&data),
        Err(e) => HttpResponse::InternalServerError().body(format!("获取失败：{e}")),
    }
}

#[post("/api/v1/design/dispatch")]
async fn dispatch(data: web::Json<Vec<[u8; 8]>>) -> impl Responder {
    let data = data.0;
    match Store::insert(&data.iter().map(|d| d.to_vec()).collect::<Vec<_>>()) {
        Ok(_) => match raspi_clock::dispatch_data(&data) {
            Ok(_) => HttpResponse::Ok().finish(),
            Err(e) => HttpResponse::InternalServerError().body(format!("保存出错：{e}")),
        },
        Err(e) => HttpResponse::InternalServerError().body(format!("保存出错：{e}")),
    }
}

#[actix_web::main]
async fn main() -> anyhow::Result<(), Err> {
    HttpServer::new(|| App::new().service(index).service(get).service(dispatch))
        .bind(("127.0.0.1", 7219))?
        .run()
        .await?;
    Ok(())
}
