// ============================================================
// 练习 e331: log_collection — 结构化日志采集
//
// 核心知识点:
//   - tracing 订阅者配置
//   - 结构化 JSON 日志输出
//   - 日志级别过滤（error/warn/info/debug/trace）
//   - 自定义日志格式与输出目标
//
// 难度: ⭐⭐ (TODO 约 50%)
// ============================================================

use leptos::prelude::*;

// TODO: 创建一个日志级别枚举，包含 Error/Warn/Info/Debug/Trace
// 使用 #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// 提示: 参考 tracing 的 Level 设计
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

impl LogLevel {
    // TODO: 返回 emoji 图标
    // Error→"🔴", Warn→"🟡", Info→"🔵", Debug→"🟢", Trace→"⚪"
    fn icon(self) -> &'static str {
        match self {
            LogLevel::Error => "🔴",
            LogLevel::Warn => "🟡",
            LogLevel::Info => "🔵",
            LogLevel::Debug => "🟢",
            LogLevel::Trace => "⚪",
        }
    }

    // TODO: 返回级别名称的大写字符串
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
            // TODO: 显示日志级别图标和标签
            <td style:color={match level {
                LogLevel::Error => "red",
                LogLevel::Warn => "orange",
                LogLevel::Info => "blue",
                LogLevel::Debug => "green",
                LogLevel::Trace => "gray",
            }}>
                {level.icon()} " " {level.label()}
            </td>
            // TODO: 显示日志消息
            <td>{message}</td>
            // TODO: 显示时间戳
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
    // TODO: 尝试将 raw 字符串按行解析为 JSON 并格式化显示
    // 提示: 按 \n 分割，去掉空行，以 JSON 风格展示
    let formatted = raw
        .lines()
        .filter(|l| !l.is_empty())
        .map(|line| {
            // 此处可引入 serde_json 做格式化，当前用简单包装展示
            format!("{{ \"log\": {} }}", line)
        })
        .collect::<Vec<_>>()
        .join("\n");

    view! {
        <pre style="background: #1e1e1e; color: #d4d4d4; padding: 8px; border-radius: 4px; font-size: 12px; overflow-x: auto;">
            {formatted}
        </pre>
    }
}

