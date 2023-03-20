// @generated automatically by Diesel CLI.

diesel::table! {
    games (id) {
        id -> Text,
        name -> Text,
        user_a_id -> Text,
        user_b_id -> Text,
        active_user_id -> Text,
        goal_a -> Float,
        goal_b -> Float,
        initial_value -> Float,
        current_value -> Float,
        status -> Text,
        created_at -> Text,
    }
}

diesel::table! {
    moves (id) {
        id -> Text,
        game_id -> Text,
        user_id -> Text,
        operator -> Text,
        term -> Text,
        created_at -> Text,
    }
}

diesel::table! {
    users (id) {
        id -> Text,
        username -> Text,
        created_at -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    games,
    moves,
    users,
);
