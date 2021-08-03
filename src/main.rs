#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_sync_db_pools;
extern crate sqlx;

mod router;
mod db;

use rocket::fairing::AdHoc;

#[rocket::main]
async fn main() -> std::result::Result<(), rocket::Error> {
    rocket::build()
        .attach(AdHoc::try_on_ignite("SQLx database", db::init_db))
        .mount("/kv", routes![router::get_value, router::set_value])
        .launch()
        .await
}
