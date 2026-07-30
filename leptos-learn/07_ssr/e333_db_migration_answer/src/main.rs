// ============================================================
// Exercise e333 — Answer: DB Migration
//
// Core: sqlx migrate CLI, migration files, up/down migrations,
//       migration state tracking
// ============================================================

use leptos::prelude::*;

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
            <pre style="background:#1e1e1e;color:#d4d4d4;padding:8px;border-radius:4px;font-size:12px;overflow-x:auto;margin:0;">
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
    view! {
        <div style="border:1px solid #ddd;border-radius:6px;margin-bottom:12px;overflow:hidden;">
            <div style="display:flex;align-items:center;padding:10px 14px;background:#f8f9fa;gap:12px;">
                <span style="font-family:monospace;font-weight:bold;color:#555;">
                    {migration.version}
                </span>
                <span style="font-family:monospace;flex:1;">
                    {migration.name}
                </span>
                <span>
                    {if migration.applied { "✅ 已应用" } else { "⏳ 待定" }}
                </span>
            </div>
            <div style="padding:8px 14px;">
                <SqlBlock sql={migration.up_sql} label="Up" />
                <SqlBlock sql={migration.down_sql} label="Down" />
            </div>
        </div>
    }
}

#[component]
fn Exercise() -> impl IntoView {
    let (migrations, set_migrations) = signal(vec![
        Migration { version: 20240701000001, name: "create_users", up_sql: USERS_UP, down_sql: USERS_DOWN, applied: false },
        Migration { version: 20240701000002, name: "create_posts", up_sql: POSTS_UP, down_sql: POSTS_DOWN, applied: false },
    ]);

    let (log, set_log) = signal(Vec::new());

    let run_pending = move || {
        set_migrations.update(|migs| {
            for m in migs.iter_mut() {
                if !m.applied {
                    m.applied = true;
                }
            }
        });
        set_log.update(|l| l.push("▶️ Run Pending: 所有待定迁移已应用".to_string()));
    };

    let revert_last = move || {
        set_migrations.update(|migs| {
            if let Some(last) = migs.iter_mut().rev().find(|m| m.applied) {
                last.applied = false;
                set_log.update(|l| l.push(format!("⏪ Revert: 已回滚迁移 {}", last.version)));
            } else {
                set_log.update(|l| l.push("⏪ Revert: 没有已应用的迁移可以回滚".to_string()));
            }
        });
    };

    let reset = move || {
        set_migrations.update(|migs| {
            for m in migs.iter_mut() {
                m.applied = false;
            }
        });
        set_log.set(Vec::new());
    };

    view! {
        <div style="font-family:system-ui,sans-serif;max-width:900px;margin:20px auto;padding:0 16px;">
            <h2>"🗄️ 数据库迁移管理 (sqlx)"</h2>
            <p style="color:#666;margin-bottom:16px;">
                "使用 sqlx migrate CLI 管理数据库 schema 版本变更。每批迁移包含 Up（应用）和 Down（回滚）脚本。"
            </p>

            <div style="display:flex;gap:8px;margin-bottom:16px;flex-wrap:wrap;">
                <button on:click=move |_| run_pending()
                    style="background:#28a745;color:white;border:none;padding:8px 16px;border-radius:4px;cursor:pointer;">
                    "▶️ Run Pending"
                </button>
                <button on:click=move |_| revert_last()
                    style="background:#ff8800;color:white;border:none;padding:8px 16px;border-radius:4px;cursor:pointer;">
                    "⏪ Revert Last"
                </button>
                <button on:click=move |_| reset()
                    style="background:#6c757d;color:white;border:none;padding:8px 16px;border-radius:4px;cursor:pointer;">
                    "🔄 Reset"
                </button>
            </div>

            <h3>"迁移文件"</h3>
            {move || migrations.get().into_iter().map(|m| {
                view! {
                    <MigrationCard migration={m} on_run={None} on_revert={None} />
                }
            }).collect_view()}

            <h3>"sqlx CLI 命令"</h3>
            <pre style="background:#f5f5f5;padding:12px;border-radius:4px;font-size:13px;">
                {CLI_COMMANDS}
            </pre>

            <h3>"📝 操作日志"</h3>
            <div style="background:#f8f9fa;padding:8px 12px;border-radius:4px;min-height:40px;font-family:monospace;font-size:13px;">
                {move || {
                    let entries = log.get();
                    if entries.is_empty() {
                        "暂无操作".to_string()
                    } else {
                        entries.join("\n")
                    }
                }}
            </div>

            <div style="margin-top:20px;padding:12px;background:#f0f8ff;border-radius:6px;font-size:13px;color:#333;">
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
