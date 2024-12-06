// @generated automatically by Diesel CLI.

diesel::table! {
    musics (faixa) {
        // id -> Integer,
        faixa -> Text,
        artista -> Text,
        partitura -> Text,
        genero -> Text,
    }
}

diesel::table! {
    users (id) {
        id -> Integer,
        username -> Text,
        email -> Text,
        senha -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    musics,
    users,
);
