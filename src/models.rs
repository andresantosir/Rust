use diesel::{Insertable, Queryable, Selectable};
use crate::schema::users;
use crate::schema::musics;
use diesel::AsChangeset;
use rocket::serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub senha: String,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
#[derive(Deserialize, Serialize)]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub senha: String,
}

#[derive(AsChangeset)]
#[diesel(table_name = users)]
#[derive(Deserialize, Serialize)]
pub struct UpdateUser {
    pub username: String,
    pub email: String,
    pub senha: String,
}

// STRUCT DA ENTIDADE MÚSICA
#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = musics)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Music {
    pub faixa: String,
    pub artista: String,
    pub partitura: String,
    pub genero: String,
}

// ADICIONAR MÚSICA NOVA
#[derive(Insertable)]
#[diesel(table_name = musics)]
#[derive(Deserialize, Serialize)]
pub struct NewMusic {
    pub faixa: String,
    pub artista: String,
    pub partitura: String,
    pub genero: String,
}


// ATUALIZAR DADOS DA MÚSICA
#[derive(AsChangeset)]
#[diesel(table_name = musics)]
#[derive(Deserialize, Serialize)]
pub struct UpdateMusic {
    pub faixa: String,
    pub artista: String,
    pub partitura: String,
    pub genero: String,
}