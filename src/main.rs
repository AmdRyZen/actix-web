#![allow(unused_must_use)]

mod application;
mod controller;

use crate::application::{not_std, MLTT};
use crate::controller::*;
use dotenv::dotenv;
use std::env;
use std::sync::Arc;
use std::time::Duration;

use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};
use mimalloc::MiMalloc;
use nacos_rust_client::client::config_client::{ConfigDefaultListener, ConfigKey};
use nacos_rust_client::client::naming_client::{Instance, NamingClient, QueryInstanceListParams};
use nacos_rust_client::client::{ConfigClient, HostInfo};

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

#[tokio::main]
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

    // nacos
    let host = HostInfo::parse("127.0.0.1:8848");

    // 配置中心
    let config_client = ConfigClient::new(host.clone(), String::new());
    let key = ConfigKey::new("actix_web-dev", "dev", "");
    let c = Box::new(ConfigDefaultListener::new(
        key.clone(),
        Arc::new(|s| {
            //字符串反序列化为对象，如:serde_json::from_str::<T>(s)
            Some(s.to_owned())
        }),
    ));
    config_client.set_config(&key, "1234").await.unwrap();
    let value = config_client
        .get_config(&key)
        .await
        .unwrap_or("".to_string());
    println!("value: {:?}", value.as_str());
    //监听
    config_client.subscribe(c.clone()).await;
    //从监听对象中获取
    println!("actix_web value: {:?}", c.get_value());

    // 服务中心
    let client = NamingClient::new(host.clone(), "".to_owned());
    let ip = local_ipaddress::get().unwrap();
    for i in 0..2 {
        let port = 10000 + i;
        let instance = Instance::new(&ip, port, "actix_web", "", "", "", None);
        //注册
        client.register(instance);
    }
    let client2 = client.clone();
    tokio::spawn(async move {
        query_params(client2.clone()).await;
    });
    tokio::signal::ctrl_c()
        .await
        .expect("failed to listen for event");

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
    .workers(16)
    .keep_alive(Duration::from_secs(60))
    .run()
    .await
}

async fn query_params(client: Arc<NamingClient>) -> anyhow::Result<()> {
    let params = QueryInstanceListParams::new("", "", "actix_web", None, true);
    // 模拟每秒钟获取一次实例
    loop {
        //查询并按权重随机选择其中一个实例
        match client.select_instance(params.clone()).await {
            Ok(instances) => {
                println!("select instance {}:{}", &instances.ip, &instances.port);
            }
            Err(e) => {
                println!("select_instance error {:?}", &e)
            }
        }
        tokio::time::sleep(Duration::from_millis(10000)).await;
    }
}
