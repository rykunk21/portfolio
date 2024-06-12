use serde::{Serialize, Deserialize};
use std::sync::Mutex;
use reqwest;


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
    pub async fn get_repos() -> Repositories {
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
    }
}


