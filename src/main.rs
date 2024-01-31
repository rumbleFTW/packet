use clap::{arg, Command};
use packet::commands;

/// Define the command-line interface for the "packet" tool.
fn cli() -> Command {
    Command::new("packet")
        .bin_name("packet")
        .about("A package manager for Python")
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
                .about("Install modules to the current project")
                .arg(arg!(<PACKAGE_NAME> "Name of the module"))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("init")
            .about("Initialize a new project in the current directory")
        )
        .subcommand(
            Command::new("check")
            .about("Check if there is any error with the project")
        )
}

/// Main function to parse command-line arguments and execute corresponding actions.
fn main() {
    // Get command-line matches based on the defined CLI structure
    let matches = cli().get_matches();

    // Match the subcommand and execute the corresponding action
    match matches.subcommand() {
        Some(("new", sub_matches)) => {
            // Execute the "new" command with the specified project name
            commands::new::exec(
                sub_matches
                    .get_one::<String>("PROJECT_NAME")
                    .expect("required"),
            )
            .unwrap();
        }
        Some(("run", _)) => {
            // Execute the "run" command
            commands::run::exec().unwrap();
        }
        Some(("add", sub_matches)) => {
            // Execute the "add" command with the specified package name
            commands::add::exec(
                sub_matches
                    .get_one::<String>("PACKAGE_NAME")
                    .expect("required"),
            )
            .unwrap();
        }
        Some(("init", _)) => {
            // Execute the "init" command
            commands::init::exec().unwrap();
        }
        Some(("check", _)) => {
            // Execute the "check" command
            commands::check::exec().unwrap();
        }
        _ => {}
    }
}
