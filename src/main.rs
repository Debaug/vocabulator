use clap::Parser;
use vocabulator::cli::Config;

fn main() {
    let config = Config::parse();
    vocabulator::run(config).expect("error");
}
