use clap::{CommandFactory, FromArgMatches, Parser};
use verbose::verbality;

#[derive(Parser, Debug)]
#[command(name = "verbose")]
#[command(about = "Number verbalization utility")]
struct Args {
    #[arg(help = "Number to verbalize")]
    number: u64,

    #[arg(short, long)]
    lang: String,
}

fn main() {
    let registry = verbality::registry();
    let langs = registry.codes_string();
    let mut cmd = Args::command();
    cmd = cmd.mut_arg("lang", |arg| arg.help(format!("Language ({})", langs)));

    let matches = cmd.get_matches();
    let args = Args::from_arg_matches(&matches).unwrap();

    let verbalizer = match registry.get(&args.lang) {
        Some(v) => v,
        None => {
            eprintln!(
                "Error: unknown language '{}'. Available: {}",
                args.lang,
                registry.codes_string()
            );
            std::process::exit(1);
        }
    };

    match verbalizer.verbalize(args.number) {
        Ok(result) => println!("{}", result),
        Err(e) => {
            eprintln!("Error: {}.", e);
            std::process::exit(1);
        }
    }
}
