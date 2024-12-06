#[macro_use]
extern crate rocket;

//use rocket::http::Status;
use diesel::prelude::*;
use rocket_app_rotas::models::{User, NewUser, UpdateUser};
use rocket_app_rotas::schema::users::dsl::users;
use rocket_app_rotas::schema::musics::dsl::musics;
use rocket_app_rotas::models::{Music, NewMusic, UpdateMusic};
use rocket_app_rotas::establish_connection;
use rocket::serde::json::{Json, Value as JsonValue};
use serde_json::json; 

// >>>>>>>>>>>>>>> ROTAS GET <<<<<<<<<<<<<<<
#[get("/users")]
fn list_users() -> Json<JsonValue> {
    let connection = &mut establish_connection();

    let results = users
        .load::<User>(connection)
        .expect("Erro ao consultar users");

    Json(JsonValue::from(json!({
        "users": results,
    })))
}

#[get("/musics")]
fn list_musics() -> Json<JsonValue> {
    let connection = &mut establish_connection();

    let results = musics
        .load::<Music>(connection)
        .expect("Erro ao consultar a tabela de músicas.");

    Json(JsonValue::from(json!({
        "Músicas": results,
    })))
}

// >>>>>>>>>>>>>>> ROTAS POST <<<<<<<<<<<<<<<
#[post("/users", format = "json",data = "<new_user>")]
fn create_user(new_user: Json<NewUser>) -> Json<JsonValue> {

    let new_user = NewUser {
        username: new_user.username.clone(),
        email: new_user.email.clone(),
        senha: new_user.senha.clone(),
    };

    let connection = &mut establish_connection();

    diesel::insert_into(users)
        .values(&new_user)
        .execute(connection)
        .expect("erro ao criar user");

    Json(JsonValue::from(json!({
        "status": "Sucesso!",
        "message": "O usuário foi adicionado.",    
    })))
}

#[post("/musics", format = "json", data = "<new_music>")]
fn create_music(new_music: Json<NewMusic>) -> Json<JsonValue> {
    let new_music = NewMusic {
        faixa: new_music.faixa.clone(),
        artista: new_music.artista.clone(),
        partitura: new_music.partitura.clone(),
        genero: new_music.genero.clone(),
    };

    let connection = &mut establish_connection();

    diesel::insert_into(musics)
        .values(&new_music)
        .execute(connection)
        .expect("Erro ao adicionar nova música");

    Json(JsonValue::from(json!({
        "status": "Sucesso!!",
        "message": "A música foi adicionada.",    
    })))
}


// >>>>>>>>>>>>>>> ROTAS PUT <<<<<<<<<<<<<<<
#[put("/users/<id>", data = "<updated_user>")]
fn edit_user(id: i32, updated_user: Json<UpdateUser>) -> Json<JsonValue> {

    let updated_user = UpdateUser {
        username: updated_user.username.clone(),
        email: updated_user.email.clone(),
        senha: updated_user.senha.clone(),
    };

    let connection = &mut establish_connection();

    diesel::update(users.find(id))
        .set(&updated_user)
        .execute(connection)
        .expect("erro ao editar user");

    Json(JsonValue::from(json!({
        "status": "success",
        "message": "Student has been edited",    
    })))
}

#[put("/music/<faixa>", data = "<updated_music>")]
fn update_music(faixa: &str, updated_music: Json<UpdateMusic>) -> Json<JsonValue> {

    let updated_music_persistance = UpdateMusic {
        faixa: updated_music.faixa.clone(),
        artista: updated_music.artista.clone(),
        partitura: updated_music.partitura.clone(),
        genero: updated_music.genero.clone(),
    };

    let connection = &mut establish_connection();

    diesel::update(musics.find(faixa))
        .set(&updated_music_persistance)
        .execute(connection)
        .expect("Erro ao alterar música.");

    Json(JsonValue::from(json!({
        "status": "Sucesso!",
        "message": "A música foi alterada.",    
    })))
}

// >>>>>>>>>>>>>>> ROTAS DELETE <<<<<<<<<<<<<<<
#[delete("/users/<id>")]
fn delete_user(id: i32) -> Json<JsonValue> {
    let connection = &mut establish_connection();

    diesel::delete(users.find(id))
        .execute(connection)
        .expect("erro ao deletar user");

    Json(JsonValue::from(json!({
        "status": "success",
        "message": "Student has been deleted",    
    })))
}

#[delete("/musics/<faixa>")]
fn delete_music(faixa: String) -> Json<JsonValue> {
    let connection = &mut establish_connection();

    diesel::delete(musics.find(faixa))
        .execute(connection)
        .expect("Não foi possível deletar a música.");

    Json(JsonValue::from(json!({
        "status": "Sucesso!",
        "message": "A música foi deletada!",    
    })))
}


#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![ 
            list_users,  
            create_user,  
            edit_user, 
            delete_user, 
            list_musics,
            create_music,
            update_music,
            delete_music,
        ])
}
