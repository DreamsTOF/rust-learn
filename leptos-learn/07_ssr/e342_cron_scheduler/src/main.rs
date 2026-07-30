// ============================================================
// 练习 e342: Cron Scheduler — 定时任务与后台作业
//
// 核心知识点:
//   - tokio-cron-scheduler: 基于 cron 表达式的任务调度
//   - 后台任务: 异步定时执行
//   - 任务生命周期: 启动、停止、重试
//
// 难度: ⭐⭐⭐ (少量 TODO)
// ============================================================

use leptos::prelude::*;

// TODO: 定义定时任务配置
// ⭐⭐⭐ 提示: 使用 tokio-cron-scheduler 的 JobScheduler
// 添加多个 Job，每个 Job 有唯一的 cron 表达式和异步回调

/// 定时任务示例配置（cron 表达式格式: 秒 分 时 日 月 周）
const CRON_EVERY_MINUTE: &str = "0 * * * * *";
const CRON_EVERY_HOUR: &str = "0 0 * * * *";
const CRON_DAILY_MIDNIGHT: &str = "0 0 0 * * *";
const CRON_WEEKLY_SUNDAY: &str = "0 0 0 * * 0";

/// 任务类型枚举
#[derive(Debug, Clone)]
enum ScheduledTask {
    /// 日志清理（每天凌晨）
    LogCleanup,
    /// 数据库备份（每周日）
    DatabaseBackup,
    /// 缓存预热（每小时）
    CacheWarmUp,
    /// 统计报表生成（每分钟 — 演示用）
    ReportGeneration,
}

// TODO: 实现任务调度器配置函数
// ⭐⭐⭐ 使用 JobScheduler::new() 创建调度器
// 为每个任务添加 Job::new_async(cron_expr).with_callback(...)
// 调用 scheduler.start() 启动
// 提示: 需要 tokio runtime
fn configure_scheduler() -> Result<(), String> {
    // 示意实现:
    // 实际代码:
    //
    // use tokio_cron_scheduler::{JobScheduler, Job, JobToRun};
    //
    // let mut scheduler = JobScheduler::new();
    //
    // // 每日日志清理
    // let log_job = Job::new_async(CRON_DAILY_MIDNIGHT, |_uuid, _lock| {
    //     Box::pin(async move {
    //         tracing::info!("Running log cleanup task...");
    //         // 清理过期日志文件
    //     })
    // }).map_err(|e| format!("Job creation error: {}", e))?;
    //
    // scheduler.add(log_job);
    //
    // // 每小时缓存预热
    // let cache_job = Job::new_async(CRON_EVERY_HOUR, |_uuid, _lock| {
    //     Box::pin(async move {
    //         tracing::info!("Warming up cache...");
    //         // 刷新热点缓存
    //     })
    // }).map_err(|e| format!("Job creation error: {}", e))?;
    //
    // scheduler.add(cache_job);
    //
    // scheduler.start().map_err(|e| format!("Scheduler start error: {}", e))?;

    Ok(())
}

#[component]
fn Exercise() -> impl IntoView {
    const SCHEDULER_CODE: &str = "\
use tokio_cron_scheduler::{JobScheduler, Job};

#[tokio::main]
async fn main() {
    let mut scheduler = JobScheduler::new();

    // 每分钟执行
    let job = Job::new_async(\"0 * * * * *\", |_uuid, _lock| {
        Box::pin(async move {
            println!(\"Task executed at {:?}\", chrono::Utc::now());
        })
    }).unwrap();
    scheduler.add(job).await;

    scheduler.start().await;
    tokio::signal::ctrl_c().await.ok();
    scheduler.shutdown().await;
}";

    let tasks = vec![
        ("Log Cleanup", CRON_DAILY_MIDNIGHT, "删除 30 天前的日志文件"),
        ("Database Backup", CRON_WEEKLY_SUNDAY, "全量数据库备份 + WAL 归档"),
        ("Cache Warm-Up", CRON_EVERY_HOUR, "刷新 Redis 热点缓存"),
        ("Report Generation", CRON_EVERY_MINUTE, "聚合实时统计数据"),
    ];

    view! {
        <div>
            <h1>"Cron Scheduler — 定时任务调度"</h1>

            <section>
                <h2>"tokio-cron-scheduler 示例"</h2>
                <pre>{SCHEDULER_CODE}</pre>
            </section>

            <section>
                <h2>"定时任务列表"</h2>
                <table>
                    <thead>
                        <tr>
                            <th>"任务名称"</th>
                            <th>"Cron 表达式"</th>
                            <th>"说明"</th>
                        </tr>
                    </thead>
                    <tbody>
                        {tasks.into_iter().map(|(name, expr, desc)| view! {
                            <tr>
                                <td><strong>{name}</strong></td>
                                <td><code>{expr}</code></td>
                                <td>{desc}</td>
                            </tr>
                        }).collect::<Vec<_>>()}
                    </tbody>
                </table>
            </section>

            <section>
                <h2>"Cron 表达式格式"</h2>
                <pre>{"\
┌───────── 秒 (0-59)
│ ┌──────── 分 (0-59)
│ │ ┌─────── 时 (0-23)
│ │ │ ┌────── 日 (1-31)
│ │ │ │ ┌───── 月 (1-12)
│ │ │ │ │ ┌──── 周 (0-6, 0=周日)
│ │ │ │ │ │
* * * * * *"}</pre>
            </section>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
