use diesel::prelude::*;
use diesel::dsl::{exists, select};
use log::{info, error, warn};
use crate::infrastructure::persistence::establish_connection;
use crate::domain::entities::word_cards::{NewWordCard, WordCard};
use crate::infrastructure::persistence::schema::word_cards::dsl::*;

#[tauri::command]
pub fn save_word_card(card: NewWordCard) -> Result<(), String> {
    info!("保存單字卡: '{}'", card.word);
    
    let mut conn = establish_connection();

    // 檢查單字是否已存在
    let exists_result = select(exists(word_cards.filter(word.eq(&card.word))))
        .get_result::<bool>(&mut conn)
        .map_err(|e| {
            let error_msg = format!("查詢單字存在性失敗: {}", e);
            error!("❌ {}", error_msg);
            e.to_string()
        })?;

    if exists_result {
        // 單字已存在，更新 seen_count
        info!("單字 '{}' 已存在，更新查看次數", card.word);
        
        let updated_rows = diesel::update(word_cards.filter(word.eq(&card.word)))
            .set(seen_count.eq(seen_count + 1))
            .execute(&mut conn)
            .map_err(|e| {
                let error_msg = format!("更新查看次數失敗 - 單字: '{}', 錯誤: {}", card.word, e);
                error!("❌ {}", error_msg);
                e.to_string()
            })?;
        
        if updated_rows > 0 {
            info!("✅ 成功更新單字 '{}' 的查看次數", card.word);
        } else {
            warn!("⚠️ 更新操作未影響任何行數，單字: '{}'", card.word);
        }
    } else {
        // 單字不存在，插入新記錄
        info!("插入新單字卡: '{}'", card.word);
        
        let inserted_rows = diesel::insert_into(word_cards)
            .values(&card)
            .execute(&mut conn)
            .map_err(|e| {
                let error_msg = format!("插入新單字失敗 - 單字: '{}', 錯誤: {}", card.word, e);
                error!("❌ {}", error_msg);
                e.to_string()
            })?;
        
        if inserted_rows > 0 {
            info!("✅ 成功插入新單字卡: '{}'", card.word);
        } else {
            warn!("⚠️ 插入操作未影響任何行數，單字: '{}'", card.word);
        }
    }

    Ok(())
}

#[tauri::command]
pub fn get_word_card_by_word(word_query: String) -> Result<Option<WordCard>, String> {
    if word_query.trim().is_empty() {
        warn!("查詢參數為空字串");
        return Err("查詢單字不能為空".to_string());
    }
    
    info!("查詢單字卡: '{}'", word_query);
    let mut conn = establish_connection();
    
    let result = word_cards
        .filter(word.eq(&word_query))
        .select(WordCard::as_select())
        .first::<WordCard>(&mut conn);

    match result {
        Ok(card) => {
            info!("✅ 找到單字卡: '{}'", word_query);
            Ok(Some(card))
        }
        Err(diesel::result::Error::NotFound) => {
            info!("未找到單字卡: '{}'", word_query);
            Ok(None)
        }
        Err(e) => {
            let error_msg = format!("查詢單字卡失敗 - 單字: '{}', 錯誤: {}", word_query, e);
            error!("❌ {}", error_msg);
            Err(e.to_string())
        }
    }
}

#[tauri::command]
pub fn get_all_word_cards() -> Result<Vec<WordCard>, String> {
    info!("查詢所有單字卡");
    
    let mut conn = establish_connection();
    
    let result = word_cards
        .select(WordCard::as_select())
        .load::<WordCard>(&mut conn);

    match result {
        Ok(cards) => {
            let count = cards.len();
            info!("✅ 成功查詢所有單字卡，共 {} 筆記錄", count);
            Ok(cards)
        }
        Err(e) => {
            let error_msg = format!("查詢所有單字卡失敗: {}", e);
            error!("❌ {}", error_msg);
            Err(e.to_string())
        }
    }
}

