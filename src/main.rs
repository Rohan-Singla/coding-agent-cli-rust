
use clap::{Arg, Command};
use reqwest;
use serde_json::Value;
use serde::{Deserialize, Serialize};
use tui_assignment::save_provider;
use std::collections::HashMap;

#[tokio::main]
async fn main() {
    let cli = Command::new("open-code")
    .about("This is a CLI based Coding Agent !")
    .subcommand(
        Command::new("providers")
            .about("Provider-related commands")
            .subcommand(
                Command::new("list")
                    .about("List all available providers")
            )
            .subcommand(
                Command::new("login")
                    .about("Login to a specific provider")
                    .arg(
                        Arg::new("provider")
                            .long("provider")
                            .required(true)
                            .help("Name of the provider to log in to")
                    )
                    .arg(
                        Arg::new("api_key")
                            .long("api_key")
                            .required(true)
                            .help("API key for the specified provider")
                    )
            )
    )
    .get_matches();

    if let Some(("providers", providers_m)) = cli.subcommand() {

        match providers_m.subcommand() {
    
            Some(("list", _)) => {
                let result = fetch_and_list_providers().await;
                match result {
                    Ok(_) => {}
                    Err(e) => {
                        eprint!("{e}");
                    }
                }
            }
    
            Some(("login", login_m)) => {
                let provider = login_m.get_one::<String>("provider").unwrap();
                let api_key = login_m.get_one::<String>("api_key").unwrap();
                match save_provider(provider, api_key) {
                    Ok(_) => {
                        println!("Successfully saved credentials for {}", provider);
                    }
                    Err(e) => {
                        eprintln!("Failed to save credentials: {}", e);
                    }
                }
            }
    
            _ => {
                println!("Please use 'providers list' or 'providers login --provider <name> --api_key <key>'");
            }
    
        }
    }


    // if let Some(("login",_login_m)) = cli.subcommand() {
    //     let provider = _login_m.get_one::<String>("provider").unwrap();
    //     let api_key = _login_m.get_one::<String>("api_key").unwrap();

    //     match save_provider(provider, api_key) {
    //         Ok(_) => {
    //             println!("Successfully saved credentials for {}", provider);
    //         }
    //         Err(e) => {
    //             eprintln!("Failed to save credentials: {}", e);
    //         }
    //     }

    // } 

}

async fn fetch_and_list_providers() -> Result<(), Box<dyn std::error::Error>> {

    // OpenCode models API url
    
    let url = "https://models.dev/api.json";

    let resp = reqwest::get(url)
        .await?
        .json::<Value>()
        .await?;


    if let Some(providers) = resp.as_object() {

        for (provider_name, provider_data) in providers {
            println!("Provider: {}", provider_name);

            // print AI Models by providers

            // if let Some(models) = provider_data["models"].as_object() {
            //     for (model_name, _) in models {
            //         println!("  {}", model_name);
            //     }
            // }
        }

    }

        // println!("{:#?}", resp);

    Ok(())
}

