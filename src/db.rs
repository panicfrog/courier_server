use rocket::{Rocket, Build, fairing};
use rocket::serde::{Serialize, Deserialize, json::Json};

pub type Db = sqlx::MySqlPool;

pub async fn init_db(rocket: Rocket<Build>) -> fairing::Result {
    use rocket_sync_db_pools::Config;
    let config = match Config::from("kv_db", &rocket) {
        Ok(config) => config,
        Err(e) => {
            error!("Failed to read SQLx config: {}", e);
            return Err(rocket);
        }
    };

    let db = match sqlx::mysql::MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&config.url).await {
        Ok(db) => db,
        Err(e) => {
            error!("Failed to connect to SQLx database: {}", e);
            return Err(rocket);
        }
    };


    if let Err(e) = sqlx::migrate!().run(&db).await {
        error!("Failed to initialize SQLx database: {}", e);
        return Err(rocket);
    }

    Ok(rocket.manage(db))
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct KV {
    // #[serde(skip_serializing, skip_serializing_if = "Option::is_none")]
    // id: Option<i64>,
    pub key: String,
    pub value: String,
}

pub async fn insert_kv(db: &Db, key: &str, value: &str) -> Result<(), sqlx::Error> {
    sqlx::query!("INSERT INTO kv (`key`, `value`) VALUES (?, ?)", key, value)
        .execute(db)
        .await?;
    Ok(())
}

pub async fn query_latest_kv(db: &Db, key: &str) -> Result<KV, sqlx::Error> {
    match sqlx::query!("SELECT `key`, `value` FROM kv WHERE `key` = ? ORDER BY create_at DESC LIMIT 1", key)
        .fetch_one(db).await {
        Ok(r) => Ok(KV { key: r.key, value: r.value }),
        Err(e) => Err(e)
    }
}


