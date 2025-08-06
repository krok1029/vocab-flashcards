use diesel::prelude::*;
use diesel::dsl::{exists, select};
use log::{info, error, debug, warn};
use crate::infrastructure::persistence::establish_connection;
use crate::domain::entities::word_cards::{NewWordCard, WordCard};
use crate::infrastructure::persistence::schema::word_cards::dsl::*;
use crate::infrastructure::logger::Logger;

#[tauri::command]
pub fn save_word_card(card: NewWordCard) -> Result<(), String> {
    let mut conn = establish_connection();

    Logger::log_command_execution("save_word_card", &format!("word: {}", card.word));
    info!("🔍 嘗試儲存單字卡：{}", card.word);

    let exists = select(exists(word_cards.filter(word.eq(&card.word))))
        .get_result::<bool>(&mut conn)
        .map_err(|e| {
            Logger::error_with_context("save_word_card", &format!("查詢是否存在時失敗：{}", e));
            error!("❌ 查詢是否存在時失敗：{}", e);
            e.to_string()
        })?;

    if exists {
        info!("📝 該單字已存在，更新 seen_count...");
        diesel::update(word_cards.filter(word.eq(&card.word)))
            .set(seen_count.eq(seen_count + 1))
            .execute(&mut conn)
            .map_err(|e| {
                Logger::error_with_context("save_word_card", &format!("更新失敗：{}，錯誤：{}", card.word, e));
                error!("❌ 更新失敗：{}，錯誤：{}", card.word, e);
                e.to_string()
            })?;
        info!("✅ 更新成功：{} 的 seen_count +1", card.word);
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

#[tauri::command]
pub fn get_all_word_cards() -> Result<Vec<WordCard>, String> {
    let mut conn = establish_connection();

    word_cards
        .select(WordCard::as_select())
        .load::<WordCard>(&mut conn)
        .map_err(|e| {
            println!("❌ 查詢所有單字卡失敗：{}", e);
            e.to_string()
        })
}

#[tauri::command]
pub fn update_word_card_familiarity(card_id: i32, familiarity_level: i32) -> Result<(), String> {
    let mut conn = establish_connection();

    println!("🔄 更新單字卡熟悉度：ID {} -> {}", card_id, familiarity_level);

    let updated_rows = diesel::update(word_cards.filter(id.eq(card_id)))
        .set(familiarity.eq(familiarity_level))
        .execute(&mut conn)
        .map_err(|e| {
            println!("❌ 更新熟悉度失敗：{}", e);
            e.to_string()
        })?;

    if updated_rows == 0 {
        return Err(format!("找不到 ID 為 {} 的單字卡", card_id));
    }

    println!("✅ 熟悉度更新成功：ID {} -> {}", card_id, familiarity_level);
    Ok(())
}

#[tauri::command]
pub fn delete_word_card(card_id: i32) -> Result<(), String> {
    let mut conn = establish_connection();

    println!("🗑️ 刪除單字卡：ID {}", card_id);

    let deleted_rows = diesel::delete(word_cards.filter(id.eq(card_id)))
        .execute(&mut conn)
        .map_err(|e| {
            println!("❌ 刪除單字卡失敗：{}", e);
            e.to_string()
        })?;

    if deleted_rows == 0 {
        return Err(format!("找不到 ID 為 {} 的單字卡", card_id));
    }

    println!("✅ 單字卡刪除成功：ID {}", card_id);
    Ok(())
}

#[tauri::command]
pub fn increment_word_card_seen_count(card_id: i32) -> Result<(), String> {
    let mut conn = establish_connection();

    println!("👀 增加單字卡查看次數：ID {}", card_id);

    let updated_rows = diesel::update(word_cards.filter(id.eq(card_id)))
        .set(seen_count.eq(seen_count + 1))
        .execute(&mut conn)
        .map_err(|e| {
            println!("❌ 更新查看次數失敗：{}", e);
            e.to_string()
        })?;

    if updated_rows == 0 {
        return Err(format!("找不到 ID 為 {} 的單字卡", card_id));
    }

    println!("✅ 查看次數更新成功：ID {}", card_id);
    Ok(())
}
