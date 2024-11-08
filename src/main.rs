#[macro_use] extern crate rocket;

//use rocket::response::content;
use rocket::form::Form;
use rocket_dyn_templates::{Template, context};

#[get("/")]
fn index() -> Template {
    // Aqui, você pode passar dados opcionais para o template usando o contexto
    Template::render("index", context! {
        title: "Página Inicial",
        message: "Bem-vindo à página inicial!"
    })
}

// Estrutura para capturar os dados do formulário
#[derive(FromForm)]
struct UserInput {
    first_name: String,
    last_name: String,
}

// Rota POST para processar o formulário e renderizar a página de saudação
#[post("/submit", data = "<user_input>")]
fn submit(user_input: Form<UserInput>) -> Template {
    let user = user_input.into_inner();
    Template::render("greeting", context! {
        title: "Saudação",
        greeting_message: format!("Olá, {} {}!", user.first_name, user.last_name)
    })
}

// Nova rota para a lista de músicas favoritas
#[get("/favorite-songs")]
fn favorite_songs() -> Template {
    let songs = vec![
        "Imagine - John Lennon",
        "Bohemian Rhapsody - Queen",
        "Stairway to Heaven - Led Zeppelin",
        "Hotel California - Eagles",
        "Hey Jude - The Beatles",
    ];

    Template::render("favorite_songs", context! {
        title: "Minhas Músicas Favoritas",
        songs
    })
}




#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, submit, favorite_songs])
        .attach(Template::fairing()) // Anexa o fairing do Handlebars para processar templates
}
