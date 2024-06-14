diesel::table! {
    boards (id) {
        id -> Integer,
        name -> Text,
    }
}
diesel::table! {
    board_objects (board_id, object_type_number, object_id) {
            board_id -> Integer,
            left -> Text,
            top -> Text,
            width -> Text,
            height -> Text,
            object_type_number -> Integer,
            object_id -> Integer,
        }
}

diesel::table! {
    sticky_notes (id, board_id) {
        id -> Integer,
        board_id -> Integer,
        memo -> Nullable<Text>,
    }
}
diesel::table! {
    headlines (id, board_id) {
        id -> Integer,
        board_id -> Integer,
        headline -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    boards,
    board_objects,
    sticky_notes,
    headlines
);
