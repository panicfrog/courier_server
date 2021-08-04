#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_sync_db_pools;
extern crate sqlx;
extern crate sled;

mod router;
mod db;
mod kv;

use rocket::fairing::AdHoc;

#[rocket::main]
async fn main() -> std::result::Result<(), rocket::Error> {
    rocket::build()
        .attach(AdHoc::try_on_ignite("mysql database", db::init_db))
        .attach(AdHoc::try_on_ignite("sled database", kv::init_kv))
        .register("/", catchers![router::internal_error, router::not_found])
        .mount("/kv", routes![router::get_value, router::set_value])
        .launch()
        .await
}
