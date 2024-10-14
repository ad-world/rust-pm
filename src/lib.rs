pub mod models;
pub mod schema;
use diesel::prelude::*;
use dotenvy::dotenv;
use models::{NewVault, Vault};
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_vault(conn: &mut PgConnection, name: &str, password: &str) -> Vault {
    use crate::schema::vault;

    let new_vault = NewVault { name: name.to_string(), password: password.to_string() };

    diesel::insert_into(vault::table)
        .values(&new_vault)
        .returning(Vault::as_returning())
        .get_result(conn)
        .expect("Error saving new vault")
}

pub fn delete_vault(conn: &mut PgConnection, id: i32) -> Result<(), String> {
    use crate::schema::vault;

    match diesel::delete(vault::table.find(id)).execute(conn) {
        Ok(num_deleted) => {
            if num_deleted == 0 {
                Err("No vault found with the given ID".to_string())
            } else {
                Ok(())
            }
        },
        Err(e) => Err(format!("Error deleting vault: {}", e)),
    }
}

pub fn get_vaults(conn: &mut PgConnection) -> Vec<Vault> {
    use crate::schema::vault;

    vault::table.load::<Vault>(conn).expect("Error loading vaults")
}

