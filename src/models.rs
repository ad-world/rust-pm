use diesel::prelude::*;
use crate::schema::vault;

#[derive(Queryable, Selectable)]
#[diesel(table_name = vault)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Vault {
    pub id: i32,
    pub name: String,
    pub password: String,
    pub created_at: std::time::SystemTime,
    pub updated_at: std::time::SystemTime,
}



#[derive(Insertable)]
#[diesel(table_name = vault)]
pub struct NewVault {
    pub name: String,
    pub password: String,
}