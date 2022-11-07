#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;

use rocket::fairing::AdHoc;

#[derive(Debug)]
pub struct EnvConf {
    appid: String,
    secret: String,
}

mod db;
mod routes;
use crate::routes::{index, wechat, meeting};

fn main() {
    rocket::ignite()
        .manage(db::connect())
        .mount(
        "/",
        routes![
            index::index,
            wechat::sign,
            meeting::area_list,
            meeting::room_list,
            meeting::room_detail,
            meeting::create,
            meeting::entry_list,
            meeting::entry_detail,
            meeting::cancel,
            meeting::checkin,
        ]
    )
    .attach(AdHoc::on_attach(
        "Config Printer",
        |rocket| {
            let appid = rocket.config().get_str("appid").unwrap().to_string();
            let secret = rocket.config().get_str("secret").unwrap().to_string();
            Ok(rocket.manage(EnvConf{appid, secret}))
        }
    ))
    .launch();
}
