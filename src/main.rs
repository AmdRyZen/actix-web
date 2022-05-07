#![allow(unused_must_use)]

mod application;
mod controller;

use crate::application::{not_std, MLTT};
use crate::controller::*;
use dotenv::dotenv;
use std::env;
use std::time::Duration;

use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};
use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    // RUSTFLAGS="-C target-cpu=native" cargo build --release
    // cargo build --release --target=aarch64-apple-darwin
    // cargo expand --bin actix-web // 代码展开
    // cargo rustc -- -Zself-profile 生成的跟踪文件
    // RUSTFLAGS=-Zprint-type-sizes cargo build --release  检测缓存对齐

    //initialization
    application::Mltt::initialization().await;

    // envs
    let env = env::var("ENV").expect("ENV is not set in .env file");
    not_std::Cout << "env = " << env << not_std::Endl;
    not_std::Cout << "HttpServer" << " run " << "success!" << not_std::Endl;

    // Multiply x by 6 using shifts and adds
    /* let mut x: u64 = 4;
    unsafe {
        asm!(
        "mov {tmp}, {x}",
        "shl {tmp}, 1",
        "shl {x}, 2",
        "add {x}, {tmp}",
        x = inout(reg) x,
        tmp = out(reg) _,
        );
    }
    assert_eq!(x, 4 * 6);*/

    HttpServer::new(move || {
        App::new()
            .service(hello)
            .service(echo)
            .service(list)
            .service(get_value)
            .service(get_py_sql)
            .service(get_html_sql)
            .service(curl_post)
    })
    .bind("127.0.0.1:9090")?
    .workers(12)
    .keep_alive(Duration::from_secs(5))
    .run()
    .await
}
