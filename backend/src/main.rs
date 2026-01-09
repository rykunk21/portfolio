#[macro_use]
extern crate rocket;

use rocket::{fs::FileServer, Config};
use std::{
    env,
    net::{IpAddr, Ipv4Addr},
};

mod routes;
use routes::init;

mod db;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    // Use Railway's dynamic PORT
    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "80".to_string())
        .parse()
        .expect("PORT must be a number");

    // Initialize DB first (async)
    init().await.expect("Failed to startup db...");

    // Build Rocket with config
    let _rocket = rocket::custom(Config {
        port,
        address: IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
        ..Config::default()
    })
    .mount("/", FileServer::from("./frontend/dist"))
    .mount(
        "/api",
        routes![routes::create, routes::read, routes::update, routes::delete],
    )
    .launch()
    .await?;

    Ok(())
}