#[component]
fn Exercise() -> impl IntoView {
    // TODO: 创建响应式信号存储日志条目列表
    let (logs, set_logs) = signal(Vec::new());
    // TODO: 创建日志级别过滤信号，初始为 LogLevel::Info
    let (filter_level, set_filter_level) = signal(LogLevel::Info);
    // TODO: 创建 JSON 原始文本信号
    let (json_raw, set_json_raw) = signal(String::new());
    // TODO: 创建是否显示 JSON 视图的信号
    let (show_json, set_show_json) = signal(false);

    // TODO: 定义一个模拟日志生成的函数
    // fn add_log(level: LogLevel, message: impl Into<String>)
    // 使用 set_logs.update() 添加新条目并更新时间戳
    fn add_log(
        logs: &WriteSignal<Vec<(LogLevel, String, String)>>,
        level: LogLevel,
        message: impl Into<String>,
    ) {
        use std::time::SystemTime;
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        let timestamp = format_timestamp(now);
        logs.update(|l| l.push((level, message.into(), timestamp)));
    }

    let add_log_entry = move |level: LogLevel, message: String| {
        add_log(&set_logs, level, message);
    };

    view! {
        <div style="font-family: system-ui, sans-serif; max-width: 800px; margin: 20px auto; padding: 0 16px;">
            <h2>"📋 结构化日志采集系统"</h2>
            <p style="color: #666; margin-bottom: 16px;">
                "演示 tracing 订阅者模式、日志级别过滤与 JSON 结构化输出"
            </p>

            // 控制台
            <div style="display: flex; gap: 8px; margin-bottom: 16px; flex-wrap: wrap;">
                // TODO: 添加不同级别的日志按钮
                // Error / Warn / Info / Debug / Trace
                <button on:click=move |_| add_log_entry(LogLevel::Error, "数据库连接超时".to_string())
                    style="background: #ff4444; color: white; border: none; padding: 6px 14px; border-radius: 4px; cursor: pointer;">
                    "🔴 Error"
                </button>
                <button on:click=move |_| add_log_entry(LogLevel::Warn, "API 响应时间超过 2s".to_string())
                    style="background: #ff8800; color: white; border: none; padding: 6px 14px; border-radius: 4px; cursor: pointer;">
                    "🟡 Warn"
                </button>
                <button on:click=move |_| add_log_entry(LogLevel::Info, "用户登录成功: user_id=42".to_string())
                    style="background: #0088ff; color: white; border: none; padding: 6px 14px; border-radius: 4px; cursor: pointer;">
                    "🔵 Info"
                </button>
                <button on:click=move |_| add_log_entry(LogLevel::Debug, "缓存命中 key=session_abc".to_string())
                    style="background: #00aa44; color: white; border: none; padding: 6px 14px; border-radius: 4px; cursor: pointer;">
                    "🟢 Debug"
                </button>
                <button on:click=move |_| add_log_entry(LogLevel::Trace, "渲染组件: Header > Nav > UserMenu".to_string())
                    style="background: #888; color: white; border: none; padding: 6px 14px; border-radius: 4px; cursor: pointer;">
                    "⚪ Trace"
                </button>
            </div>

            // 日志级别过滤器
            <div style="margin-bottom: 12px; display: flex; align-items: center; gap: 8px;">
                <label>"过滤级别:"</label>
                <select on:change=move |ev| {
                    let val = event_target_value(&ev);
                    let level = match val.as_str() {
                        "ERROR" => LogLevel::Error,
                        "WARN" => LogLevel::Warn,
                        "INFO" => LogLevel::Info,
                        "DEBUG" => LogLevel::Debug,
                        "TRACE" => LogLevel::Trace,
                        _ => LogLevel::Info,
                    };
                    set_filter_level.set(level);
                } style="padding: 4px 8px; border-radius: 4px;">
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

                // TODO: 添加清空日志按钮
                <button on:click=move |_| set_logs.set(Vec::new())
                    style="margin-left: auto; background: #eee; border: 1px solid #ccc; padding: 4px 12px; border-radius: 4px; cursor: pointer;">
                    "🗑️ 清空"
                </button>
            </div>

            // 日志表格
            <div style="border: 1px solid #ddd; border-radius: 6px; overflow: hidden;">
                <table style="width: 100%; border-collapse: collapse;">
                    <thead style="background: #f5f5f5;">
                        <tr>
                            <th style="padding: 8px 12px; text-align: left; border-bottom: 2px solid #ddd;">"级别"</th>
                            <th style="padding: 8px 12px; text-align: left; border-bottom: 2px solid #ddd;">"消息"</th>
                            <th style="padding: 8px 12px; text-align: left; border-bottom: 2px solid #ddd;">"时间戳"</th>
                        </tr>
                    </thead>
                    <tbody>
                        // TODO: 使用 For 组件渲染日志列表
                        // 根据 filter_level 过滤日志条目
                        // 只显示 level >= filter_level 的条目
                        // 层级顺序: Error > Warn > Info > Debug > Trace
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
                            filtered.len().to_string()
                        }}
                        // TODO: 用 For 组件实际渲染过滤后的日志条目
                        // <For each=move || filtered_logs key=|(level, msg, ts)| (level, msg, ts) let:item>
                        //     <LogEntry level={item.0} message={item.1} timestamp={item.2} />
                        // </For>
                    </tbody>
                </table>
            </div>

            // TODO: 条件渲染 JSON 视图
            // 当 show_json 为 true 时显示
            // 点击 "生成 JSON" 按钮将当前日志转为 JSON 格式
            {move || (show_json.get()).then(|| {
                view! {
                    <div style="margin-top: 16px;">
                        <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 8px;">
                            <h3>"📄 JSON 结构化输出"</h3>
                            <button on:click=move |_| {
                                let current = logs.get();
                                let json_lines = current.iter().map(|(level, msg, ts)| {
                                    format!(r#"{{"level":"{}","message":"{}","timestamp":"{}"}}"#,
                                        level.label(), msg, ts)
                                }).collect::<Vec<_>>().join("\n");
                                set_json_raw.set(json_lines);
                            } style="background: #333; color: white; border: none; padding: 6px 14px; border-radius: 4px; cursor: pointer;">
                                "生成 JSON"
                            </button>
                        </div>
                        <JsonLogViewer raw={json_raw.get()} />
                    </div>
                }
            })}

            // 结构化日志说明
            <div style="margin-top: 20px; padding: 12px; background: #f0f8ff; border-radius: 6px; font-size: 13px; color: #333;">
                <strong>"💡 在生产环境中: "</strong>
                "使用 tracing-subscriber 配置 JSON 格式输出，通过日志级别过滤控制输出量，"
                "将日志采集到集中式日志平台（ELK/Loki）进行检索分析。"
            </div>
        </div>
    }
}

