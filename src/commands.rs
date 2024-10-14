use rust_pm::*;

pub fn handle_vault_command(args: Vec<String>) {
    let connection = &mut establish_connection();
    if args.len() < 3 {
        println!("Usage: rust-pm vault <command> [args]");
        return;
    }

    let command: String = args[2].clone();

    match command.as_str() {
        "list" => {
            if args.len() != 3 {
                println!("Usage: rust-pm vault list");
                return;
            }

            let vaults = get_vaults(connection);

            for vault in vaults {
                println!("{} {} {}", vault.id, vault.name, vault.password);
            }
        },
        "create" => {
            if args.len() != 5 {
                println!("Usage: rust-pm vault create <name> <password>");
                return;
            }

            let name: String = args[3].clone();
            let password: String = args[4].clone();

            let vault = create_vault(connection, name.as_str(), password.as_str());

            println!("Vault created: {}", vault.name);
        },
        "delete" => {
            if args.len() != 4 {
                println!("Usage: rust-pm vault delete <id>");
                return;
            }

            let id: i32 = args[3].parse().unwrap();

            let result = delete_vault(connection, id);

            match result {
                Ok(_) => println!("Vault deleted: {}", id),
                Err(e) => println!("Error deleting vault: {}", e),
            }
        },
        _ => println!("Unknown command"),
    }
}

