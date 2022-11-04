// @generated automatically by Diesel CLI.

diesel::table! {
    section (id) {
        id -> Integer,
        sec -> Varchar,
        studentNo -> Varchar,
    }
}

diesel::table! {
    task (id) {
        id -> Integer,
        name -> Varchar,
        class -> VarChar,
    }
}

diesel::table! {
    users (id) {
        id -> Integer,
        name -> Nullable<Varchar>,
        rollno -> Integer,
        sec -> Varchar,
    }
}

diesel::allow_tables_to_appear_in_same_query!(section, task, users,);