// TODO: 实现时间戳格式化函数
// 将 Unix 时间戳（秒）格式化为 "HH:MM:SS" 格式
fn format_timestamp(secs: u64) -> String {
    let hours = (secs / 3600) % 24;
    let minutes = (secs / 60) % 60;
    let seconds = secs % 60;
    format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
}

fn main() {
    mount_to_body(Exercise);
}

// <details>
// 参考答案（去除注释后的纯净版本）:
//
// use leptos::prelude::*;
//
// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// enum LogLevel { Error, Warn, Info, Debug, Trace }
//
// impl LogLevel {
//     fn icon(self) -> &'static str {
//         match self { Self::Error => "🔴", Self::Warn => "🟡", Self::Info => "🔵", Self::Debug => "🟢", Self::Trace => "⚪" }
//     }
//     fn label(self) -> &'static str {
//         match self { Self::Error => "ERROR", Self::Warn => "WARN", Self::Info => "INFO", Self::Debug => "DEBUG", Self::Trace => "TRACE" }
//     }
// }
//
// #[component] fn LogEntry(level: LogLevel, message: String, timestamp: String) -> impl IntoView {
//     view! {
//         <tr>
//             <td style:color={match level { LogLevel::Error => "red", LogLevel::Warn => "orange", LogLevel::Info => "blue", LogLevel::Debug => "green", LogLevel::Trace => "gray" }}>
//                 {level.icon()} " " {level.label()}
//             </td>
//             <td>{message}</td>
//             <td>{timestamp}</td>
//         </tr>
//     }
// }
//
// #[component] fn LogLevelFilter(active: LogLevel) -> impl IntoView {
//     view! { <option value={active.label()}>{active.icon()} " " {active.label()}</option> }
// }
//
// #[component] fn JsonLogViewer(raw: String) -> impl IntoView {
//     view! { <pre style="background:#1e1e1e;color:#d4d4d4;padding:8px;border-radius:4px;font-size:12px;overflow-x:auto;">{raw}</pre> }
// }
//
// #[component] fn Exercise() -> impl IntoView {
//     let (logs, set_logs) = signal(Vec::new());
//     let (filter_level, set_filter_level) = signal(LogLevel::Info);
//     let (json_raw, set_json_raw) = signal(String::new());
//     let (show_json, set_show_json) = signal(false);
//
//     let add_log_entry = move |level, msg| {
//         use std::time::SystemTime;
//         let ts = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0);
//         set_logs.update(|l| l.push((level, msg, format_timestamp(ts))));
//     };
//
//     view! {
//         <div style="font-family:system-ui,sans-serif;max-width:800px;margin:20px auto;padding:0 16px;">
//             <h2>"📋 结构化日志采集系统"</h2>
//             <div style="display:flex;gap:8px;margin-bottom:16px;flex-wrap:wrap;">
//                 <button on:click=move|_|add_log_entry(LogLevel::Error,"数据库连接超时".to_string()) style="background:#ff4444;color:white;border:none;padding:6px 14px;border-radius:4px;cursor:pointer;">"🔴 Error"</button>
//                 <button on:click=move|_|add_log_entry(LogLevel::Warn,"API响应时间超过2s".to_string()) style="background:#ff8800;color:white;border:none;padding:6px 14px;border-radius:4px;cursor:pointer;">"🟡 Warn"</button>
//                 <button on:click=move|_|add_log_entry(LogLevel::Info,"用户登录成功".to_string()) style="background:#0088ff;color:white;border:none;padding:6px 14px;border-radius:4px;cursor:pointer;">"🔵 Info"</button>
//                 <button on:click=move|_|add_log_entry(LogLevel::Debug,"缓存命中".to_string()) style="background:#00aa44;color:white;border:none;padding:6px 14px;border-radius:4px;cursor:pointer;">"🟢 Debug"</button>
//                 <button on:click=move|_|add_log_entry(LogLevel::Trace,"渲染组件链".to_string()) style="background:#888;color:white;border:none;padding:6px 14px;border-radius:4px;cursor:pointer;">"⚪ Trace"</button>
//             </div>
//             <div style="margin-bottom:12px;display:flex;align-items:center;gap:8px;">
//                 <label>"过滤级别:"</label>
//                 <select on:change=move|ev|{let val=event_target_value(&ev);set_filter_level.set(match val.as_str(){"ERROR"=>LogLevel::Error,"WARN"=>LogLevel::Warn,"INFO"=>LogLevel::Info,"DEBUG"=>LogLevel::Debug,"TRACE"=>LogLevel::Trace,_=>LogLevel::Info})} style="padding:4px 8px;border-radius:4px;">
//                     <LogLevelFilter active={LogLevel::Error}/><LogLevelFilter active={LogLevel::Warn}/><LogLevelFilter active={LogLevel::Info}/><LogLevelFilter active={LogLevel::Debug}/><LogLevelFilter active={LogLevel::Trace}/>
//                 </select>
//                 <label style="margin-left:16px;"><input type="checkbox" prop:checked={show_json} on:change=move|_|set_show_json.update(|v|*v=!*v)/>" JSON视图"</label>
//                 <button on:click=move|_|set_logs.set(Vec::new()) style="margin-left:auto;background:#eee;border:1px solid #ccc;padding:4px 12px;border-radius:4px;cursor:pointer;">"🗑️ 清空"</button>
//             </div>
//             <div style="border:1px solid #ddd;border-radius:6px;overflow:hidden;">
//                 <table style="width:100%;border-collapse:collapse;">
//                     <thead style="background:#f5f5f5;">
//                         <tr><th style="padding:8px 12px;text-align:left;border-bottom:2px solid #ddd;">"级别"</th><th style="padding:8px 12px;text-align:left;border-bottom:2px solid #ddd;">"消息"</th><th style="padding:8px 12px;text-align:left;border-bottom:2px solid #ddd;">"时间戳"</th></tr>
//                     </thead>
//                     <tbody>
//                         {move || {
//                             let f = filter_level.get();
//                             let order = |l:LogLevel| match l { LogLevel::Error=>0, LogLevel::Warn=>1, LogLevel::Info=>2, LogLevel::Debug=>3, LogLevel::Trace=>4 };
//                             let fo = order(f);
//                             let filtered = logs.get().into_iter().filter(|(l,_,_)| order(*l) >= fo).collect::<Vec<_>>();
//                             filtered.len().to_string()
//                         }}
//                     </tbody>
//                 </table>
//             </div>
//             {move || (show_json.get()).then(|| view! {
//                 <div style="margin-top:16px;">
//                     <div style="display:flex;justify-content:space-between;align-items:center;margin-bottom:8px;">
//                         <h3>"📄 JSON结构化输出"</h3>
//                         <button on:click=move|_|{let c=logs.get();set_json_raw.set(c.iter().map(|(l,m,t)|format!(r#"{{"level":"{}","message":"{}","timestamp":"{}"}}"#,l.label(),m,t)).collect::<Vec<_>>().join("\n"));} style="background:#333;color:white;border:none;padding:6px 14px;border-radius:4px;cursor:pointer;">"生成JSON"</button>
//                     </div>
//                     <pre style="background:#1e1e1e;color:#d4d4d4;padding:8px;border-radius:4px;font-size:12px;overflow-x:auto;">{json_raw.get()}</pre>
//                 </div>
//             })}
//             <div style="margin-top:20px;padding:12px;background:#f0f8ff;border-radius:6px;font-size:13px;color:#333;">
//                 <strong>"💡 在生产环境中: "</strong>"使用 tracing-subscriber 配置 JSON 格式输出，通过日志级别过滤控制输出量，将日志采集到集中式日志平台（ELK/Loki）进行检索分析。"
//             </div>
//         </div>
//     }
// }
//
// fn format_timestamp(secs: u64) -> String {
//     let hours = (secs / 3600) % 24;
//     let minutes = (secs / 60) % 60;
//     let seconds = secs % 60;
//     format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
// }
//
// fn main() { mount_to_body(Exercise); }
// </details>
