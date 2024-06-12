#[macro_use]
extern crate rocket;

use anyhow::{Error, Result};
use rocket::fs::FileServer;


use tera::Tera;

mod www;
mod art;

#[tokio::main]
async fn main() -> Result<(), Error>  {


    let tera = Tera::new("templates/**/*").unwrap();

    let rocket = rocket::build().manage(www::Repositories::get())
        .manage(tera)
        .mount("/", routes![www::index])
        .mount("/static", FileServer::from("static"));
    
    rocket.launch().await?;
    
    Ok(())
}
