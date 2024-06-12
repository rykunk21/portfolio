use reqwest;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::art;


use rocket::{get, State};
use rocket_contrib::templates::Template;
use std::collections::HashMap;
use tera::{Tera, Context, Error};

#[get("/")]
pub async fn index(tera: State<'_, Tera>) -> Result<String, Error> {
    // Populate context data to pass to the template
    let mut context = Context::new();
    let repos = Repositories::get(); // Retrieve repos here
    context.insert("repos", &repos); // Insert repos into the context

    // Render the index template with the context data
    let rendered_template = tera.render("index.html", &context)?;

    Ok(rendered_template)
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Repo {
    name: String,
    html_url: String,
    description: Option<String>, // description might be null
    pixel_sequence: String,      // Placeholder for generated pixel sequence
}

#[derive(Default)]
pub struct Repositories {
    pub repos: Vec<Repo>,
}

impl Repositories {
    
    pub async fn get() -> Result<Repositories, Box<dyn std::error::Error + Send + Sync>> {
        let username = "rykunk21"; // Replace with your GitHub username
        let url = format!("https://api.github.com/users/{}/repos", username);

        let response = reqwest::get(&url).await?;

        let body = response.text().await?;

        let json_data: Value = serde_json::from_str(&body)?;

        let repos: Vec<Repo> = json_data
            .as_array()
            .ok_or("Expected JSON array but received something else")?
            .iter()
            .map(|data| Repo {
                name: data["name"].as_str().unwrap_or_default().to_string(),
                html_url: data["html_url"].as_str().unwrap_or_default().to_string(),
                description: data["description"].as_str().map_or_else(
                    || Some("No description available".to_string()),
                    |s| Some(s.to_string()),
                ),
                pixel_sequence: art::generate_pixel_sequence(),
            })
            .collect();

        Ok(Repositories { repos })
    }
    
}
