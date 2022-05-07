use fast_log::config::Config;
use fast_log::consts::LogSize;
use fast_log::plugin::file_split::RollingType;
use fast_log::plugin::packer::LogPacker;
use lazy_static::lazy_static;
use mobc::Pool;
use mobc_redis::{redis, RedisConnectionManager};
use rbatis::rbatis::Rbatis;
use std::env;

/*
    华烨带领天宫集团攻打天使之城，
    并用黑洞引擎在梅洛天庭附近制造了微型黑洞并预留了1000小时。
    为从内部了解黑洞并瓦解它，
    彻底破解黑洞引擎，天基王鹤熙亲自进入黑洞，
    其身体被分解为神圣原子并与宇宙融为一体，但意识仍存，还同早已陨落的凯莎发生了交流
*/
pub struct Mltt {
    pub rb: Rbatis,
    pub redis: mobc::Pool<RedisConnectionManager>,
}

// init global pool
lazy_static! {
    pub static ref MLTT: Mltt = Mltt {
        rb: Rbatis::new(),
        redis: Pool::builder()
            .max_open(100)
            .build(RedisConnectionManager::new(
                redis::Client::open(env::var("REDIS_URL").unwrap()).unwrap()
            ))
    };
}

impl Mltt {
    pub async fn initialization() {
        //log
        //fast_log::init_log("requests.log", log::Level::Info, None, true);
        fast_log::init(Config::new().console().file_split(
            "logs/",
            LogSize::MB(5),
            RollingType::All,
            LogPacker {},
        ))
        .unwrap();
        MLTT.rb
            .link(env::var("DATABASE_URL").unwrap().as_mut_str())
            .await;
    }
}
