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
                .arg(arg!(<PROJECT_NAME> "Name of the project"))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("run")
                .about("Run the current project")
                .args_conflicts_with_subcommands(true),
        )
        .subcommand(
            Command::new("add")
                .about("Install modules to current project")
                .arg(arg!(<PACKAGE_NAME> "Name of the module"))
                .arg_required_else_help(true),
        )
}
fn main() {
    let matches = cli().get_matches();
    match matches.subcommand() {
        Some(("new", sub_matches)) => {
            commands::new::exec(
                sub_matches
                    .get_one::<String>("PROJECT_NAME")
                    .expect("required"),
            )
            .unwrap();
        }
        Some(("run", _)) => {
            commands::run::exec().unwrap();
        }
        Some(("add", sub_matches)) => commands::add::exec(
            sub_matches
                .get_one::<String>("PACKAGE_NAME")
                .expect("required"),
            false,
        )
        .unwrap(),
        _ => {}
    }
}
