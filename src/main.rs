use commands::handle_vault_command;
mod commands;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let command: String = args[1].clone();

    match command.as_str() {
        "vault" => handle_vault_command(args),
        _ => println!("Unknown command"),
    }
}