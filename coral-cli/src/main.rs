use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(author = "Chris Boyce, Cameron Fraser", version = "0.1", about, long_about = None)]
struct Args {
    #[clap(subcommand)]
    action: Action,
}

#[derive(Subcommand, Debug)]
enum Action {
    /// Initialize coral
    Init {
        #[clap(short, long, forbid_empty_values = true, required = false)]
        /// Location of directory to initialize in if different from cwd
        dir: Option<String>,
    },
}

fn main() {
    let args = Args::parse();

    match args.action {
        Action::Init { dir } => match dir {
            Some(d) => println!("dir provided at {}", d),
            None => println!("No dir provided"),
        },
    }
}
