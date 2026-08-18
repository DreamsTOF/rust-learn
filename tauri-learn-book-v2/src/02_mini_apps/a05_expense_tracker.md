# 练习 A05: 记账本

## 为什么要学这个

前面存数据用内存（A01 的 `Vec`）、存文件（A02 的 `note.txt`）。但当数据多起来、要**查询、汇总、删除单条**时，它们都不合适：

1. **内存**：应用一关就没了；
2. **文件**：每次都得整个文件读出来，自己写代码筛选、求和，还要处理格式。

这一章要回答三个问题：

1. **数据库解决了什么？** —— 为什么"记流水账 + 按月合计"是数据库最典型的场景？
2. **Tauri 怎么用数据库？** —— sql 插件怎么接入？建表、增删改查、聚合各长什么样？
3. **为什么要小心 SQL？** —— 参数绑定（`$1`）防的是什么？

学完你会发现：**换数据库只是把"存储"换了个引擎，命令、状态、错误处理这些套路原封不动。**

---

## 从问题出发

练习 A05 要做的事：**一个记账本——记每笔支出（标题、金额、分类），按月份看合计，能删。**

**核心矛盾：** 记账数据既要**持久化**（关掉还在），又要**能查询统计**（"这个月花了多少？"）。用文件硬算当然能行，但每加一条都重写整个文件、每次合计都全量扫描——又慢又容易出错。这正是**数据库**的舞台：数据落在磁盘（持久化），查询和聚合交给数据库引擎（`SELECT ... WHERE / GROUP BY / SUM`）。

所以本课不用"命令存 Vec"（A01）也不用"文件读写"（A02），而是引入 **SQLite**——一个文件就是一个数据库，零服务、免安装，桌面应用首选。

```text
前端 (React)                          Rust 进程
┌────────────────────────┐            ┌───────────────────────────────┐
│ Database.load           │ ──连接──► │ sql 插件 + 迁移（建表）        │
│ execute(INSERT/DELETE)  │ ────────► │ SQLite（expenses.db）          │
│ select(SELECT/SUM)      │ ◄──────── │ 数据落盘 · 引擎负责查询聚合     │
└────────────────────────┘            └───────────────────────────────┘
```

<a id="sec-a05-why"></a>
## 1. 为什么是数据库

| 存储 | 持久化 | 查询/统计 | 适合 |
|------|:---:|:---:|------|
| 内存 `Vec`（A01） | ✗ | 自己写循环 | 临时状态 |
| 文件（A02） | ✓ | 全量读 + 自己算 | 单一内容 |
| **数据库（本课）** | ✓ | **SQL 交给引擎** | 多条结构化数据 + 统计 |

**SQL 的核心价值：** 你只要说"要什么"（`SELECT ... WHERE 分类='餐饮' GROUP BY 月份 SUM(金额)`），**怎么找、怎么算由数据库引擎负责**。数据量从几十条涨到几百万条，你的代码一行都不用改。

<a id="sec-a05-migration"></a>
## 2. 接入 sql 插件 — 注册 + 迁移建表

### Rust 端：注册插件 + 声明"建表迁移"

```rust
use tauri_plugin_sql::{Migration, MigrationKind};

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
```

- `sqlite:expenses.db`：连接串。SQLite 会把数据库文件建在应用配置目录，**一个文件就是一个库**
- **迁移（migration）**：描述"数据库应该长什么样"的版本化脚本。`version: 1` 表示第一个版本；`CREATE TABLE IF NOT EXISTS` 保证重复执行不报错
- **迁移什么时候跑？** 前端第一次 `Database.load("sqlite:expenses.db")` 时，插件自动把还没跑过的迁移执行一遍（建表）。这是"**schema 由 Rust 管，使用由前端管**"的分工

> **为什么建表放在 Rust 而不是前端？** 如果建表语句散落在前端各处，项目大起来就是灾难。把 schema 声明集中在迁移里，升级表结构就是加一条 `version: 2` 的迁移——**数据库的"版本管理"**。

<a id="sec-a05-api"></a>
## 3. 前端 Database API — load / execute / select

### 连接数据库

```typescript
import Database from "@tauri-apps/plugin-sql";

const db = await Database.load("sqlite:expenses.db");
```

`Database.load` 连接数据库（触发迁移），返回一个连接对象。**连接一次，反复使用**——所以放在 React 里存成 state，挂载时连一次。

### 改数据用 `execute`，查数据用 `select`

```typescript
// 插入（execute 执行不返回行的语句）
await db.execute(
  "INSERT INTO expenses (title, amount, category, created_at) VALUES ($1, $2, $3, datetime('now','localtime'))",
  [title, amount, category]
);

// 查询（select 返回行数组）
const rows = await db.select<Expense[]>(
  "SELECT id, title, amount, category, created_at AS createdAt FROM expenses ORDER BY id DESC"
);
```

| API | 用在哪 | 返回 |
|-----|--------|------|
| `db.execute(query, values)` | INSERT / DELETE / UPDATE | 影响行数等 |
| `db.select<T>(query, values)` | SELECT | `T[]`（行数组） |

### 权限（练习版已配好）

sql 插件是**插件 API**，前端调用要权限：

```json
"permissions": ["core:default", "sql:default", "sql:allow-execute"]
```

- `sql:default`：允许连接（load）、关闭、**读**（select）
- `sql:allow-execute`：**写**（insert/delete）是额外权限，不加就报"未授权"

