use rocket::tokio::time::{sleep, Duration};
use rocket::serde::{ Deserialize, Serialize, json::Json };
use rocket::State;
use rocket::response::status::Created;

use super::db::{ Db, KV, query_latest_kv, insert_kv  };

type Result<T, E = rocket::response::Debug<sqlx::Error>> = std::result::Result<T, E>;

// TODO: 返回值用宏来包装一下，包装成 { code: xxx, message: xxx, data: xxx }
#[get("/getValue/<key>")]
pub async fn get_value(db: &State<Db>, key: String) -> Result<Json<KV>> {
    let kv =  query_latest_kv(&**db,key.as_str() ).await?;
    Ok(Json(kv))
}

#[post("/setValue", format = "json", data = "<data>")]
pub async fn set_value(db: &State<Db>, data: Json<KV>) -> Result<Created<Json<KV>>> {
    let (key, value) = (data.key.as_str(), data.value.as_str());
    let _ = insert_kv(&**db, key, value).await?;
    Ok(Created::new("/").body(data))
}
