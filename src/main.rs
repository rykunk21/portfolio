#[macro_use]
extern crate rocket;

use reqwest;
use serde::{Serialize, Deserialize};
use serde_json::Value;
use anyhow::{Error, Result, Context};
use rocket::{response::content, fs::FileServer};
use std::sync::Mutex;

use tera::Tera;

mod art;

#[derive(Serialize, Deserialize, Debug)]
struct Repo {
    name: String,
    html_url: String,
    description: Option<String>, // description might be null
    pixel_sequence: String,      // Placeholder for generated pixel sequence
}


// Shared state for repositories
#[derive(Default)]
struct Repositories {
    repos: Mutex<Vec<Repo>>,
}


#[get("/")]
async fn index(tera: &rocket::State<Tera>, repos: &rocket::State<Repositories>) -> content::RawHtml<String> {
    let repos_data = repos.repos.lock().unwrap();
    let mut context = tera::Context::new();
    context.insert("repos", &*repos_data);

    match tera.render("index.html", &context) {
        Ok(html) => content::RawHtml(html),
        Err(e) => {
            eprintln!("Failed to render template: {}", e);
            content::RawHtml(String::new())
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Error>  {
    let username = "rykunk21"; // Replace with your GitHub username
    let url = format!("https://api.github.com/users/{}/repos", username);

    let response = reqwest::Client::new()
        .get(&url)
        .header("User-Agent", "Portfolio-Query")
        .send()
        .await
        .context("Failed to send request")?;

    let body = response.text().await
        .context("Failed to read response text")?;

    // Parse the response body into JSON
    let json_data: Value = serde_json::from_str(&body)
        .context("Failed to parse JSON response")?;
    
    
    // Map the data to our Repo struct, generating pixel sequences
    // Iterate over the JSON data and construct Repo structs
    let repos: Vec<Repo> = json_data.as_array().unwrap_or_else(|| {
        panic!("Expected JSON array but received something else")
    }).iter().map(|data| {
        Repo {
            name: data["name"].as_str().unwrap_or_default().to_string(),
            html_url: data["html_url"].as_str().unwrap_or_default().to_string(),
            description: data["description"].as_str()
                .map_or_else(|| Some("No description available".to_string()), |s| Some(s.to_string())),
            pixel_sequence: art::generate_pixel_sequence(),
        }
    }).collect();

    let tera = Tera::new("templates/**/*").unwrap();

    let rocket = rocket::build().manage(Repositories {
        repos: Mutex::new(repos),
    }).manage(tera)
      .mount("/", routes![index])
      .mount("/static", FileServer::from("static"));
    rocket.launch().await?;
        

    Ok(())
}
