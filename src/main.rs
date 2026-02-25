mod doctor;
mod use_version;
mod util;
mod versions;

use doctor::doctor;
use use_version::use_version;

use clap::builder::Styles;
use clap::builder::styling::{AnsiColor, Effects};
use clap::{CommandFactory, Parser, Subcommand};

fn clap_styles() -> Styles {
    Styles::styled()
        // Headers like "USAGE", "COMMANDS"
        .header(AnsiColor::BrightYellow.on_default().effects(Effects::BOLD))
        // Literal flags like --help
        .literal(AnsiColor::BrightBlue.on_default().effects(Effects::BOLD))
        // Placeholders like <INPUT>
        .placeholder(
            AnsiColor::BrightMagenta
                .on_default()
                .effects(Effects::ITALIC),
        )
        // Usage line
        .usage(AnsiColor::BrightYellow.on_default().effects(Effects::BOLD))
}

#[derive(Parser, Debug)]
#[command(
    version,
    about = "MicroPython Cross Compiler · Manager",
    long_about = None,
    disable_help_subcommand = true,
    styles = clap_styles(),
)]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
}

impl Args {
    fn no_subcommand(&self) -> bool {
        self.command.is_none()
    }

    fn banner(&self) {
        println!(
            r#"
            ███╗   ███╗██████╗ ██╗   ██╗ ██████╗
            ████╗ ████║██╔══██╗╚██╗ ██╔╝██╔════╝
            ██╔████╔██║██████╔╝ ╚████╔╝ ██║     
            ██║╚██╔╝██║██╔═══╝   ╚██╔╝  ██║     
            ██║ ╚═╝ ██║██║        ██║   ╚██████╗
            ╚═╝     ╚═╝╚═╝        ╚═╝    ╚═════╝


            "#
        );
    }

    fn print_long_help(&self) {
        let mut cmd = Args::command();
        cmd.print_long_help().unwrap();
        println!();
    }
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Select and if not installed, install a version to use
    Use,

    /// Build ("compile") a project,a folder or a single file
    Build {
        /// Input folder
        #[arg(short, long)]
        input: String,

        /// Output folder
        #[arg(short, long)]
        output: String,

        /// Some flags
        #[arg(long, action = clap::ArgAction::SetTrue)]
        verbose: bool,
    },

    /// Setup MpyC for your system
    Setup,

    /// Run some checks to make sure everything is working
    Doctor,
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
        Commands::Use => use_version(),
        Commands::Build {
            input,
            output,
            verbose,
        } => {
            println!("Build command: input={input}, output={output}, verbose={verbose}",);
        }
    }
}
