#[macro_use]
extern crate rocket;
use std::{
    env,
    net::{IpAddr, Ipv4Addr},
};

use rocket::{fs::FileServer, Config};

mod routes;
use routes::init;

mod db;

#[launch]
async fn rocket() -> _ {
    let port: u16 = env::var("PORT").unwrap().parse().unwrap();

    init().await.expect("Failed to startup db...");
    rocket::build()
        .configure(rocket::Config {
            port,
            address: IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),

            ..Config::default()
        })
        .mount("/", FileServer::from("./frontend/dist"))
        .mount(
            "/api",
            routes![routes::create, routes::read, routes::update, routes::delete],
        )
}
