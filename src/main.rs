use packet::commands;
fn main() {
    commands::new::exec("packet").unwrap();
    commands::run::exec().unwrap();
}
