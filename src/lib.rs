use std::{collections::HashMap, path::PathBuf};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub active_provider: Option<String>,
    pub providers: HashMap<String, ProviderConfig>,
}

#[derive(Serialize, Deserialize)]
pub struct ProviderConfig {
    pub api_key: String,
    pub active_model: Option<String>,
}

pub fn get_config_path() -> PathBuf {
    let mut path = dirs::config_dir().expect("Could not find config directory");
    path.push("open-code");
    std::fs::create_dir_all(&path).unwrap();
    path.push("config.json");
    path
}

pub fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    let path = get_config_path();

    if !path.exists() {
        return Ok(Config {
            active_provider: None,
            providers: HashMap::new(),
        });
    }

    let content = std::fs::read_to_string(path)?;
    let config: Config = serde_json::from_str(&content)?;
    Ok(config)
}

pub async fn fetch_providers_json() -> Result<Value, Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://models.dev/api.json")
        .await?
        .json::<Value>()
        .await?;
    Ok(resp)
}

pub async fn list_providers() -> Result<(), Box<dyn std::error::Error>> {
    let resp = fetch_providers_json().await?;

    if let Some(providers) = resp.as_object() {
        for (provider_name, _) in providers {
            println!("Provider: {}", provider_name);
        }
    }

    Ok(())
}

pub async fn list_models() -> Result<(), Box<dyn std::error::Error>> {
    let config = load_config().unwrap_or(Config {
        active_provider: None,
        providers: HashMap::new(),
    });

    match config.active_provider {
        Some(provider) => {
            let resp = fetch_providers_json().await?;

            match resp[&provider]["models"].as_object() {
                Some(models) => {
                    println!("Models for {}:", provider);
                    for (model_name, _) in models {
                        println!("  - {}", model_name);
                    }
                }
                None => {
                    println!("No models found for provider '{}'", provider);
                }
            }
        }
        None => {
            return Err("No active provider set. Run 'providers set --provider <name>' first".into());
        }
    }

    Ok(())
}

pub async fn save_provider(provider: &str, api_key: &str) -> Result<(), Box<dyn std::error::Error>> {
    let path = get_config_path();

    let resp = fetch_providers_json().await?;
    if resp.get(provider).is_none() {
        return Err(format!("Provider '{}' does not exist. Run 'providers list' to see available providers.", provider).into());
    }

    let mut config = load_config().unwrap_or(Config {
        active_provider: None,
        providers: HashMap::new(),
    });

    config.providers.insert(
        provider.to_string(),
        ProviderConfig {
            api_key: api_key.to_string(),
            active_model: None,
        },
    );

    let json = serde_json::to_string_pretty(&config)?;
    std::fs::write(path, json)?;

    Ok(())
}

pub fn remove_provider(provider: &str) -> Result<(), Box<dyn std::error::Error>> {
    let path = get_config_path();

    let mut config = load_config().unwrap_or(Config {
        active_provider: None,
        providers: HashMap::new(),
    });

    match config.providers.remove(provider) {
        Some(_) => println!("Removed {} successfully!", provider),
        None => println!("Provider '{}' not found", provider),
    }

    let json = serde_json::to_string_pretty(&config)?;
    std::fs::write(path, json)?;

    Ok(())
}

pub fn set_provider(provider: &str) -> Result<(), Box<dyn std::error::Error>> {
    let path = get_config_path();

    let mut config = load_config().unwrap_or(Config {
        active_provider: None,
        providers: HashMap::new(),
    });

    if config.providers.contains_key(provider) {
        config.active_provider = Some(provider.to_string());
    } else {
        return Err(format!("Provider '{}' not found, login first", provider).into());
    }

    let json = serde_json::to_string_pretty(&config)?;
    std::fs::write(path, json)?;

    println!("Active provider set to {}", provider);
    Ok(())
}

pub async fn set_model(model: &str) -> Result<(), Box<dyn std::error::Error>> {
    let path = get_config_path();

    let mut config = load_config().unwrap_or(Config {
        active_provider: None,
        providers: HashMap::new(),
    });

    match config.active_provider.clone() {
        None => {
            return Err("No active provider set. Run 'providers set --provider <name>' first".into());
        }
        Some(active_provider) => {
            let resp = fetch_providers_json().await?;

            let models = resp[&active_provider]["models"].as_object()
                .ok_or(format!("No models found for provider '{}'", active_provider))?;

            if !models.contains_key(model) {
                return Err(format!("Model '{}' not found under provider '{}'. Run 'models' to see available models.", model, active_provider).into());
            }

            config.providers
                .get_mut(&active_provider)
                .ok_or("Provider not found in config")?
                .active_model = Some(model.to_string());
        }
    }

    let json = serde_json::to_string_pretty(&config)?;
    std::fs::write(path, json)?;

    println!("Active model set to {}", model);
    Ok(())
}

pub async fn run_agent(prompt: &str) -> Result<(), Box<dyn std::error::Error>> {
    let config = load_config()?;

    let active_provider = config.active_provider
        .ok_or("No active provider set. Run 'providers set --provider <name>' first")?;

    let provider_config = config.providers.get(&active_provider)
        .ok_or("Provider not found in config, run 'providers login' first")?;

    let api_key = &provider_config.api_key;

    let model = provider_config.active_model.as_deref()
        .ok_or("No active model set. Run 'models set --model <name>' first")?;

    let mut all_code = String::new();

    for entry in walkdir::WalkDir::new(".") {

        let entry = entry?;

        if entry.path().extension().and_then(|e| e.to_str()) == Some("rs") {

            let content = std::fs::read_to_string(entry.path())?;

            all_code.push_str(&format!("\n\n// File: {}\n{}", entry.path().display(), content));

        }
    }

    let full_prompt = if all_code.is_empty() {
        
        prompt.to_string()

    } else {
        format!("Here is the codebase:\n{}\n\nTask: {}", all_code, prompt)
    };

    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}",
        model, api_key
    );

    let body = serde_json::json!({
        "contents": [{
            "parts": [{ "text": full_prompt }]
        }]
    });

    let resp = reqwest::Client::new()
        .post(&url)
        .json(&body)
        .send()
        .await?
        .json::<Value>()
        .await?;

    let text = resp["candidates"][0]["content"]["parts"][0]["text"]
        .as_str()
        .ok_or("No response from Gemini")?;

    println!("{}", text);

    Ok(())
}