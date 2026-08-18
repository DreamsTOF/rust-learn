# 练习 A05: 记账本（练习版）

**目标：** 做一个记账本——记每笔支出，按月份看合计。数据存进 **SQLite 数据库**（sql 插件）：Rust 端注册插件 + 迁移建表，前端用 `Database` API 做增删改查和 `SUM` 统计。

**新增知识：** sql 插件——迁移（`Migration`）、`Database.load` / `execute` / `select`、SQL 语句（`CREATE TABLE` / `INSERT` / `SELECT` / `DELETE` / `GROUP BY + SUM`）。

**TODO（共 7 处）：**

- `src-tauri/src/lib.rs`（2 处）
  - 步骤 1：写建表迁移语句
  - 步骤 2：注册 sql 插件并挂载迁移
- `src/App.tsx`（5 处）
  - 步骤 1：查询全部支出（`SELECT ... ORDER BY id DESC`）
  - 步骤 2：月度合计（`GROUP BY month` + `SUM(amount)`）
  - 步骤 3：插入（`INSERT ... datetime('now','localtime')`）
  - 步骤 4：删除（`DELETE ... WHERE id = $1`）

**配置文件（练习版已配好）：** capabilities 加 `sql:default`、`sql:allow-execute`；`package.json` 加 `@tauri-apps/plugin-sql`。

**卡住了？** 先看练习讲解，再对照答案讲解：`tauri-learn-book-v2/src/02_mini_apps/`。

## 运行

```bash
pnpm install
cargo tauri dev
```

## 信息

- devUrl: http://localhost:1430
- identifier: com.taurilearn.a05
