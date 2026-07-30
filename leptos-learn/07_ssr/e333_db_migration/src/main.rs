// ============================================================
// 练习 e333: DB Migration — SQLx 数据库迁移管理
//
// 核心知识点:
//   - sqlx migrate CLI: migrate add/run/revert
//   - 迁移文件命名与版本控制
//   - Up / Down 迁移脚本编写
//   - 迁移状态追踪与验证
//
// 难度: ⭐⭐ (TODO 约 50%)
// ============================================================

use leptos::prelude::*;

// 迁移 Up SQL 模板
const USERS_UP: &str = "\
CREATE TABLE users (
    id           SERIAL PRIMARY KEY,
    username     VARCHAR(100) NOT NULL UNIQUE,
    email        VARCHAR(255) NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    created_at   TIMESTAMPTZ DEFAULT NOW(),
    updated_at   TIMESTAMPTZ DEFAULT NOW()
);
CREATE INDEX idx_users_email ON users(email);";

// 迁移 Down SQL 模板
const USERS_DOWN: &str = "DROP TABLE IF EXISTS users;";

const POSTS_UP: &str = "\
CREATE TABLE posts (
    id         SERIAL PRIMARY KEY,
    user_id    INTEGER NOT NULL REFERENCES users(id),
    title      VARCHAR(255) NOT NULL,
    body       TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW()
);
CREATE INDEX idx_posts_user_id ON posts(user_id);";

const POSTS_DOWN: &str = "DROP TABLE IF EXISTS posts;";

// sqlx CLI 命令参考
const CLI_COMMANDS: &str = "\
# 创建新迁移
sqlx migrate add -r create_users

# 应用所有待定迁移
sqlx migrate run

# 回滚最后一个迁移
sqlx migrate revert

# 查看迁移状态
sqlx migrate info

# 在 Rust 代码中运行迁移
// sqlx::migrate!().run(&pool).await?;";

// TODO: 定义迁移条目结构体
// 包含: version (u64), name (&'static str), up_sql (&'static str),
//       down_sql (&'static str), applied (bool)
#[derive(Debug, Clone)]
struct Migration {
    version: u64,
    name: &'static str,
    up_sql: &'static str,
    down_sql: &'static str,
    applied: bool,
}

#[component]
fn SqlBlock(sql: &'static str, label: &'static str) -> impl IntoView {
    view! {
        <div style="margin-bottom: 8px;">
            <div style="font-size: 12px; color: #666; margin-bottom: 4px;">{label}</div>
            <pre style="background: #1e1e1e; color: #d4d4d4; padding: 8px; border-radius: 4px; font-size: 12px; overflow-x: auto; margin: 0;">
                {sql}
            </pre>
        </div>
    }
}

#[component]
fn MigrationCard(
    migration: Migration,
    on_run: Option<fn()>,
    on_revert: Option<fn()>,
) -> impl IntoView {
    // TODO: 显示迁移卡片
    // 包含: version, name, status, Up SQL, Down SQL
    // 如果 on_run 存在且未应用，显示 "Run" 按钮
    // 如果 on_revert 存在且已应用，显示 "Revert" 按钮
    view! {
        <div style="border: 1px solid #ddd; border-radius: 6px; margin-bottom: 12px; overflow: hidden;">
            <div style="display: flex; align-items: center; padding: 10px 14px; background: #f8f9fa; gap: 12px;">
                <span style="font-family: monospace; font-weight: bold; color: #555;">
                    {migration.version}
                </span>
                <span style="font-family: monospace; flex: 1;">
                    {migration.name}
                </span>
                <span>
                    {if migration.applied { "✅ 已应用" } else { "⏳ 待定" }}
                </span>
            </div>
            <div style="padding: 8px 14px;">
                <SqlBlock sql={migration.up_sql} label="Up" />
                <SqlBlock sql={migration.down_sql} label="Down" />
            </div>
        </div>
    }
}

#[component]
fn Exercise() -> impl IntoView {
    // TODO: 创建迁移列表信号
    let (migrations, set_migrations) = signal(vec![
        Migration { version: 20240701000001, name: "create_users", up_sql: USERS_UP, down_sql: USERS_DOWN, applied: false },
        Migration { version: 20240701000002, name: "create_posts", up_sql: POSTS_UP, down_sql: POSTS_DOWN, applied: false },
    ]);

    // TODO: 创建操作日志信号
    let (log, set_log) = signal(Vec::<String>::new());

    // TODO: 实现 run_pending 函数
    // 将所有待定迁移标记为已应用，记录日志

    // TODO: 实现 revert_last 函数
    // 将最后一个已应用的迁移标记为待定，记录日志

    // TODO: 实现 reset 函数
    // 将所有迁移标记为待定，清空日志

    view! {
        <div style="font-family: system-ui, sans-serif; max-width: 900px; margin: 20px auto; padding: 0 16px;">
            <h2>"🗄️ 数据库迁移管理 (sqlx)"</h2>
            <p style="color: #666; margin-bottom: 16px;">
                "使用 sqlx migrate CLI 管理数据库 schema 版本变更。每批迁移包含 Up（应用）和 Down（回滚）脚本。"
            </p>

            // TODO: 操作按钮区域
            // "▶️ Run Pending" — 执行所有待定迁移
            // "⏪ Revert Last" — 回滚最近一次迁移
            // "🔄 Reset" — 重置所有迁移

            // TODO: 迁移文件列表
            // <h3>"迁移文件"</h3>
            // 遍历 migrations 渲染 MigrationCard

            // TODO: CLI 命令参考
            <h3>"sqlx CLI 命令"</h3>
            <pre style="background: #f5f5f5; padding: 12px; border-radius: 4px; font-size: 13px;">
                {CLI_COMMANDS}
            </pre>

            // TODO: 操作日志
            // <h3>"📝 操作日志"</h3>
            // 显示每条日志记录

            // 最佳实践说明
            <div style="margin-top: 20px; padding: 12px; background: #f0f8ff; border-radius: 6px; font-size: 13px; color: #333;">
                <strong>"💡 迁移最佳实践: "</strong>
                "每次迁移应小而专注，只修改一个逻辑单元。Up 和 Down 必须对称。"
                "生产环境中使用 sqlx migrate run 自动应用待定迁移，部署前在 staging 环境验证。"
            </div>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
