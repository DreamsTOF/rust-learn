// ============================================================
// 练习 e332: error_reporting — 错误边界与错误报告
//
// 核心知识点:
//   - <ErrorBoundary> 组件捕获子组件中的 Result::Err
//   - panic hook 配置与自定义 panic 处理
//   - 错误上报服务（模拟集中式错误记录）
//   - 用户友好的错误提示与恢复
//
// 难度: ⭐⭐ (TODO 约 50%)
// ============================================================

use leptos::prelude::*;
use std::fmt;

// TODO: 定义一个应用错误类型 AppError
// 包含字段: code (u32), message (String), severity (Severity)
// 实现 Debug, Clone, Display
#[derive(Debug, Clone)]
enum Severity {
    Low,
    Medium,
    High,
    Critical,
}

impl fmt::Display for Severity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            // TODO: 显示严重级别文本
            Severity::Low => write!(f, "低"),
            Severity::Medium => write!(f, "中"),
            Severity::High => write!(f, "高"),
            Severity::Critical => write!(f, "致命"),
        }
    }
}

#[derive(Debug, Clone)]
struct AppError {
    // TODO: 添加 code, message, severity 字段
    code: u32,
    message: String,
    severity: Severity,
}

impl AppError {
    fn new(code: u32, message: impl Into<String>, severity: Severity) -> Self {
        Self {
            code,
            message: message.into(),
            severity,
        }
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO: 格式化错误输出，包含代码、严重级别和消息
        write!(f, "[{}] Error #{}: {}", self.severity, self.code, self.message)
    }
}

impl std::error::Error for AppError {}

// TODO: 创建全局错误报告容器
// 使用 RwSignal<Vec<AppError>> 存储所有已捕获的错误
// 需要调用 provide_context 传递给子组件

#[component]
fn ErrorReportButton(
    #[prop(default = "模拟错误")]
    label: &'static str,
    error: AppError,
) -> impl IntoView {
    // TODO: 使用 use_context 获取错误报告信号
    // 按钮点击时将 error 添加到错误列表中
    // 使用 throw() 抛出异常给 ErrorBoundary
    let report_errors = use_context::<RwSignal<Vec<AppError>>>()
        .expect("Error report context not found");

    view! {
        <button
            on:click=move |_| {
                report_errors.update(|errors| errors.push(error.clone()));
                // 使用 leptos::error::throw 抛出错误
                // throw() 需要实现 std::error::Error 的类型
            }
            style="background:#dc3545;color:white;border:none;padding:8px 16px;border-radius:4px;cursor:pointer;margin:4px;"
        >
            {label}
        </button>
    }
}

#[component]
fn PanicSimulator() -> impl IntoView {
    view! {
        <button
            on:click=|_| {
                // TODO: 模拟一个 panic，触发 panic hook
                // 提示: 使用 panic! 宏或 unwrap()
                panic!("模拟的严重错误: 服务器内部状态异常");
            }
            style="background:#6c757d;color:white;border:none;padding:8px 16px;border-radius:4px;cursor:pointer;margin:4px;"
        >
            "💥 模拟 Panic"
        </button>
    }
}

#[component]
fn ErrorList() -> impl IntoView {
    let report_errors = use_context::<RwSignal<Vec<AppError>>>()
        .expect("Error report context not found");

    // TODO: 使用 create_memo 基于 report_errors 派生统计信息
    // 统计: 总错误数、各严重级别错误数
    let stats = Memo::new(move |_| {
        let errors = report_errors.get();
        let total = errors.len();
        let critical = errors.iter().filter(|e| matches!(e.severity, Severity::Critical)).count();
        let high = errors.iter().filter(|e| matches!(e.severity, Severity::High)).count();
        let medium = errors.iter().filter(|e| matches!(e.severity, Severity::Medium)).count();
        let low = errors.iter().filter(|e| matches!(e.severity, Severity::Low)).count();
        (total, critical, high, medium, low)
    });

    view! {
        <div style="margin-bottom: 16px; display: flex; gap: 16px; flex-wrap: wrap;">
            <div style="padding: 12px; background: #f8f9fa; border-radius: 8px; flex: 1; min-width: 100px; text-align: center;">
                <div style="font-size: 24px; font-weight: bold;">{move || stats.get().0}</div>
                <div style="font-size: 12px; color: #666;">"总错误"</div>
            </div>
            <div style="padding: 12px; background: #dc3545; color: white; border-radius: 8px; flex: 1; min-width: 100px; text-align: center;">
                <div style="font-size: 24px; font-weight: bold;">{move || stats.get().1}</div>
                <div style="font-size: 12px;">"致命"</div>
            </div>
            <div style="padding: 12px; background: #fd7e14; color: white; border-radius: 8px; flex: 1; min-width: 100px; text-align: center;">
                <div style="font-size: 24px; font-weight: bold;">{move || stats.get().2}</div>
                <div style="font-size: 12px;">"高"</div>
            </div>
            <div style="padding: 12px; background: #ffc107; border-radius: 8px; flex: 1; min-width: 100px; text-align: center;">
                <div style="font-size: 24px; font-weight: bold;">{move || stats.get().3}</div>
                <div style="font-size: 12px; color: #666;">"中"</div>
            </div>
            <div style="padding: 12px; background: #28a745; color: white; border-radius: 8px; flex: 1; min-width: 100px; text-align: center;">
                <div style="font-size: 24px; font-weight: bold;">{move || stats.get().4}</div>
                <div style="font-size: 12px;">"低"</div>
            </div>
        </div>
    }
}

