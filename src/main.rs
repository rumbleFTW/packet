use clap::{arg, Command};
use packet::commands;

fn cli() -> Command {
    Command::new("packet")
        .bin_name("packet")
        .about("A package manager for python")
        .version("0.1.0")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .author("Packet team")
        .subcommand(
            Command::new("new")
                .about("Create a new project")
                .arg(arg!(<REMOTE> "Name of the project"))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("run")
                .about("Run the current project")
                .args_conflicts_with_subcommands(true),
        )
}
fn main() {
    let matches = cli().get_matches();
    match matches.subcommand() {
        Some(("new", sub_matches)) => {
            commands::new::exec(sub_matches.get_one::<String>("REMOTE").expect("required"))
                .unwrap();
        }
        Some(("run", _)) => {
            commands::run::exec().unwrap();
        }
        _ => {}
    }
}
