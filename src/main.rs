mod cli;
mod commands;

fn main() {
    let args = cli::parse();

    println!("{:?}", args);

    match args.command {
        cli::Commands::Status => commands::status::fn_status(),
    }
}
