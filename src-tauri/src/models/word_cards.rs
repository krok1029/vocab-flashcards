use serde::{Deserialize, Serialize};
use crate::schema::word_cards;
use diesel::insertable::Insertable;


#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = word_cards)]
pub struct NewWordCard {
    pub word: String,
    pub pos: Option<String>,
    pub definition: Option<String>,
    pub pronunciation: Option<String>,
    pub verbs: Option<String>,
    pub familiarity: Option<i32>,
    pub seen_count: Option<i32>,
}
