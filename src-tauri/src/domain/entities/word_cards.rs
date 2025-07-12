use serde::{Deserialize, Serialize};
use crate::infrastructure::persistence::schema::word_cards;
use diesel::prelude::*; // ✅ 匯入 Queryable 等 Diesel 的 derive macro
use diesel::sqlite::Sqlite;

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = word_cards)]
#[diesel(check_for_backend(Sqlite))]
pub struct NewWordCard {
    pub word: String,
    pub pos: Option<String>,
    pub definition: Option<String>,
    pub pronunciation: Option<String>,
    pub verbs: Option<String>,
    pub familiarity: Option<i32>,
    pub seen_count: Option<i32>,
}

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = word_cards)]
#[diesel(check_for_backend(Sqlite))]
pub struct WordCard {
    pub id: Option<i32>, // ✅ Nullable<Integer>
    pub word: String,
    pub pos: Option<String>,
    pub definition: Option<String>,
    pub pronunciation: Option<String>,
    pub verbs: Option<String>,
    pub familiarity: Option<i32>,
    pub seen_count: Option<i32>,
    pub created_at: Option<String>, // ✅ Nullable<Text>
}
