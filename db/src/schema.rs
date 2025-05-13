// @generated automatically by Diesel CLI.

diesel::table! {
    entries (id) {
        id -> Text,
        person_id -> Text,
        instant -> Timestamp,
        action -> Text,
    }
}

diesel::table! {
    permissions (id) {
        id -> Text,
        person_id -> Text,
        dashboard -> Bool,
        see_self_history -> Bool,
        see_others_history -> Bool,
        admin_panel -> Bool,
        edit_permissions -> Bool,
    }
}

diesel::table! {
    person (id) {
        id -> Text,
        name -> Text,
        surname -> Text,
        email -> Text,
        role -> Text,
        password_hash -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(entries, permissions, person,);