#[tauri::command]
pub fn update_word_card_familiarity(card_id: i32, familiarity_level: i32) -> Result<(), String> {
    // 驗證輸入參數
    if card_id <= 0 {
        warn!("無效的單字卡 ID: {}", card_id);
        return Err("單字卡 ID 必須大於 0".to_string());
    }
    
    if familiarity_level < 0 || familiarity_level > 3 {
        warn!("無效的熟悉度級別: {} (應該在 0-3 之間)", familiarity_level);
        return Err("熟悉度級別必須在 0-3 之間".to_string());
    }
    
    info!("更新單字卡熟悉度: ID {} -> 級別 {}", card_id, familiarity_level);
    let mut conn = establish_connection();
    
    let updated_rows = diesel::update(word_cards.filter(id.eq(Some(card_id))))
        .set(familiarity.eq(Some(familiarity_level)))
        .execute(&mut conn)
        .map_err(|e| {
            let error_msg = format!("更新熟悉度失敗 - ID: {}, 錯誤: {}", card_id, e);
            error!("❌ {}", error_msg);
            e.to_string()
        })?;

    if updated_rows == 0 {
        let error_msg = format!("找不到 ID 為 {} 的單字卡", card_id);
        warn!("⚠️ {}", error_msg);
        return Err(error_msg);
    }

    info!("✅ 熟悉度更新成功: ID {} -> 級別 {}", card_id, familiarity_level);
    Ok(())
}

#[tauri::command]
pub fn delete_word_card(card_id: i32) -> Result<(), String> {
    if card_id <= 0 {
        warn!("無效的單字卡 ID: {}", card_id);
        return Err("單字卡 ID 必須大於 0".to_string());
    }
    
    info!("刪除單字卡: ID {}", card_id);
    let mut conn = establish_connection();
    
    let deleted_rows = diesel::delete(word_cards.filter(id.eq(Some(card_id))))
        .execute(&mut conn)
        .map_err(|e| {
            let error_msg = format!("刪除單字卡失敗 - ID: {}, 錯誤: {}", card_id, e);
            error!("❌ {}", error_msg);
            e.to_string()
        })?;

    if deleted_rows == 0 {
        let error_msg = format!("找不到 ID 為 {} 的單字卡", card_id);
        warn!("⚠️ {}", error_msg);
        return Err(error_msg);
    }

    info!("✅ 單字卡刪除成功: ID {}", card_id);
    Ok(())
}

#[tauri::command]
pub fn increment_word_card_seen_count(card_id: i32) -> Result<(), String> {
    if card_id <= 0 {
        warn!("無效的單字卡 ID: {}", card_id);
        return Err("單字卡 ID 必須大於 0".to_string());
    }
    
    info!("增加單字卡查看次數: ID {}", card_id);
    let mut conn = establish_connection();
    
    let updated_rows = diesel::update(word_cards.filter(id.eq(Some(card_id))))
        .set(seen_count.eq(seen_count + 1))
        .execute(&mut conn)
        .map_err(|e| {
            let error_msg = format!("更新查看次數失敗 - ID: {}, 錯誤: {}", card_id, e);
            error!("❌ {}", error_msg);
            e.to_string()
        })?;

    if updated_rows == 0 {
        let error_msg = format!("找不到 ID 為 {} 的單字卡", card_id);
        warn!("⚠️ {}", error_msg);
        return Err(error_msg);
    }

    info!("✅ 查看次數更新成功: ID {}", card_id);
    Ok(())
}

/// 簡化版本的查詢所有單字卡函數，用於排查問題
#[tauri::command]
pub fn get_all_word_cards_simple() -> Result<Vec<WordCard>, String> {
    info!("執行簡化版查詢所有單字卡");
    
    let mut conn = establish_connection();
    
    match word_cards.select(WordCard::as_select()).load::<WordCard>(&mut conn) {
        Ok(cards) => {
            let count = cards.len();
            info!("✅ 查詢成功，找到 {} 筆記錄", count);
            Ok(cards)
        }
        Err(e) => {
            let error_msg = format!("查詢失敗: {}", e);
            error!("❌ {}", error_msg);
            Err(error_msg)
        }
    }
}

/// 測試資料庫連接的函數
#[tauri::command]
pub fn test_database_connection() -> Result<String, String> {
    info!("測試資料庫連接");
    
    let mut conn = establish_connection();
    
    // 測試簡單查詢
    let count_result: Result<i64, diesel::result::Error> = word_cards
        .count()
        .get_result(&mut conn);
    
    match count_result {
        Ok(count) => {
            let msg = format!("資料庫連接正常，共有 {} 筆單字卡記錄", count);
            info!("✅ {}", msg);
            Ok(msg)
        }
        Err(e) => {
            let error_msg = format!("查詢計數失敗: {}", e);
            error!("❌ {}", error_msg);
            Err(error_msg)
        }
    }
}
