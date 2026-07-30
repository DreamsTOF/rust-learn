// ============================================================
// 练习 e331: log_collection — 参考答案
//
// 核心知识点:
//   - tracing 订阅者配置
//   - 结构化 JSON 日志输出
//   - 日志级别过滤（error/warn/info/debug/trace）
//   - 自定义日志格式与输出目标
// ============================================================

use leptos::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

impl LogLevel {
    fn icon(self) -> &'static str {
        match self {
            LogLevel::Error => "🔴",
            LogLevel::Warn => "🟡",
            LogLevel::Info => "🔵",
            LogLevel::Debug => "🟢",
            LogLevel::Trace => "⚪",
        }
    }

    fn label(self) -> &'static str {
        match self {
            LogLevel::Error => "ERROR",
            LogLevel::Warn => "WARN",
            LogLevel::Info => "INFO",
            LogLevel::Debug => "DEBUG",
            LogLevel::Trace => "TRACE",
        }
    }
}

#[component]
fn LogEntry(
    level: LogLevel,
    message: String,
    timestamp: String,
) -> impl IntoView {
    view! {
        <tr>
            <td style:color={match level {
                LogLevel::Error => "red",
                LogLevel::Warn => "orange",
                LogLevel::Info => "blue",
                LogLevel::Debug => "green",
                LogLevel::Trace => "gray",
            }}>
                {level.icon()} " " {level.label()}
            </td>
            <td>{message}</td>
            <td>{timestamp}</td>
        </tr>
    }
}

#[component]
fn LogLevelFilter(active: LogLevel) -> impl IntoView {
    view! {
        <option value={active.label()}>
            {active.icon()} " " {active.label()}
        </option>
    }
}

#[component]
fn JsonLogViewer(raw: String) -> impl IntoView {
    view! {
        <pre style="background: #1e1e1e; color: #d4d4d4; padding: 8px; border-radius: 4px; font-size: 12px; overflow-x: auto;">
            {raw}
        </pre>
    }
}

