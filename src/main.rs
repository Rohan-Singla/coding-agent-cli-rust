use clap::{Arg, Command};
use tui_assignment::{list_models, list_providers, remove_provider, run_agent, save_provider, set_model, set_provider};

#[tokio::main]
async fn main() {
    let cli = Command::new("open-code")
        .about("This is a CLI based Coding Agent !")
        .subcommand(
            Command::new("providers")
                .about("Provider-related commands")
                .subcommand(
                    Command::new("list")
                        .about("List all available providers"),
                )
                .subcommand(
                    Command::new("login")
                        .about("Login to a specific provider")
                        .arg(
                            Arg::new("provider")
                                .long("provider")
                                .required(true)
                                .help("Name of the provider to log in to"),
                        )
                        .arg(
                            Arg::new("api_key")
                                .long("api_key")
                                .required(true)
                                .help("API key for the specified provider"),
                        ),
                )
                .subcommand(
                    Command::new("logout")
                        .about("Logout a provider")
                        .arg(
                            Arg::new("provider")
                                .long("provider")
                                .required(true)
                                .help("Name of the provider you want to logout"),
                        ),
                )
                .subcommand(
                    Command::new("set")
                        .about("Set the active provider")
                        .arg(
                            Arg::new("provider")
                                .long("provider")
                                .required(true)
                                .help("Name of the provider you want to set"),
                        ),
                ),
        )
        .subcommand(
            Command::new("models")
                .about("List or set models")
                .subcommand(
                    Command::new("set")
                        .about("Set the active model")
                        .arg(
                            Arg::new("model")
                                .long("model")
                                .required(true)
                                .help("Name of the model to set as active"),
                        ),
                ),
        )
        .subcommand(
            Command::new("agent")
                .about("Run the coding agent with a prompt")
                .arg(
                    Arg::new("prompt")
                        .short('p')
                        .long("prompt")
                        .required(true)
                        .help("The task you want the agent to perform"),
                ),
        )
        .get_matches();

    if let Some(("providers", providers_m)) = cli.subcommand() {
        match providers_m.subcommand() {
            Some(("list", _)) => {
                match list_providers().await {
                    Ok(_) => {}
                    Err(e) => eprintln!("{e}"),
                }
            }
            Some(("login", login_m)) => {
                let provider = login_m.get_one::<String>("provider").unwrap();
                let api_key = login_m.get_one::<String>("api_key").unwrap();
                match save_provider(provider, api_key).await {
                    Ok(_) => println!("Successfully saved credentials for {}", provider),
                    Err(e) => eprintln!("Failed to save credentials: {}", e),
                }
            }
            Some(("logout", logout_m)) => {
                let provider = logout_m.get_one::<String>("provider").unwrap();
                match remove_provider(provider) {
                    Ok(_) => println!("Successfully logged out provider {}", provider),
                    Err(e) => eprintln!("Error logging out provider {}", e),
                }
            }
            Some(("set", set_m)) => {
                let provider = set_m.get_one::<String>("provider").unwrap();
                match set_provider(provider) {
                    Ok(_) => println!("Successfully changed the current provider to {}", provider),
                    Err(e) => eprintln!("Error changing the current provider {}", e),
                }
            }
            _ => {
                println!("Please use 'providers list' or 'providers login --provider <name> --api_key <key>'");
            }
        }
    }

    if let Some(("models", models_m)) = cli.subcommand() {
        match models_m.subcommand() {
            Some(("set", set_m)) => {
                let model = set_m.get_one::<String>("model").unwrap();
                match set_model(model).await {
                    Ok(_) => println!("Successfully set model to {}", model),
                    Err(e) => eprintln!("{e}"),
                }
            }
            _ => {
                match list_models().await {
                    Ok(_) => {}
                    Err(e) => eprintln!("{e}"),
                }
            }
        }
    }

    if let Some(("agent", agents_m)) = cli.subcommand() {
        let prompt = agents_m.get_one::<String>("prompt").unwrap();
        match run_agent(prompt).await {
            Ok(_) => {}
            Err(e) => eprintln!("{e}"),
        }
    }
}