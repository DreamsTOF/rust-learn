// ============================================================
// 练习 e344: Error Monitoring — 错误监控与上报
//
// 核心知识点:
//   - ErrorBoundary: 捕获子组件错误
//   - panic_hook: 全局未捕获异常处理
//   - 错误上报: 将错误发送到 Sentry 等平台
//
// 难度: ⭐⭐ (关键 TODOs)
// ============================================================

use leptos::prelude::*;

// TODO: 配置 Sentry DSN
// ⭐⭐ 从环境变量读取，提供默认值
const SENTRY_DSN: &str = "https://your-dsn@sentry.io/1234567";

// TODO: 初始化 panic hook
// ⭐⭐ 在 main 函数中调用 std::panic::set_hook
// 捕获 panic 信息并通过 sentry 上报
fn init_panic_hook() {
    // ⭐⭐ TODO: 使用 std::panic::set_hook 设置全局 panic handler
    // 提示: 获取 panic 信息 (panic_info)
    //       提取 payload 和 location
    //       调用 sentry::capture_event 上报
    //
    // std::panic::set_hook(Box::new(|panic_info| {
    //     let payload = panic_info
    //         .payload()
    //         .downcast_ref::<String>()
    //         .map(|s| s.clone())
    //         .or_else(|| {
    //             panic_info.payload().downcast_ref::<&str>().map(|s| s.to_string())
    //         })
    //         .unwrap_or_default();
    //
    //     let location = panic_info
    //         .location()
    //         .map(|l| format!("{}:{}", l.file(), l.line()))
    //         .unwrap_or_default();
    //
    //     tracing::error!("Panic at {}: {}", location, payload);
    //
    //     // 上报到 Sentry
    //     sentry::capture_event(sentry::protocol::Event {
    //         message: Some(format!("Panic: {}", payload)),
    //         level: sentry::Level::Fatal,
    //         ..Default::default()
    //     });
    // }));
    _ = SENTRY_DSN;
}

// TODO: 实现错误上报函数
// ⭐⭐ 接收错误信息，上报到 Sentry
// 包含: 错误消息、文件位置、用户上下文
fn report_error(error_message: &str, file: &str, line: u32) {
    // ⭐⭐ TODO: 实现错误上报
    // 提示:
    // sentry::configure_scope(|scope| {
    //     scope.set_tag("source", "leptos_app");
    //     scope.set_extra("file", file.to_string().into());
    //     scope.set_extra("line", line.into());
    // });
    //
    // sentry::capture_message(error_message, sentry::Level::Error);
    tracing::error!("[ErrorReport] {} ({}:{})", error_message, file, line);
}

// TODO: 创建一个会抛出错误的组件
// ⭐⭐ 接收 should_error prop，控制是否抛出错误
#[component]
fn BuggyComponent(should_error: bool) -> impl IntoView {
    if should_error {
        // ⭐⭐ TODO: 使用 throw() 抛出 ServerFnError 或任何错误
        // 提示: throw() 来自 leptos::prelude
    }

    view! {
        <p>"一切正常！"</p>
    }
}

#[component]
fn Exercise() -> impl IntoView {
    let (should_error, set_should_error) = signal(false);

    const SENTRY_CODE: &str = "\
// Sentry 初始化
use sentry;

fn main() {
    let _guard = sentry::init((\"https://key@sentry.io/project\", sentry::ClientOptions {
        release: sentry::release_name!(),
        environment: Some(\"production\".into()),
        ..Default::default()
    }));

    init_panic_hook();
    // ...
}

// 全局 panic hook
fn init_panic_hook() {
    std::panic::set_hook(Box::new(|panic_info| {
        sentry::capture_event(sentry::protocol::Event {
            message: Some(format!(\"{}\", panic_info)),
            level: sentry::Level::Fatal,
            ..Default::default()
        });
    }));
}";

    view! {
        <div>
            <h1>"Error Monitoring — 错误监控"</h1>

            <section>
                <h2>"Sentry 集成"</h2>
                <pre>{SENTRY_CODE}</pre>
            </section>

            <section>
                <h2>"Error Boundary 示例"</h2>
                <button on:click=move |_| set_should_error.set(true)>
                    "触发错误"
                </button>

                <div>
                    <h3>"外层组件（安全区域）"</h3>
                    <ErrorBoundary fallback=|errors| view! {
                        <div>
                            <p>"捕获到错误!"</p>
                            <ul>
                                {move || errors.get()
                                    .into_iter()
                                    .map(|e| view! { <li>{e.to_string()}</li> })
                                    .collect::<Vec<_>>()
                                }
                            </ul>
                            <button on:click=move |_| set_should_error.set(false)>
                                "重置"
                            </button>
                        </div>
                    }>
                        <BuggyComponent should_error=should_error/>
                    </ErrorBoundary>
                </div>
            </section>

            <section>
                <h2>"错误上报流程"</h2>
                <ol>
                    <li>"ErrorBoundary 捕获子组件错误"</li>
                    <li>"report_error() 格式化错误信息"</li>
                    <li>"sentry::capture_event/capture_message 上报"</li>
                    <li>"Sentry 控制台聚合、告警、分析"</li>
                    <li>"开发者在 Sentry 查看堆栈和上下文"</li>
                </ol>
            </section>
        </div>
    }
}

fn main() {
    init_panic_hook();
    mount_to_body(Exercise);
}
