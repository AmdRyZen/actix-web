use actix_web::{get, HttpResponse, Responder};
use aes::cipher::BlockEncryptMut;
use cbc::cipher::block_padding::Pkcs7;
use cbc::cipher::KeyIvInit;
use feignhttp::post;
use feignhttp::ErrorKind;
use serde_json::{json, Value};

#[get("/aes/curlPost")]
async fn curl_post() -> impl Responder {
    type Aes128CbcEnc = cbc::Encryptor<aes::Aes128>;
    type _Aes128CbcDec = cbc::Decryptor<aes::Aes128>;
    let mut buf = [0u8; 128];
    let url = "http://127.0.0.1:9090/echo";
    let param = *b"xxxxxxxxxxxxxxxxxxxx";
    let aes128_key = *b"WuCO7COZ4Y26YpoF";
    let sign = *b"xxxxx";
    // aes
    let pt_len = param.len();
    buf[..pt_len].copy_from_slice(&param);
    let cipher = Aes128CbcEnc::new(&aes128_key.into(), &aes128_key.into());
    let aes_res = cipher
        .encrypt_padded_mut::<Pkcs7>(&mut buf, pt_len)
        .unwrap();
    let aes_param = base64::encode(aes_res).to_string();
    println!("aes_res = {:?}", &aes_param);

    //md5
    let digest = md5::compute(sign);
    println!("digest = {:?}", digest.to_owned());
    let md5_str = format!("{:x}", digest);
    println!("md5_str = {:}", md5_str);

    let john = json!({
        "channelId": "xxxxx",
        "timestamp": "1638263374",
        "param": &aes_param.as_str(),
        "sign": md5_str
    });
    let params = john.to_string();
    //let params = serde_json::json!(map).to_string();
    println!("发送数据 params = {:}", params);

    let client = reqwest::Client::new();
    let response = client.post(url).body(params).send().await.unwrap();

    let code = response.status().as_u16();
    println!("code = {:?}", code);
    if code == 200 {
        let data = response.json::<serde_json::value::Value>().await.unwrap();
        println!("rust-demo  response = {:?}", data);
    }

    // feign-http
    feign_http(john).await.unwrap();

    HttpResponse::Ok()
        .content_type("application/json;charset=UTF-8")
        .body(serde_json::json!(code).to_string())
}

async fn feign_http(john: Value) -> Result<(), Box<dyn std::error::Error>> {
    let data = aaa(john).await;
    match data {
        Err(err) => {
            // Status error.
            if err.is_status_error() {
                println!("status_error: {}", err);
            }
            if let ErrorKind::Status(status) = err.error_kind() {
                println!("status error code: {}", status.as_u16());

                if status.is_client_error() {
                    // Handle error.
                }
                if status.is_server_error() {
                    // Handle error.
                }
            }
        }
        _ => {
            println!("data: {:}", data.unwrap());
        }
    }
    Ok(())
}

#[post(url = "http://127.0.0.1:9090/echo")]
async fn aaa(#[body] params: Value) -> feignhttp::Result<String> {
    todo!()
}
