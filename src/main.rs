use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct CliArgs {
    #[arg(short, long)]
    name: String,

    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

fn hello(name: &String) -> String {
    format!("Hello {}!", name)
}

fn main() {
    let args = CliArgs::parse();

    for _ in 0..args.count {
        let msg = hello(&args.name);
        println!("{msg}");
    }
}

#[cfg(test)]
mod tests {
    use crate::hello;

    #[test]
    fn it_works() {
        let input = String::from("cli");
        let result = hello(&input);
        assert_eq!(result, "Hello cli!");
    }
}
