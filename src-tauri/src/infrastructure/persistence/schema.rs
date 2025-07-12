// @generated automatically by Diesel CLI.

diesel::table! {
    word_cards (id) {
        id -> Nullable<Integer>,
        word -> Text,
        pos -> Nullable<Text>,
        definition -> Nullable<Text>,
        pronunciation -> Nullable<Text>,
        verbs -> Nullable<Text>,
        familiarity -> Nullable<Integer>,
        seen_count -> Nullable<Integer>,
        created_at -> Nullable<Text>,
    }
}
