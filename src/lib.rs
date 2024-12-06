pub mod models;
pub mod schema;

use diesel::sqlite::SqliteConnection;
use diesel::prelude::*;
use std::env;
use dotenvy::dotenv;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

use crate::models::{User, NewUser};

pub fn create_user(conn: &mut SqliteConnection, username: &str, email: &str, senha: &str) -> User {
    use crate::schema::users::dsl::users; // Certifique-se de importar corretamente

    let new_user = NewUser {
        username: username.to_string(),  
        email: email.to_string(),        
        senha: senha.to_string(),        
    };

    diesel::insert_into(users)
        .values(&new_user)
        .get_result(conn)
        .expect("Error saving new user")
}
