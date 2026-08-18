# 练习 A05 答案讲解：记账本

> **用法**：卡住时再看本页。每一处 TODO 按三步走——先看"练习版缺什么"，再看"答案版填了什么"，最后回查原理文档对应小节。
> **作业范围**：本练习只需动 2 个文件：`src-tauri/src/lib.rs`（后端）与 `src/App.tsx`（前端 React），共 7 处 TODO。capabilities 和 `package.json`（`@tauri-apps/plugin-sql`）已配好。

## 对照总览

| 文件 | 练习版状态 | 你缺的 |
|---|---|---|
| `src-tauri/src/lib.rs` | 迁移框架已给，`sql` 为空串 | 建表语句 + 已给的 `add_migrations` |
| `src/App.tsx` | 界面、连接、refresh 骨架已给 | 4 条 SQL（查询/合计/插入/删除） |

> **前端基础提示**：`Database` 来自 `@tauri-apps/plugin-sql`（练习版已装）。`db.execute` 改数据、`db.select` 查数据。

## lib.rs TODO 1：建表迁移

### 练习版这里是什么

```rust
let migrations = vec![Migration {
    version: 1,
    description: "create_expenses",
    sql: "", // ← 替换成你的代码
    kind: MigrationKind::Up,
}];
```

### 答案版填了什么

```rust
    sql: "CREATE TABLE IF NOT EXISTS expenses (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        title TEXT NOT NULL,
        amount REAL NOT NULL,
        category TEXT NOT NULL,
        created_at TEXT NOT NULL
    );",
```

### 为什么

- `INTEGER PRIMARY KEY AUTOINCREMENT`：自增主键——插入时不填 id，数据库自动编号
- `NOT NULL`：必填字段，防止脏数据
- `REAL`：SQLite 的浮点类型（存金额）
- 迁移在 `Database.load` 时自动执行；`IF NOT EXISTS` 保证重复执行安全

### 回查文档

[第 2 节：接入 sql 插件](a05_expense_tracker.md#sec-a05-migration)、[第 4 节：SQL 语句](a05_expense_tracker.md#sec-a05-sql)。

## App.tsx TODO 1：查询全部支出

### 答案版填了什么

```typescript
const rows = await db.select<Expense[]>(
  "SELECT id, title, amount, category, created_at AS createdAt FROM expenses ORDER BY id DESC"
);
```

### 为什么

- `db.select<Expense[]>`：返回行数组，泛型声明 TS 类型
- `ORDER BY id DESC`：主键倒序 = 最新在前
- **`created_at AS createdAt`**：给列起别名，让前端字段名是驼峰（`createdAt`），与 TS 接口对齐——避免 `created_at` 到前端的大小写不一致问题

### 回查文档

[第 3 节：execute / select](a05_expense_tracker.md#sec-a05-api)。

## App.tsx TODO 2：月度合计

### 答案版填了什么

```typescript
const months = await db.select<MonthTotal[]>(
  "SELECT strftime('%Y-%m', created_at) AS month, SUM(amount) AS total " +
    "FROM expenses GROUP BY month ORDER BY month DESC"
);
```

### 为什么

- `strftime('%Y-%m', created_at)`：把时间截到月份，别名 `month`
- `GROUP BY month`：按月份分组；`SUM(amount)`：每组求和
- `ORDER BY month DESC`：最近月份在前
- 一句 SQL 完成"按月统计"——数据库引擎替你把循环和求和干了

### 回查文档

[第 5 节：聚合统计](a05_expense_tracker.md#sec-a05-sum)。

## App.tsx TODO 3：插入支出

### 答案版填了什么

```typescript
await db.execute(
  "INSERT INTO expenses (title, amount, category, created_at) VALUES ($1, $2, $3, datetime('now','localtime'))",
  [title.trim(), value, category]
);
```

### 为什么

- `$1 $2 $3` 占位符对应 `[title, amount, category]`——**值绝不拼进 SQL 字符串**（防注入）
- `datetime('now','localtime')`：**SQLite 自己生成当前时间**，前端不用处理日期、也传不进去
- `execute` 用于改数据（INSERT/DELETE），返回影响行数（这里忽略）

### 回查文档

[第 4 节：参数绑定](a05_expense_tracker.md#sec-a05-sql)。

## App.tsx TODO 4：删除

### 答案版填了什么

```typescript
await db.execute("DELETE FROM expenses WHERE id = $1", [id]);
```

### 为什么

- `WHERE id = $1`：只删 id 匹配的那一条，不影响其他行
- 删除后调用 `refresh(db)` 重新拉列表和合计——**数据变了就刷新，保持界面一致**

## 验收标准

```bash
cd 02_mini_apps/a05_expense_tracker
cargo tauri dev
```

添加几笔支出 → 列表显示、月度合计更新；删除 → 记录消失；重启应用 → 数据还在（SQLite 落盘）。数据库文件在应用配置目录下（`expenses.db`）。

**破坏性验证**（确认你是理解了，而不是碰巧对）：

- 把 `db.execute` 的参数绑定改成字符串拼接 → 输入含 `'` 的内容时行为异常（验证参数绑定）
- 把 `sql:allow-execute` 从 capabilities 删掉 → 点添加报"未授权"（验证写操作需要额外权限）
- 把 `GROUP BY month` 删掉 → 只显示一行"合计"（验证分组的作用）
- 把 `ORDER BY id DESC` 改成 `ASC` → 列表顺序反过来（验证排序）

## 升级挑战（可选）

- 按分类统计：`GROUP BY category`，做一个"本月各分类占比"
- 加"最近 7 天"筛选：`WHERE created_at >= datetime('now','-7 days')`