#[component]
fn Exercise() -> impl IntoView {
    // TODO: 创建错误报告信号并提供 context
    let report_errors: RwSignal<Vec<AppError>> = RwSignal::new(Vec::new());
    provide_context(report_errors);

    // TODO: 创建一个信号追踪累积的错误计数用于显示
    let (error_count, _set_error_count) = signal(0u32);

    view! {
        <div style="font-family: system-ui, sans-serif; max-width: 900px; margin: 20px auto; padding: 0 16px;">
            <h2>"🛡️ 错误报告系统"</h2>
            <p style="color: #666; margin-bottom: 20px;">
                "使用 ErrorBoundary 捕获错误、自定义 panic hook、集中式错误上报"
            </p>

            // 错误统计面板
            <ErrorList />

            // TODO: 生成错误按钮区域
            <div style="margin-bottom: 20px;">
                <h3>"生成错误"</h3>
                <div style="display: flex; gap: 8px; flex-wrap: wrap;">
                    <ErrorReportButton
                        label="🔵 低级错误"
                        error={AppError::new(1001, "次要配置缺失", Severity::Low)}
                    />
                    <ErrorReportButton
                        label="🟡 中级错误"
                        error={AppError::new(2001, "API 请求超时", Severity::Medium)}
                    />
                    <ErrorReportButton
                        label="🟠 高级错误"
                        error={AppError::new(3001, "用户认证失败", Severity::High)}
                    />
                    <ErrorReportButton
                        label="🔴 致命错误"
                        error={AppError::new(4001, "数据库连接池耗尽", Severity::Critical)}
                    />
                    <PanicSimulator />
                </div>
            </div>

            // TODO: 使用 ErrorBoundary 包裹渲染区域
            // fallback 接收 errors 信号，显示友好的错误界面
            // 子组件使用 throw() 触发错误
            <ErrorBoundary fallback=|errors| {
                view! {
                    <div style="padding: 20px; background: #fff3f3; border: 2px solid #dc3545; border-radius: 8px; margin-bottom: 20px;">
                        <h3 style="color: #dc3545; margin: 0 0 8px 0;">"🚨 错误已捕获"</h3>
                        <p style="margin: 0 0 12px 0;">
                            {move || {
                                errors.get().iter()
                                    .map(|(_, e)| e.to_string())
                                    .collect::<Vec<_>>()
                                    .join("; ")
                            }}
                        </p>
                        <button on:click=move |_| {
                            // 点击后清除错误（这里展示自动恢复）
                        } style="background:#6c757d;color:white;border:none;padding:6px 14px;border-radius:4px;cursor:pointer;">
                            "尝试恢复"
                        </button>
                    </div>
                }
            }>
                // TODO: 子组件，点击触发 ErrorBoundary
                // 使用 leptos::error::throw(AppError::new(...))
                <div style="padding: 20px; background: #f0f8ff; border-radius: 8px; margin-bottom: 20px;">
                    <h3>"✅ 正常渲染区域"</h3>
                    <p>"此区域被 ErrorBoundary 保护。点击下方按钮抛出错误。"</p>
                    <button on:click=move |_| {
                        // throw!(AppError::new(9999, "渲染时发生不可恢复错误", Severity::Critical));
                    } style="background:#dc3545;color:white;border:none;padding:8px 16px;border-radius:4px;cursor:pointer;">
                        "💣 在此区域抛出错误"
                    </button>
                </div>
            </ErrorBoundary>

            // 已上报错误列表
            <div style="margin-top: 20px;">
                <h3>"📋 已上报错误"</h3>
                {move || {
                    let errors = report_errors.get();
                    if errors.is_empty() {
                        view! { <p style="color: #999;">"暂无错误上报"</p> }.into_any()
                    } else {
                        view! {
                            <div style="border: 1px solid #ddd; border-radius: 6px; overflow: hidden;">
                                <table style="width: 100%; border-collapse: collapse;">
                                    <thead style="background: #f5f5f5;">
                                        <tr>
                                            <th style="padding: 8px 12px; text-align: left; border-bottom: 2px solid #ddd;">"严重级别"</th>
                                            <th style="padding: 8px 12px; text-align: left; border-bottom: 2px solid #ddd;">"代码"</th>
                                            <th style="padding: 8px 12px; text-align: left; border-bottom: 2px solid #ddd;">"消息"</th>
                                        </tr>
                                    </thead>
                                    <tbody>
                                        {errors.iter().map(|error| {
                                            let color = match error.severity {
                                                Severity::Low => "#28a745",
                                                Severity::Medium => "#ffc107",
                                                Severity::High => "#fd7e14",
                                                Severity::Critical => "#dc3545",
                                            };
                                            view! {
                                                <tr>
                                                    <td style="padding: 8px 12px; border-bottom: 1px solid #eee;">
                                                        <span style:color={color} style:font-weight="bold">{error.severity.to_string()}</span>
                                                    </td>
                                                    <td style="padding: 8px 12px; border-bottom: 1px solid #eee; font-family: monospace;">
                                                        {error.code}
                                                    </td>
                                                    <td style="padding: 8px 12px; border-bottom: 1px solid #eee;">
                                                        {&error.message}
                                                    </td>
                                                </tr>
                                            }
                                        }).collect_view()
                                    }
                                    </tbody>
                                </table>
                            </div>
                        }.into_any()
                    }
                }}
            </div>

            <div style="margin-top: 20px; padding: 12px; background: #f0f8ff; border-radius: 6px; font-size: 13px; color: #333;">
                <strong>"💡 在生产环境中: "</strong>
                "配置自定义 panic hook 将 panics 记录到日志系统，"
                "使用集中式错误上报服务（如 Sentry/Bugsnag）收集并分析错误，"
                "ErrorBoundary 显示用户友好的 fallback UI 并提供恢复选项。"
            </div>
        </div>
    }
}