> 回想 A02 的结论：自己写的命令不需要权限，**插件 API 要权限**——sql 是插件，所以 `sql:*` 权限必须显式配。

<a id="sec-a05-sql"></a>
## 4. SQL 语句与参数绑定

本课的四条 SQL 就是全套"增删改查"：

```sql
-- 建表（迁移里）
CREATE TABLE IF NOT EXISTS expenses (
  id INTEGER PRIMARY KEY AUTOINCREMENT,   -- 自增主键：数据库自动编号
  title TEXT NOT NULL,
  amount REAL NOT NULL,
  category TEXT NOT NULL,
  created_at TEXT NOT NULL
);

-- 增（INSERT）：$1 $2 $3 是占位符，对应 values 数组
INSERT INTO expenses (title, amount, category, created_at)
VALUES ($1, $2, $3, datetime('now','localtime'));

-- 查（SELECT）：按 id 倒序，最新在前
SELECT id, title, amount, category, created_at AS createdAt
FROM expenses ORDER BY id DESC;

-- 删（DELETE）：只删指定 id 那一条
DELETE FROM expenses WHERE id = $1;
```

**参数绑定（`$1`、`$2`）是安全底线：** 值永远通过 values 数组传，**绝不拼进 SQL 字符串**。

```typescript
// ❌ 危险：把用户输入拼进 SQL（SQL 注入）
db.execute(`INSERT ... VALUES ('${title}', ...)`);

// ✅ 正确：占位符 + 参数数组
db.execute("INSERT ... VALUES ($1, $2, $3)", [title, amount, category]);
```

如果用户输入 `'; DROP TABLE expenses; --`，拼字符串会把你的表删光；参数绑定则把它当成普通字符串。

<a id="sec-a05-sum"></a>
## 5. 聚合统计 — GROUP BY + SUM

"按月份合计"是数据库的强项，一句 SQL 就完成：

```sql
SELECT strftime('%Y-%m', created_at) AS month, SUM(amount) AS total
FROM expenses
GROUP BY month
ORDER BY month DESC;
```

拆开看三件事：

1. **`strftime('%Y-%m', created_at)`**：把 `"2025-08-15 10:30:00"` 截成 `"2025-08"`——"按月"就是把时间归到月份
2. **`GROUP BY month`**：按月份分组，同组的行合并成一组
3. **`SUM(amount)`**：对每组求和——这就是"这个月花了多少"

结果长这样：

```
month     total
2025-08   1234.56
2025-07   980.00
```

> 之前用文件"自己算"的活，这里全部交给数据库引擎——**这就是为什么要学 SQL**：声明式地表达"要什么"，引擎负责"怎么做"。

---

## 练习指引

**作业范围：** 动 2 个文件，共 7 处 TODO。

| 文件 | 步骤 | 内容 |
|------|------|------|
| `src-tauri/src/lib.rs` | 1 | 写建表迁移语句（五个字段） |
| `src-tauri/src/lib.rs` | 2 | `add_migrations` 注册迁移 |
| `src/App.tsx` | 1 | `SELECT ... ORDER BY id DESC` |
| `src/App.tsx` | 2 | 月度合计 `GROUP BY month` + `SUM` |
| `src/App.tsx` | 3 | `INSERT ... VALUES ($1,$2,$3, datetime(...))` |
| `src/App.tsx` | 4 | `DELETE ... WHERE id = $1` |

**怎么验证：**

```bash
cd 02_mini_apps/a05_expense_tracker
cargo tauri dev
```

填"买了什么 / 金额 / 分类"点添加 → 列表出现记录，月度合计更新；点删除 → 记录消失。关掉应用再启动 → 数据还在（SQLite 落盘）。

**故意踩坑看效果：** 把 `$1` 占位符写成把值直接拼进字符串 → 输入特殊字符时行为诡异（详见讲解第 4 节）；把 `sql:allow-execute` 从 capabilities 删掉 → 点添加报"未授权"。

---

## 知识点连起来看

```text
Rust: 注册 sql 插件 + 迁移（建表）       ← schema 管理，前端 load 时自动执行
        │
前端:  Database.load("sqlite:expenses.db")  ← 连接（触发迁移）
        │
db.execute(INSERT/DELETE, [$1,...])      ← 写：参数绑定防注入
db.select<Expense[]>(SELECT, [...])      ← 读：返回行数组
        │
SELECT ... GROUP BY month SUM(amount)    ← 聚合：统计交给引擎
        │
capabilities: sql:default + sql:allow-execute   ← 插件权限
```

| 层 | 解决的问题 | 关键概念 |
|----|-----------|---------|
| 存储选型 | 数据多了怎么办 | 内存 vs 文件 vs 数据库 |
| 接入 | 插件怎么装 | 注册、迁移、`Database.load` |
| 增删改查 | 数据怎么操作 | `execute` / `select`、参数绑定 `$1` |
| 聚合 | 统计怎么做 | `GROUP BY`、`SUM`、`strftime` |
| 权限 | 前端为什么能调 | `sql:default`、`sql:allow-execute` |

**一通百通的核心：** 这一课换的是**存储引擎**——从"内存 Vec"换到"SQLite"，但应用的骨架（React 状态、命令、错误处理、权限配置）完全没变。数据库的价值在于：**把"查什么、怎么算"从代码里抽出来交给引擎**。

**递进关系：** 练习 A06（汇率查询）转向"**应用与外界的连接**"——HTTP 请求拿外部数据，并用 Store 缓存避免重复请求。
