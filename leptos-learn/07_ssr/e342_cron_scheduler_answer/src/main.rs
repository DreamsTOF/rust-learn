// ============================================================
// Exercise e342 — Answer: Cron Scheduler
//
// Core: tokio-cron-scheduler, periodic tasks, background jobs
// ============================================================

use leptos::prelude::*;

const CRON_EVERY_MINUTE: &str = "0 * * * * *";
const CRON_EVERY_HOUR: &str = "0 0 * * * *";
const CRON_DAILY_MIDNIGHT: &str = "0 0 0 * * *";
const CRON_WEEKLY_SUNDAY: &str = "0 0 0 * * 0";

#[derive(Debug, Clone)]
enum ScheduledTask {
    LogCleanup,
    DatabaseBackup,
    CacheWarmUp,
    ReportGeneration,
}

fn configure_scheduler() -> Result<(), String> {
    use tokio_cron_scheduler::{JobScheduler, Job};

    let mut scheduler = JobScheduler::new();

    let log_job = Job::new_async(CRON_DAILY_MIDNIGHT, |_uuid, _lock| {
        Box::pin(async move {
            tracing::info!("Running log cleanup task...");
        })
    }).map_err(|e| format!("Job creation error: {}", e))?;

    scheduler.add(log_job);

    let cache_job = Job::new_async(CRON_EVERY_HOUR, |_uuid, _lock| {
        Box::pin(async move {
            tracing::info!("Warming up cache...");
        })
    }).map_err(|e| format!("Job creation error: {}", e))?;

    scheduler.add(cache_job);

    scheduler.start().map_err(|e| format!("Scheduler start error: {}", e))?;
    Ok(())
}

#[component]
fn Exercise() -> impl IntoView {
    const SCHEDULER_CODE: &str = "\
use tokio_cron_scheduler::{JobScheduler, Job};

#[tokio::main]
async fn main() {
    let mut scheduler = JobScheduler::new();

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
        ("Log Cleanup", CRON_DAILY_MIDNIGHT, "Delete log files older than 30 days"),
        ("Database Backup", CRON_WEEKLY_SUNDAY, "Full DB backup + WAL archive"),
        ("Cache Warm-Up", CRON_EVERY_HOUR, "Refresh Redis hot cache"),
        ("Report Generation", CRON_EVERY_MINUTE, "Aggregate real-time stats"),
    ];

    view! {
        <div>
            <h1>"Cron Scheduler — Periodic Tasks"</h1>

            <section>
                <h2>"tokio-cron-scheduler Example"</h2>
                <pre>{SCHEDULER_CODE}</pre>
            </section>

            <section>
                <h2>"Scheduled Tasks"</h2>
                <table>
                    <thead>
                        <tr>
                            <th>"Task"</th>
                            <th>"Cron Expression"</th>
                            <th>"Description"</th>
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
                <h2>"Cron Expression Format"</h2>
                <pre>{"\
┌───────── second (0-59)
│ ┌──────── minute (0-59)
│ │ ┌─────── hour (0-23)
│ │ │ ┌────── day of month (1-31)
│ │ │ │ ┌───── month (1-12)
│ │ │ │ │ ┌──── day of week (0-6, 0=Sun)
│ │ │ │ │ │
* * * * * *"}</pre>
            </section>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
