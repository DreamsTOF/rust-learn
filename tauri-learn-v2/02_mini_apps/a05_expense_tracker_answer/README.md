# 练习 A05: 记账本（答案版）

**目标：** 做一个记账本——记每笔支出，按月份看合计。数据存进 **SQLite 数据库**（sql 插件）：Rust 端注册插件 + 迁移建表，前端用 `Database` API 做增删改查和 `SUM` 统计。

**对照练习版补全的内容：**

- `src-tauri/src/lib.rs`
  - 建表迁移：`CREATE TABLE IF NOT EXISTS expenses (...)`
  - `tauri_plugin_sql::Builder::default().add_migrations("sqlite:expenses.db", migrations).build()`
- `src/App.tsx`
  - `SELECT id, title, amount, category, created_at AS createdAt FROM expenses ORDER BY id DESC`
  - `SELECT strftime('%Y-%m', created_at) AS month, SUM(amount) AS total FROM expenses GROUP BY month ORDER BY month DESC`
  - `INSERT INTO expenses (title, amount, category, created_at) VALUES ($1, $2, $3, datetime('now','localtime'))`
  - `DELETE FROM expenses WHERE id = $1`

**完整讲解见：** `tauri-learn-book-v2/src/02_mini_apps/a05_expense_tracker_answer.md`。

## 运行

```bash
pnpm install
cargo tauri dev
```

## 信息

- devUrl: http://localhost:1431
- identifier: com.taurilearn.a05a
