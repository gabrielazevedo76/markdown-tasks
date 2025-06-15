use chrono::Local;
use clap::{Parser, Subcommand};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::path::PathBuf;

// --- New: Configuration Struct ---
// This struct holds our application's configuration.
// `serde` traits allow it to be easily converted to/from JSON.
#[derive(Serialize, Deserialize, Debug, Default)]
struct Config {
    global_file: Option<PathBuf>,
}

#[derive(Parser, Debug)]
#[command(version = "0.1", about = "CLI to manage Markdown Tasks", long_about = None)]
struct Cli {
    #[command(subcommand)]
    action: Action,
}

#[derive(Subcommand, Debug)]
enum Action {
    /// Create a new task.
    Create(CreateArgs),
    /// Delete a task (not yet implemented).
    Delete,
    /// Manage application configuration.
    Config(ConfigArgs),
}

#[derive(Parser, Debug)]
struct CreateArgs {
    /// The content of the task to create.
    #[arg(allow_hyphen_values = true)]
    content: String,

    /// Path of the file to use. Overrides the global config.
    #[arg(long)]
    file: Option<PathBuf>,
}

#[derive(Parser, Debug)]
struct ConfigArgs {
    /// Sets the global file path for all tasks.
    #[arg(long)]
    global_file: PathBuf,
}

#[derive(Serialize, Debug)]
struct ApiRequest {
    model: String,
    prompt: String,
    max_tokens: u32,
}

#[derive(Deserialize, Debug)]
struct ApiResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize, Debug)]
struct Choice {
    text: String,
}

async fn improve_task_with_llm(user_input: &str) -> Result<String, reqwest::Error> {
    println!("🤖 Calling LLM to improve the task... please wait.");

    let api_url = "https://openrouter.ai/api/v1/completions";

    let api_key = env::var("OPENROUTER_API_KEY")
        .expect("Error: OPENROUTER_API_KEY environment variable not set.");

    // Construct the prompt for the LLM.
    let prompt = format!(
        "Human: You are a helpful assistant. Take the following raw task and improve it for a markdown task list.
        Make it clearer and more actionable. Add a single '- [ ] 📋' prefix. Raw task: \"{}\"\n\nAssistant:",
        user_input
    );

    // Create the JSON payload for the request.
    let request_payload = ApiRequest {
        model: "google/gemini-2.0-flash-001".to_string(),
        prompt,
        max_tokens: 100,
    };

    let client = reqwest::Client::new();
    let response = client
        .post(api_url)
        .bearer_auth(api_key)
        .json(&request_payload)
        .send()
        .await?;

    if response.status().is_success() {
        let api_response = response.json::<ApiResponse>().await?;

        if let Some(choice) = api_response.choices.into_iter().next() {
            Ok(choice.text.trim().to_string())
        } else {
            Ok(user_input.to_string())
        }
    } else {
        // If the API returns an error, print it and fall back to the original input.
        eprintln!("Error from LLM API: {}", response.status());
        eprintln!("Response body: {}", response.text().await?);
        Ok(format!("- [ ] {}", user_input)) // Fallback to original
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let cli = Cli::parse();
    let mut config = load_config()?;

    match cli.action {
        Action::Create(args) => {
            let path = match args.file.or(config.global_file) {
                Some(path) => path,
                None => {
                    eprintln!("Error: No file path provided.");
                    eprintln!(
                        "Please specify a file with --file <PATH>, or set a global default with:"
                    );
                    eprintln!("tasks config --global-file <PATH>");
                    std::process::exit(1);
                }
            };

            let improved_content = improve_task_with_llm(&args.content)
                .await
                .unwrap_or_else(|e| {
                    eprintln!("Failed to call LLM API: {}. Using original task.", e);
                    // Fallback if the API call completely fails
                    format!("- [ ] 📋{}", args.content)
                });

            let now = Local::now();
            let now_formated = now.format("%d/%m/%Y %H:%M");

            let task_content = format!("{} - 🕓{}", improved_content, now_formated);

            // 2. Write the *improved* content to the file.
            let mut file = OpenOptions::new().append(true).create(true).open(&path)?;
            writeln!(file, "{}", task_content)?;

            println!("\n✅ Successfully added improved task to {:?}", path);
            println!("   > {}", improved_content);
        }
        Action::Delete => println!("Delete Task"),
        Action::Config(args) => {
            config.global_file = Some(args.global_file);
            save_config(&config)?;

            if let Some(path) = &config.global_file {
                println!("Global file path successfully set to: {:?}", path);
            }
        }
    };

    Ok(())
}

/// Gets the path to the configuration file.
/// Uses the `directories` create to find the appropriate system location.
fn get_config_path() -> Option<PathBuf> {
    if let Some(proj_dirs) = ProjectDirs::from("com", "org", "TasksCLI") {
        let config_dir = proj_dirs.config_dir();
        if !config_dir.exists() {
            fs::create_dir_all(config_dir).ok()?;
        }
        Some(config_dir.join("config.json"))
    } else {
        None
    }
}

/// Loads the configuration from the JSON file.
/// Returns a default config if the file doesn't exist.
fn load_config() -> std::io::Result<Config> {
    let config_path = match get_config_path() {
        Some(path) => path,
        None => {
            eprintln!("Error: Could not determine a valid configuration path for the application.");
            std::process::exit(1);
        }
    };

    if !config_path.exists() {
        return Ok(Config::default());
    }

    let mut file = File::open(config_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let config = serde_json::from_str(&contents).unwrap_or_else(|err| {
        eprintln!(
            "Warning: Could not parse config file, using default. Error: {}",
            err
        );
        Config::default()
    });
    Ok(config)
}

/// Saves the configuration to the JSON file.
fn save_config(config: &Config) -> std::io::Result<()> {
    let config_path = match get_config_path() {
        Some(path) => path,
        None => {
            eprintln!("Error: Could not determine a valid configuration path to save to.");
            std::process::exit(1);
        }
    };

    let contents = serde_json::to_string_pretty(config)?;
    fs::write(config_path, contents)?;
    Ok(())
}
