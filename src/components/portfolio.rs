use reqwest::Error;
use serde::Deserialize;
use rand::Rng;
use yew::prelude::*;

use wasm_bindgen_futures::spawn_local;

#[derive(Clone, Properties, PartialEq, Deserialize)]
pub struct RepoProps {
    pub id: u32,
    pub name: String,
    pub description: Option<String>,
    pub html_url: String,
}

#[derive(Default)]
pub struct Repositories {
    pub repos: Vec<RepoProps>,
}

impl Repositories {
    pub async fn get() -> Result<Repositories, Error> {
        let username = "rykunk21";

        let url = format!("https://api.github.com/users/{}/repos", username);
        let response = reqwest::get(&url).await?;
        let repos: Vec<RepoProps> = response.json().await?;

        Ok(Repositories { repos })
    }
}

#[function_component(Grid)]
pub fn grid() -> Html {
    // Generate a 4x4 grid of random colors
    let mut rng = rand::thread_rng();
    let colors: Vec<String> = (0..16)
        .map(|_| {
            format!(
                "rgb({}, {}, {})",
                rng.gen_range(0..256),
                rng.gen_range(0..256),
                rng.gen_range(0..256)
            )
        })
        .collect();

    // Create grid items
    let grid_items = colors.iter().map(|color| {
        html! {
            <div class="grid-item" style={format!("background-color: {}", color)}></div>
        }
    });

    // Render the grid
    html! {
        <div class="grid-container">
            { for grid_items }
        </div>
    }
}


#[function_component(Repo)]
pub fn repo(props: &RepoProps) -> Html {
    html! {
        <a href={ props.html_url.clone() }>
            <div class="card">
                
                <h2>{ &props.name }</h2>
                { if let Some(description) = &props.description {
                    html! { <h3>{ description }</h3> }
                } else {
                    html! {} // Render nothing if description is None
                }}
                
            </div>        
        </a>
    }
}


#[function_component(Portfolio)]
pub fn portfolio() -> Html {
    let repos = use_state(|| vec![]);
    let error = use_state(|| None);

    {
        let repos = repos.clone();
        let error = error.clone();
        use_effect(move || {
            let repos = repos.clone();
            let error = error.clone();
            spawn_local(async move {
                match Repositories::get().await {
                    Ok(result) => repos.set(result.repos),
                    Err(err) => error.set(Some(err.to_string())),
                }
            });
            || ()
        });
    }

    html! {
        <>
            { if !repos.is_empty() {
                html! {
                    <div class="container">
                        { for repos.iter().map(|repo| html! { <Repo key={repo.id} ..repo.clone() /> }) }
                    </div>
                }
            } else if let Some(err) = error.as_ref() {
                html! { <p>{ err }</p> }
            } else {
                html! { <p>{ "No data" }</p> }
            }}
        </>
    }
}
