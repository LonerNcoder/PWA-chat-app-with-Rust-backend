// @generated automatically by Diesel CLI.

diesel::table! {
    conversations (id) {
        id -> Text,
        room_id -> Text,
        sender_id -> Text,
        receiver_id -> Text,
        content -> Text,
        seen -> Bool,
        created_at -> Text,
    }
}

diesel::table! {
    rooms (room_id) {
        room_id -> Text,
        last_message -> Text,
        created_at -> Text,
    }
}

diesel::table! {
    user_friends (id) {
        id -> Text,
        user_id -> Text,
        friend_id -> Text,
        friend_name -> Text,
        created_at -> Text,
    }
}

diesel::table! {
    users (id) {
        id -> Text,
        username -> Text,
        phone -> Text,
        password -> Text,
        created_at -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    conversations,
    rooms,
    user_friends,
    users,
);
