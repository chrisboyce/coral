use clap::{Parser, Subcommand};
use dialoguer::{theme::ColorfulTheme, Input};
use std::{fmt::Debug, str::FromStr};

#[derive(Parser, Debug)]
#[clap(author = "Chris Boyce, Cameron Fraser", version = "0.1", about, long_about = None)]
struct Args {
    #[clap(subcommand)]
    action: Action,
}

#[derive(Parser, Debug)]
struct InitInputArgs {
    #[clap(short, long)]
    dir: Option<String>,

    #[clap(short, long)]
    name: Option<String>,
}

#[derive(Debug)]
struct InitArgs {
    dir: String,
    name: String,
}

impl From<InitInputArgs> for InitArgs {
    fn from(input: InitInputArgs) -> Self {
        let name = match input.name {
            Some(name) => name,
            None => prompt("What is the name of your monorepo?", None),
        };
        let dir = match input.dir {
            Some(d) => d,
            None => ".".to_owned(),
        };
        Self { name, dir }
    }
}

#[derive(Subcommand, Debug)]
enum Action {
    Init(InitInputArgs),
}

fn main() {
    let args = Args::parse();

    match args.action {
        Action::Init(init_input_args) => {
            let init_args = InitArgs::from(init_input_args);
            println!("{:?}", init_args);
        }
    }
}

fn prompt<T>(prompt: &str, default: Option<T>) -> T
where
    T: Clone + ToString + FromStr,
    <T as FromStr>::Err: Debug + ToString,
{
    let theme = &ColorfulTheme::default();
    let mut builder = Input::<T>::with_theme(theme);
    builder.with_prompt(prompt);

    if let Some(default) = default {
        builder.default(default);
    }

    builder.interact_text().unwrap()
}
