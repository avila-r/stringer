use ::clap::{Args, Parser, Subcommand};

mod stringer;

#[derive(Subcommand)]
enum Commands {
    Reverse(Reverse),
    Inspect(Inspect),
}

#[derive(Args)]
struct Reverse {
    #[arg(help = "The string you want to reverse")]
    string: Option<String>,
}

#[derive(Args)]
struct Inspect {
    #[arg(help = "The string you want to inspect")]
    string: Option<String>,

    #[arg(
        short = 'd',
        long = "digits",
        help = "Inspect only the digits in the string"
    )]
    only_digits: bool,
}

#[derive(Parser)]
#[command(author, version)]
#[command(about = "stringer can transform your strings!", long_about = None)]
struct Input {
    #[command(subcommand)]
    command: Option<Commands>,
}

fn main() {
    let input = Input::parse();

    match &input.command {
        Some(Commands::Reverse(n)) => match n.string {
            Some(ref _n) => println! {"{}", stringer::reverse(_n)},
            None => println! {"provide a string to reverse"},
        },

        Some(Commands::Inspect(n)) => match n.string {
            Some(ref _n) => {
                let (res, variant) = stringer::inspect(_n, n.only_digits);

                println! {"{:?} has {} {}{}.", _n, res, variant, if res == 1 { "s" } else { "" } }
            }

            None => println! {"provide a string to inspect"},
        },

        None => {}
    }
}
