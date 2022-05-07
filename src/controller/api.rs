use actix_web::{get, HttpResponse, Responder};

use crate::MLTT;
use mobc_redis::redis;
use mobc_redis::redis::aio::Connection;
use rbatis::core::db::DBExecResult;
use rbatis::crud::{CRUDMut, CRUD};
use rbatis::crud_table;
use rbatis::executor::RbatisExecutor;
use rbatis::html_sql;
use rbatis::push_index;
use rbatis::py_sql;
use rbatis::rb_html;
use rbatis::rb_py;
use rbatis::rbatis::Rbatis;
use rbatis::{Page, PageRequest};

#[crud_table(formats_pg:"id:{}::uuid")]
#[derive(Clone, Debug)]
pub struct TMediaScreenshot {
    pub id: Option<u64>,
    pub name: Option<String>,
    pub pull_url: Option<String>,
    pub server_name: Option<String>,
    pub status: Option<u64>,
    pub created_at: Option<rbatis::DateTimeNative>,
}

#[crud_table(formats_pg:"id:{}::uuid")]
#[derive(Clone, Debug)]
pub struct TMediaScreenshotVo {
    pub id: Option<u64>,
    pub name: Option<String>,
    pub phone: Option<String>,
    pub pull_url: Option<String>,
    pub server_name: Option<String>,
    pub status: Option<u64>,
    pub created_at: Option<rbatis::DateTimeNative>,
}
/*impl Default for TMediaScreenshot {
    fn default() -> Self {
        TMediaScreenshot {
            id: None,
            name: None,
            pull_url: None,
            server_name: None,
            status: None,
            created_at: None,
        }
    }
}*/

#[get("/api/htmlSql")]
async fn get_html_sql() -> impl Responder {
    let exec = insert_media(
        &mut MLTT.rb.as_executor(),
        &TMediaScreenshot {
            id: Some(0),
            name: Some(rbatis::Uuid::new().to_string()),
            pull_url: Some("www.baidu.com".to_string()),
            server_name: Some(rbatis::Uuid::new().to_string()),
            status: Some(1),
            created_at: Option::from(rbatis::DateTimeNative::now()),
        },
    )
    .await
    .unwrap();
    println!("rows_affected = {}", exec.rows_affected);
    println!("last_insert_id = {}", exec.last_insert_id.unwrap());

    update_by_id(
        &mut MLTT.rb.as_executor(),
        &TMediaScreenshot {
            id: Some(2),
            name: Some("babel".to_string()),
            pull_url: None,
            server_name: None,
            status: None,
            created_at: None,
        },
    )
    .await
    .unwrap();

    let data: Page<TMediaScreenshotVo> = select_by_condition(
        &mut MLTT.rb.as_executor(),
        &PageRequest::new(1, 10),
        "test_updated",
        &rbatis::DateTimeNative::now(),
    )
    .await
    .unwrap();

    HttpResponse::Ok()
        .content_type("application/json;charset=UTF-8")
        .body(serde_json::json!(data).to_string())
}

#[html_sql("src/mapper/example.html")]
async fn insert_media(
    rb: &mut RbatisExecutor<'_, '_>,
    model: &TMediaScreenshot,
) -> rbatis::core::Result<DBExecResult> {
    todo!()
}

#[html_sql("src/mapper/example.html")]
async fn update_by_id(
    rb: &mut RbatisExecutor<'_, '_>,
    model: &TMediaScreenshot,
) -> rbatis::core::Result<DBExecResult> {
    todo!()
}

#[html_sql("src/mapper/example.html")]
async fn select_by_condition(
    rb: &mut RbatisExecutor<'_, '_>,
    page_req: &PageRequest,
    name: &str,
    dt: &rbatis::DateTimeNative,
) -> Page<TMediaScreenshotVo> {
    todo!()
}

#[get("/api/getValue")]
async fn get_value() -> impl Responder {
    let mut conn = MLTT.redis.get().await.unwrap();
    let s: String = redis::cmd("GET")
        .arg("aa")
        .query_async(&mut conn as &mut Connection)
        .await
        .unwrap_or("".to_string());
    //not_std::Cout << format!("MLTT.redis.aa.value = {}", s) << not_std::Endl;
    HttpResponse::Ok()
        .content_type("application/json;charset=UTF-8")
        .body(s)
}

#[get("/api/pySql")]
async fn get_py_sql() -> impl Responder {
    let data = join_select(&MLTT.rb, "test_updated").await.unwrap();
    //println!("{:?}", data);
    //println!("{}", format!("MLTT.redis.value = {}",s));
    HttpResponse::Ok()
        .content_type("application/json;charset=UTF-8")
        .body(serde_json::json!(data).to_string())
}

#[py_sql(
    "SELECT * FROM t_media_screenshot
           WHERE 1 = 1
           if  name != '':
               AND name = #{name}"
)]
async fn join_select(rbatis: &Rbatis, name: &str) -> Option<Vec<TMediaScreenshot>> {
    todo!()
}

#[get("/api/list")]
async fn list() -> impl Responder {
    //let _v = MLTT.rb.fetch_list::<TMediaScreenshot>().await.unwrap();
    forget_commit().await;

    let mut conn = MLTT.redis.get().await.unwrap();
    // let s: String = redis::cmd("SET").arg("a").arg(1).query_async(&mut conn as &mut Connection).await.unwrap();
    let s: String = redis::cmd("GET")
        .arg("aa")
        .query_async(&mut conn as &mut Connection)
        .await
        .unwrap_or("".to_string());

    println!("{}", format!("MLTT.redis.value = {}", s));

    let req = PageRequest::new(1, 20); //分页请求，页码，条数
    let wraper = MLTT.rb.new_wrapper().eq("status", 1);
    let data: Page<TMediaScreenshot> = MLTT.rb.fetch_page_by_wrapper(wraper, &req).await.unwrap();
    println!("{}", serde_json::to_string(&data).unwrap());
    HttpResponse::Ok()
        .content_type("application/json;charset=UTF-8")
        .body(serde_json::json!(data).to_string())
}

#[inline(always)]
pub async fn forget_commit() -> rbatis::core::Result<()> {
    // tx will be commit.when func end
    let mut tx = MLTT
        .rb
        .acquire_begin()
        .await?
        .defer_async(|mut tx1| async move {
            if !tx1.is_done() {
                tx1.rollback().await;
                println!("tx rollback success!");
            } else {
                println!("don't need rollback!");
            }
        });
    /* let sql = format!("update t_media_screenshot set name = {} where id = {}", "'xxx-zzz'", 2);
    println!("{}", sql.as_str());
    let _v = tx.exec_sql(sql.as_str()).await;*/
    let rows_affected = tx
        .update_by_column::<TMediaScreenshot>(
            "id",
            &TMediaScreenshot {
                id: Some(2),
                name: Some("test_updated".to_string()),
                pull_url: None,
                server_name: None,
                status: None,
                created_at: None,
            },
        )
        .await;
    tx.commit().await; //if commit, print 'don't need rollback!' ,if not,print 'tx rollback success!'
    println!("rows_affected = {:?}", rows_affected.unwrap());
    return Ok(());
}
