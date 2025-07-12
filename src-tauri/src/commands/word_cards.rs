use diesel::prelude::*;
use diesel::dsl::{exists, select};
use crate::db::establish_connection;
use crate::models::word_cards::NewWordCard;
use crate::schema::word_cards::dsl::*;

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
