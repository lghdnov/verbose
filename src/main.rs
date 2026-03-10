use clap::Parser;
use verbose::VerbalizerRegistry;

#[derive(Parser, Debug)]
#[command(name = "verbose")]
#[command(about = "Number verbalization utility", long_about = None)]
struct Args {
    #[arg(help = "Number to verbalize")]
    number: u64,

    #[arg(short, long, help = "Language: ru, en")]
    lang: String,
}

fn main() {
    let args = Args::parse();

    let registry = VerbalizerRegistry::default();

    let verbalizer = match registry.get(&args.lang) {
        Some(v) => v,
        None => {
            eprintln!(
                "Error: unknown language '{}'. Available: {}",
                args.lang,
                registry.codes().join(", ")
            );
            std::process::exit(1);
        }
    };

    let result = verbalizer.verbalize(args.number);
    println!("{}", result);
}