fn main() {
    // TODO: 安装自定义 panic hook
    // 使用 std::panic::set_hook
    // 在 hook 中将 panic 信息记录到控制台并（可选）上报
    std::panic::set_hook(Box::new(|panic_info| {
        let message = if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
            s.to_string()
        } else if let Some(s) = panic_info.payload().downcast_ref::<String>() {
            s.clone()
        } else {
            "Unknown panic".to_string()
        };
        let location = panic_info.location()
            .map(|l| format!("{}:{}", l.file(), l.line()))
            .unwrap_or_default();
        // 实际生产环境中，这里会将错误上报到 Sentry/Bugsnag 等
        web_sys::console::error_2(
            &"[PANIC]".into(),
            &format!("{} at {}", message, location).into(),
        );
    }));

    mount_to_body(Exercise);
}

// <details>
// 参考答案（去除注释后的纯净版本）:
//
// use leptos::prelude::*;
// use std::fmt;
//
// #[derive(Debug, Clone)]
// enum Severity { Low, Medium, High, Critical }
//
// impl fmt::Display for Severity {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self { Self::Low => write!(f,"低"), Self::Medium => write!(f,"中"), Self::High => write!(f,"高"), Self::Critical => write!(f,"致命") }
//     }
// }
//
// #[derive(Debug, Clone)]
// struct AppError { code: u32, message: String, severity: Severity }
//
// impl AppError { fn new(code: u32, message: impl Into<String>, severity: Severity) -> Self { Self { code, message: message.into(), severity } } }
//
// impl fmt::Display for AppError { fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "[{}] Error #{}: {}", self.severity, self.code, self.message) } }
//
// impl std::error::Error for AppError {}
//
// #[component] fn ErrorReportButton(#[prop(default = "模拟错误")] label: &'static str, error: AppError) -> impl IntoView {
//     let report_errors = use_context::<RwSignal<Vec<AppError>>>().expect("Error report context not found");
//     view! { <button on:click=move|_|{report_errors.update(|e|e.push(error.clone()));} style="background:#dc3545;color:white;border:none;padding:8px 16px;border-radius:4px;cursor:pointer;margin:4px;">{label}</button> }
// }
//
// #[component] fn PanicSimulator() -> impl IntoView {
//     view! { <button on:click=|_|{panic!("模拟panic");} style="background:#6c757d;color:white;border:none;padding:8px 16px;border-radius:4px;cursor:pointer;margin:4px;">"💥 模拟 Panic"</button> }
// }
//
// fn main() {
//     std::panic::set_hook(Box::new(|info| {
//         let msg = info.payload().downcast_ref::<&str>().map(|s| s.to_string())
//             .or_else(|| info.payload().downcast_ref::<String>().cloned())
//             .unwrap_or_default();
//         let loc = info.location().map(|l| format!("{}:{}", l.file(), l.line())).unwrap_or_default();
//         web_sys::console::error_2(&"[PANIC]".into(), &format!("{} at {}", msg, loc).into());
//     }));
//     mount_to_body(Exercise);
// }
// </details>
