use diesel::prelude::*;
use crate::db::establish_connection;
use crate::models::word_cards::NewWordCard;
use crate::schema::word_cards;

#[tauri::command]
pub fn save_word_card(card: NewWordCard) -> Result<(), String> {
    let mut conn = establish_connection();

    diesel::insert_into(word_cards::table)
        .values(&card)
        .execute(&mut conn)
        .map_err(|e| e.to_string())?;

    Ok(())
}
