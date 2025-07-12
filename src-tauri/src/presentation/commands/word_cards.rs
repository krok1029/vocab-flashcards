use diesel::prelude::*;
use diesel::dsl::{exists, select};
use crate::infrastructure::persistence::establish_connection;
use crate::domain::entities::word_cards::{NewWordCard, WordCard};
use crate::infrastructure::persistence::schema::word_cards::dsl::*;

#[tauri::command]
pub fn save_word_card(card: NewWordCard) -> Result<(), String> {
    let mut conn = establish_connection();

    println!("🔍 嘗試儲存單字卡：{}", card.word);

    let exists = select(exists(word_cards.filter(word.eq(&card.word))))
        .get_result::<bool>(&mut conn)
        .map_err(|e| {
            println!("❌ 查詢是否存在時失敗：{}", e);
            e.to_string()
        })?;

    if exists {
        println!("📝 該單字已存在，更新 seen_count...");
        diesel::update(word_cards.filter(word.eq(&card.word)))
            .set(seen_count.eq(seen_count + 1))
            .execute(&mut conn)
            .map_err(|e| {
                println!("❌ 更新失敗：{}，錯誤：{}", card.word, e);
                e.to_string()
            })?;
        println!("✅ 更新成功：{} 的 seen_count +1", card.word);
    } else {
        diesel::insert_into(word_cards)
            .values(&card)
            .execute(&mut conn)
            .map_err(|e| {
                println!("❌ 插入失敗：{}，錯誤：{}", card.word, e);
                e.to_string()
            })?;
        println!("✅ 插入成功：{}", card.word);
    }

    Ok(())
}

#[tauri::command]
pub fn get_word_card_by_word(word_query: String) -> Result<Option<WordCard>, String> {
    let mut conn = establish_connection();

    match word_cards
        .filter(word.eq(&word_query))
        .select(WordCard::as_select()) // 明確選取 struct 對應欄位
        .first::<WordCard>(&mut conn)
    {
        Ok(card) => Ok(Some(card)),
        Err(diesel::result::Error::NotFound) => Ok(None),
        Err(e) => {
            println!("❌ 查詢單字卡失敗：{}", e);
            Err(e.to_string())
        }
    }
}
