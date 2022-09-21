// @generated automatically by Diesel CLI.

diesel::table! {
    threads (id) {
        id -> Varchar,
        board -> Varchar,
        user -> Varchar,
        title -> Varchar,
        text -> Nullable<Mediumtext>,
        media -> Nullable<Mediumtext>,
        score -> Integer,
    }
}
