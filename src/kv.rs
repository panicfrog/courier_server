use rocket::{Rocket, Build, fairing};
use sled;

pub async fn init_kv(rocket: Rocket<Build>) -> fairing::Result {
    match sled::open("kv") {
        Ok(db) => Ok(rocket.manage(db)),
        Err(e) => {
            error!("{}", e);
            Err(rocket)
        }
    }
}