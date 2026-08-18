// ============================================================
// 练习 A05: 记账本 —— 答案版
// 目标: SQL 插件（建表迁移 + 前端增删改查 + SUM 统计）
// ============================================================

use tauri_plugin_sql::{Migration, MigrationKind};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // 迁移：前端 Database.load 连接数据库时自动执行（见 sql 插件的 load 命令）
    let migrations = vec![Migration {
        version: 1,
        description: "create_expenses",
        sql: "CREATE TABLE IF NOT EXISTS expenses (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            amount REAL NOT NULL,
            category TEXT NOT NULL,
            created_at TEXT NOT NULL
        );",
        kind: MigrationKind::Up,
    }];

    tauri::Builder::default()
        .plugin(
            tauri_plugin_sql::Builder::default()
                .add_migrations("sqlite:expenses.db", migrations)
                .build(),
        )
        .run(tauri::generate_context!())
        .expect("启动 Tauri 应用失败");
}
