use yew::prelude::*;
use reqwest::Error;
use serde::Deserialize;

use wasm_bindgen_futures::spawn_local;

#[derive(Properties, PartialEq, Deserialize)]
pub struct RepoProps {
    name: String,
    html_url: String,
    description: Option<String>, // description might be null
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

#[function_component(Repo)]
pub fn repo(props: &RepoProps) -> Html {
    html! {
        <a href={ props.html_url.clone() }>
            <ul>
                <li>{ &props.name }</li>
                <li>{ &props.description.clone().unwrap() }</li>
            </ul>
        </a>
    }
}

#[function_component(Portfolio)]
pub fn portfolio() -> Html {
    let repos = use_state(|| vec![]);
    let error = use_state(|| None);
    
    let fetch_repos = {
        let repos = repos.clone();
        let error = error.clone();
        Callback::from(move |_| {
            let repos = repos.clone();
            let error = error.clone();
            spawn_local(async move {
                match Repositories::get().await {
                    Ok(result) => repos.set(result.repos),
                    Err(err) => error.set(Some(err.to_string())),
                }
            });
        })
    };


    html! {
        <>
            <button onclick={fetch_repos}>{ "Fetch Repos" }</button>
            { if !(*repos).is_empty() {
                html! {
                    <ul>
                        { for repos.iter().map(|repo| html! { <li>{ &repo.name }</li> }) }
                    </ul>
                }
            } else if let Some(error) = &*error {
                html! { <p>{ error }</p> }
            } else {
                html! { <p>{ "No data" }</p> }
            }}
        </>
    }
}
