fn main() {
    // 告訴 Cargo 重新運行 build script 如果環境變量改變
    println!("cargo:rerun-if-env-changed=SQLITE3_LIB_DIR");
    println!("cargo:rerun-if-env-changed=SQLITE3_INCLUDE_DIR");
    println!("cargo:rerun-if-env-changed=TARGET");
    
    // 獲取目標平台
    let target = std::env::var("TARGET").unwrap_or_default();
    
    // 對於 Windows 目標，強制使用 bundled SQLite
    if target.contains("windows") {
        println!("cargo:rustc-cfg=feature=\"bundled\"");
        // 設置環境變量強制使用 bundled SQLite
        std::env::set_var("SQLITE3_STATIC", "1");
        std::env::set_var("LIBSQLITE3_SYS_USE_BUNDLED", "1");
    }
    
    tauri_build::build()
}
