use clap::Parser;

#[derive(Parser, Debug)]
#[command(version = "0.1", about = "CLI to manage Tasks in TickTick", long_about = None)]
struct Cli {
    #[arg(short, long, value_enum, default_value_t = Action::Create)]
    action: Action,
}

#[derive(clap::ValueEnum, Clone, Debug)]
enum Action {
    Create,
    Delete,
}

fn main() {
    let cli = Cli::parse();

    let action = match cli.action {
        Action::Create => println!("Create Task"),
        Action::Delete => println!("Delete Task"),
    };
}
