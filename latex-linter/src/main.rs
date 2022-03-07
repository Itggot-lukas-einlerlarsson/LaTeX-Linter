mod cli;
use cli::CLI;

fn main() {
    let mut cli = CLI::new();
    CLI::parse_args(&mut cli);
    CLI::run(&mut cli)
}
