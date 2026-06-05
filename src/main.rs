
use clap::{Command};
use reqwest;
use serde_json::Value;

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
        )
        .get_matches();

    if let Some(("providers", providers_m)) = cli.subcommand() {
        
        if let Some(("list", _)) = providers_m.subcommand() {

            match fetch_and_list_providers().await {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("Error fetching providers: {}", e);
                }
            }
            return;
        }
    }

    println!("No valid subcommand provided. Use --help for more information.");
}

async fn fetch_and_list_providers() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://models.dev/api.json";

    let resp = reqwest::get(url)
        .await?
        .json::<Value>()
        .await?;


        println!("{:#?}", resp);

    Ok(())
}

