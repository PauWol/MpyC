use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    install: Option<String>,

    build: Option<String>,

    doctor: Option<String>,

    setup: Option<String>,
    
}

impl Args {
    
    fn no_subcommand(&self) -> bool {
        self.install.is_none() && self.build.is_none() && self.doctor.is_none() && self.setup.is_none()
    }
}

fn main() {
    let args = Args::parse();

    if args.no_subcommand() {
        println!("No subcommand was used");
        return;
    }

    println!("{:?}", args);

}