#[component]
fn Exercise() -> impl IntoView {
    let (logs, set_logs) = signal(Vec::new());
    let (filter_level, set_filter_level) = signal(LogLevel::Info);
    let (json_raw, set_json_raw) = signal(String::new());
    let (show_json, set_show_json) = signal(false);

    let add_log_entry = move |level: LogLevel, message: String| {
        use std::time::SystemTime;
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        let timestamp = format_timestamp(now);
        set_logs.update(|l| l.push((level, message, timestamp)));
    };

    view! {
        <div style="font-family: system-ui, sans-serif; max-width: 800px; margin: 20px auto; padding: 0 16px;">
            <h2>"📋 结构化日志采集系统"</h2>
            <p style="color: #666; margin-bottom: 16px;">
                "演示 tracing 订阅者模式、日志级别过滤与 JSON 结构化输出"
            </p>

            <div style="display: flex; gap: 8px; margin-bottom: 16px; flex-wrap: wrap;">
                <button on:click=move |_| add_log_entry(LogLevel::Error, "数据库连接超时".to_string())
                    style="background:#ff4444;color:white;border:none;padding:6px 14px;border-radius:4px;cursor:pointer;">
                    "🔴 Error"
                </button>
                <button on:click=move |_| add_log_entry(LogLevel::Warn, "API 响应时间超过 2s".to_string())
                    style="background:#ff8800;color:white;border:none;padding:6px 14px;border-radius:4px;cursor:pointer;">
                    "🟡 Warn"
                </button>
                <button on:click=move |_| add_log_entry(LogLevel::Info, "用户登录成功: user_id=42".to_string())
                    style="background:#0088ff;color:white;border:none;padding:6px 14px;border-radius:4px;cursor:pointer;">
                    "🔵 Info"
                </button>
                <button on:click=move |_| add_log_entry(LogLevel::Debug, "缓存命中 key=session_abc".to_string())
                    style="background:#00aa44;color:white;border:none;padding:6px 14px;border-radius:4px;cursor:pointer;">
                    "🟢 Debug"
                </button>
                <button on:click=move |_| add_log_entry(LogLevel::Trace, "渲染组件: Header > Nav > UserMenu".to_string())
                    style="background:#888;color:white;border:none;padding:6px 14px;border-radius:4px;cursor:pointer;">
                    "⚪ Trace"
                </button>
            </div>

            <div style="margin-bottom: 12px; display: flex; align-items: center; gap: 8px;">
                <label>"过滤级别:"</label>
                <select on:change=move |ev| {
                    let val = event_target_value(&ev);
                    set_filter_level.set(match val.as_str() {
                        "ERROR" => LogLevel::Error,
                        "WARN" => LogLevel::Warn,
                        "INFO" => LogLevel::Info,
                        "DEBUG" => LogLevel::Debug,
                        "TRACE" => LogLevel::Trace,
                        _ => LogLevel::Info,
                    });
                } style="padding:4px 8px;border-radius:4px;">
                    <LogLevelFilter active={LogLevel::Error} />
                    <LogLevelFilter active={LogLevel::Warn} />
                    <LogLevelFilter active={LogLevel::Info} />
                    <LogLevelFilter active={LogLevel::Debug} />
                    <LogLevelFilter active={LogLevel::Trace} />
                </select>

                <label style="margin-left: 16px;">
                    <input type="checkbox"
                        prop:checked={show_json}
                        on:change=move |_| set_show_json.update(|v| *v = !*v)
                    />
                    " JSON 视图"
                </label>

                <button on:click=move |_| set_logs.set(Vec::new())
                    style="margin-left:auto;background:#eee;border:1px solid #ccc;padding:4px 12px;border-radius:4px;cursor:pointer;">
                    "🗑️ 清空"
                </button>
            </div>

            <div style="border:1px solid #ddd;border-radius:6px;overflow:hidden;">
                <table style="width:100%;border-collapse:collapse;">
                    <thead style="background:#f5f5f5;">
                        <tr>
                            <th style="padding:8px 12px;text-align:left;border-bottom:2px solid #ddd;">"级别"</th>
                            <th style="padding:8px 12px;text-align:left;border-bottom:2px solid #ddd;">"消息"</th>
                            <th style="padding:8px 12px;text-align:left;border-bottom:2px solid #ddd;">"时间戳"</th>
                        </tr>
                    </thead>
                    <tbody>
                        {move || {
                            let filter = filter_level.get();
                            let level_order = |l: LogLevel| match l {
                                LogLevel::Error => 0,
                                LogLevel::Warn => 1,
                                LogLevel::Info => 2,
                                LogLevel::Debug => 3,
                                LogLevel::Trace => 4,
                            };
                            let filter_order = level_order(filter);
                            let filtered = logs
                                .get()
                                .into_iter()
                                .filter(|(level, _, _)| level_order(*level) >= filter_order)
                                .collect::<Vec<_>>();

                            filtered.into_iter().map(|(level, msg, ts)| {
                                view! { <LogEntry level={level} message={msg} timestamp={ts} /> }
                            }).collect_view()
                        }}
                    </tbody>
                </table>
            </div>

            {move || (show_json.get()).then(|| {
                view! {
                    <div style="margin-top:16px;">
                        <div style="display:flex;justify-content:space-between;align-items:center;margin-bottom:8px;">
                            <h3>"📄 JSON 结构化输出"</h3>
                            <button on:click=move |_| {
                                let current = logs.get();
                                let json_lines = current.iter().map(|(level, msg, ts)| {
                                    format!(r#"{{"level":"{}","message":"{}","timestamp":"{}"}}"#,
                                        level.label(), msg, ts)
                                }).collect::<Vec<_>>().join("\n");
                                set_json_raw.set(json_lines);
                            } style="background:#333;color:white;border:none;padding:6px 14px;border-radius:4px;cursor:pointer;">
                                "生成 JSON"
                            </button>
                        </div>
                        <JsonLogViewer raw={json_raw.get()} />
                    </div>
                }
            })}

            <div style="margin-top:20px;padding:12px;background:#f0f8ff;border-radius:6px;font-size:13px;color:#333;">
                <strong>"💡 在生产环境中: "</strong>
                "使用 tracing-subscriber 配置 JSON 格式输出，通过日志级别过滤控制输出量，"
                "将日志采集到集中式日志平台（ELK/Loki）进行检索分析。"
            </div>
        </div>
    }
}

fn format_timestamp(secs: u64) -> String {
    let hours = (secs / 3600) % 24;
    let minutes = (secs / 60) % 60;
    let seconds = secs % 60;
    format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
}

fn main() {
    mount_to_body(Exercise);
}
