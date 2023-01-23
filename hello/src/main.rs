/*Command-line interface for Marco Polo */
use clap::Parser;

#[derive(Parser)]
#[clap(
    version = "1.0",
    author = "Noah Gift",
    about = "Command-line interface for Marco Polo"
)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    #[clap(version = "1.0", author = "Noah Gift")]
    Marco {
        #[clap(short, long)]
        input: String,
    },
}

fn main() {
    let args = Cli::parse();
    if let Some(Commands::Marco { input }) = args.command {
        let result = hello::marco_polo(&input);
        println!("{}", result);
    } else {
        println!("No command given");
    }
}
