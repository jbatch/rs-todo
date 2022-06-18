use clap::Parser;
use todo::{handle_to_do_command, Args};

fn main() {
    let args = Args::parse();
    handle_to_do_command(args.command);
}
