mod doctor;
mod util;

use doctor::doctor;

use clap::{CommandFactory, Parser, Subcommand};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
}

impl Args {
    fn no_subcommand(&self) -> bool {
        self.command.is_none()
    }

    fn banner(&self) {
        println!("Welcome to the Rust CLI!");
    }

    fn print_long_help(&self) {
        let mut cmd = Args::command();
        cmd.print_long_help().unwrap();
        println!();
    }
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Setup MpyC for your system
    Setup,

    /// List installed and available versions
    List,

    /// Select and if not installed, install a version to use
    Use,

    /// Run some checks to make sure everything is working
    Doctor,

    /// Build ("compile") a project,a folder or a single file
    Build {
        /// Input folder
        #[arg(short, long)]
        input: String,

        /// Output folder
        #[arg(short, long)]
        output: String,

        /// Optional file input
        #[arg(short, long)]
        file: Option<String>,

        /// Some flags
        #[arg(long, action = clap::ArgAction::SetTrue)]
        verbose: bool,
    },
}

fn main() {
    let args = Args::parse();

    if args.no_subcommand() {
        args.banner();
        args.print_long_help();
        return;
    }

    match args.command.unwrap() {
        Commands::Doctor => doctor(),
        Commands::Setup => println!("Setup command not implemented yet"),
        Commands::List => println!("List command not implemented yet"),
        Commands::Use => println!("Use command not implemented yet"),
        Commands::Build {
            input,
            output,
            file,
            verbose,
        } => {
            println!(
                "Build command: input={input}, output={output}, file={:?}, verbose={verbose}",
                file
            );
        }
    }
}
