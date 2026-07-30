// ============================================================
// 练习 e332: error_reporting — 参考答案
//
// 核心知识点:
//   - <ErrorBoundary> 组件捕获子组件中的 Result::Err
//   - panic hook 配置与自定义 panic 处理
//   - 错误上报服务（模拟集中式错误记录）
//   - 用户友好的错误提示与恢复
// ============================================================

use leptos::prelude::*;
use std::fmt;

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
            Severity::Low => write!(f, "低"),
            Severity::Medium => write!(f, "中"),
            Severity::High => write!(f, "高"),
            Severity::Critical => write!(f, "致命"),
        }
    }
}

#[derive(Debug, Clone)]
struct AppError {
    code: u32,
    message: String,
    severity: Severity,
}

impl AppError {
    fn new(code: u32, message: impl Into<String>, severity: Severity) -> Self {
        Self { code, message: message.into(), severity }
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] Error #{}: {}", self.severity, self.code, self.message)
    }
}

impl std::error::Error for AppError {}

#[component]
fn ErrorReportButton(
    #[prop(default = "模拟错误")]
    label: &'static str,
    error: AppError,
) -> impl IntoView {
    let report_errors = use_context::<RwSignal<Vec<AppError>>>()
        .expect("Error report context not found");

    view! {
        <button
            on:click=move |_| {
                report_errors.update(|errors| errors.push(error.clone()));
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
            on:click=|_| { panic!("模拟的严重错误: 服务器内部状态异常"); }
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
            <div style="padding:12px;background:#f8f9fa;border-radius:8px;flex:1;min-width:100px;text-align:center;">
                <div style="font-size:24px;font-weight:bold;">{move || stats.get().0}</div>
                <div style="font-size:12px;color:#666;">"总错误"</div>
            </div>
            <div style="padding:12px;background:#dc3545;color:white;border-radius:8px;flex:1;min-width:100px;text-align:center;">
                <div style="font-size:24px;font-weight:bold;">{move || stats.get().1}</div>
                <div style="font-size:12px;">"致命"</div>
            </div>
            <div style="padding:12px;background:#fd7e14;color:white;border-radius:8px;flex:1;min-width:100px;text-align:center;">
                <div style="font-size:24px;font-weight:bold;">{move || stats.get().2}</div>
                <div style="font-size:12px;">"高"</div>
            </div>
            <div style="padding:12px;background:#ffc107;border-radius:8px;flex:1;min-width:100px;text-align:center;">
                <div style="font-size:24px;font-weight:bold;">{move || stats.get().3}</div>
                <div style="font-size:12px;color:#666;">"中"</div>
            </div>
            <div style="padding:12px;background:#28a745;color:white;border-radius:8px;flex:1;min-width:100px;text-align:center;">
                <div style="font-size:24px;font-weight:bold;">{move || stats.get().4}</div>
                <div style="font-size:12px;">"低"</div>
            </div>
        </div>
    }
}

#[component]
fn Exercise() -> impl IntoView {
    let report_errors: RwSignal<Vec<AppError>> = RwSignal::new(Vec::new());
    provide_context(report_errors);

    view! {
        <div style="font-family:system-ui,sans-serif;max-width:900px;margin:20px auto;padding:0 16px;">
            <h2>"🛡️ 错误报告系统"</h2>
            <p style="color:#666;margin-bottom:20px;">
                "使用 ErrorBoundary 捕获错误、自定义 panic hook、集中式错误上报"
            </p>

            <ErrorList />

            <div style="margin-bottom:20px;">
                <h3>"生成错误"</h3>
                <div style="display:flex;gap:8px;flex-wrap:wrap;">
                    <ErrorReportButton label="🔵 低级错误" error={AppError::new(1001, "次要配置缺失", Severity::Low)} />
                    <ErrorReportButton label="🟡 中级错误" error={AppError::new(2001, "API 请求超时", Severity::Medium)} />
                    <ErrorReportButton label="🟠 高级错误" error={AppError::new(3001, "用户认证失败", Severity::High)} />
                    <ErrorReportButton label="🔴 致命错误" error={AppError::new(4001, "数据库连接池耗尽", Severity::Critical)} />
                    <PanicSimulator />
                </div>
            </div>

            <ErrorBoundary fallback=|errors| {
                view! {
                    <div style="padding:20px;background:#fff3f3;border:2px solid #dc3545;border-radius:8px;margin-bottom:20px;">
                        <h3 style="color:#dc3545;margin:0 0 8px 0;">"🚨 错误已捕获"</h3>
                        <p style="margin:0 0 12px 0;">
                            {move || errors.get().iter().map(|(_, e)| e.to_string()).collect::<Vec<_>>().join("; ")}
                        </p>
                        <button on:click=|_|{}
                            style="background:#6c757d;color:white;border:none;padding:6px 14px;border-radius:4px;cursor:pointer;">
                            "尝试恢复"
                        </button>
                    </div>
                }
            }>
                <div style="padding:20px;background:#f0f8ff;border-radius:8px;margin-bottom:20px;">
                    <h3>"✅ 正常渲染区域"</h3>
                    <p>"此区域被 ErrorBoundary 保护。点击下方按钮抛出错误。"</p>
                    <button on:click=move |_| {
                        leptos::error::throw(AppError::new(9999, "渲染时发生不可恢复错误", Severity::Critical));
                    }
                        style="background:#dc3545;color:white;border:none;padding:8px 16px;border-radius:4px;cursor:pointer;">
                        "💣 在此区域抛出错误"
                    </button>
                </div>
            </ErrorBoundary>

            <div style="margin-top:20px;">
                <h3>"📋 已上报错误"</h3>
                {move || {
                    let errors = report_errors.get();
                    if errors.is_empty() {
                        view! { <p style="color:#999;">"暂无错误上报"</p> }.into_any()
                    } else {
                        view! {
                            <div style="border:1px solid #ddd;border-radius:6px;overflow:hidden;">
                                <table style="width:100%;border-collapse:collapse;">
                                    <thead style="background:#f5f5f5;">
                                        <tr>
                                            <th style="padding:8px 12px;text-align:left;border-bottom:2px solid #ddd;">"严重级别"</th>
                                            <th style="padding:8px 12px;text-align:left;border-bottom:2px solid #ddd;">"代码"</th>
                                            <th style="padding:8px 12px;text-align:left;border-bottom:2px solid #ddd;">"消息"</th>
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
                                                    <td style="padding:8px 12px;border-bottom:1px solid #eee;">
                                                        <span style:color={color};font-weight="bold">{error.severity.to_string()}</span>
                                                    </td>
                                                    <td style="padding:8px 12px;border-bottom:1px solid #eee;font-family:monospace;">
                                                        {error.code}
                                                    </td>
                                                    <td style="padding:8px 12px;border-bottom:1px solid #eee;">
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

            <div style="margin-top:20px;padding:12px;background:#f0f8ff;border-radius:6px;font-size:13px;color:#333;">
                <strong>"💡 在生产环境中: "</strong>
                "配置自定义 panic hook 将 panics 记录到日志系统，"
                "使用集中式错误上报服务（如 Sentry/Bugsnag）收集并分析错误，"
                "ErrorBoundary 显示用户友好的 fallback UI 并提供恢复选项。"
            </div>
        </div>
    }
}

fn main() {
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
        web_sys::console::error_2(
            &"[PANIC]".into(),
            &format!("{} at {}", message, location).into(),
        );
    }));

    mount_to_body(Exercise);
}
