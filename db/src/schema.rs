// @generated automatically by Diesel CLI.

diesel::table! {
    entries (id) {
        #[max_length = 36]
        id -> Bpchar,
        #[max_length = 36]
        person_id -> Bpchar,
        instant -> Timestamp,
        #[max_length = 100]
        action -> Varchar,
    }
}

diesel::table! {
    permissions (id) {
        #[max_length = 36]
        id -> Bpchar,
        #[max_length = 36]
        person_id -> Bpchar,
        dashboard -> Bool,
        see_self_history -> Bool,
        see_others_history -> Bool,
        admin_panel -> Bool,
        edit_permissions -> Bool,
    }
}

diesel::table! {
    person (id) {
        #[max_length = 36]
        id -> Bpchar,
        #[max_length = 100]
        name -> Varchar,
        #[max_length = 100]
        surname -> Varchar,
        #[max_length = 100]
        email -> Varchar,
        #[max_length = 20]
        role -> Varchar,
        #[max_length = 100]
        password_hash -> Varchar,
    }
}

diesel::joinable!(entries -> person (person_id));
diesel::joinable!(permissions -> person (person_id));

diesel::allow_tables_to_appear_in_same_query!(
    entries,
    permissions,
    person,
);
