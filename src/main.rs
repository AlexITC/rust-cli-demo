use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct CliArgs {
    #[arg(short, long)]
    name: String,

    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

fn main() {
    let args = CliArgs::parse();

    for _ in 0..args.count {
        println!("Hello {}!", args.name)
    }
}
