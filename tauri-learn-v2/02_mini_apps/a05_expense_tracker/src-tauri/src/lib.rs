// ============================================================
// 练习 A05: 记账本 —— 练习版
// 目标: SQL 插件（建表迁移 + 前端增删改查 + SUM 统计）
// TODO: 按注释提示补全（共 2 处）
// ============================================================

use tauri_plugin_sql::{Migration, MigrationKind};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // 迁移：前端 Database.load 连接数据库时自动执行（建表）
    let migrations = vec![Migration {
        version: 1,
        description: "create_expenses",
        // === 步骤 1 ————————————————————————————————————————————
        // TODO: 写建表语句——expenses 表五个字段：
        //       id 自增主键 / title 文本 / amount 实数 / category 文本 / created_at 文本
        // 提示: CREATE TABLE IF NOT EXISTS expenses (
        //         id INTEGER PRIMARY KEY AUTOINCREMENT,
        //         title TEXT NOT NULL,
        //         amount REAL NOT NULL,
        //         category TEXT NOT NULL,
        //         created_at TEXT NOT NULL
        //       );
        sql: "", // ← 替换成你的代码
        kind: MigrationKind::Up,
    }];

    tauri::Builder::default()
        .plugin(
            // === 步骤 2 ————————————————————————————————————
            // TODO: 注册 sql 插件，并把迁移挂到 "sqlite:expenses.db"
            // 提示: tauri_plugin_sql::Builder::default()
            //           .add_migrations("sqlite:expenses.db", migrations)
            //           .build()
            tauri_plugin_sql::Builder::default()
                .add_migrations("sqlite:expenses.db", migrations)
                .build(),
        )
        .run(tauri::generate_context!())
        .expect("启动 Tauri 应用失败");
}
