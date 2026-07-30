use leptos::prelude::*;

// ============================================================
// 练习 319: 水合陷阱 (Hydration Pitfalls) — 参考答案
//
// 核心: SSR 与 CSR HTML 不一致 → 水合错误 → 延迟到客户端再执行
// ============================================================

/// ⚠️ 会导致水合错误的组件
/// SSR 生成一个随机值，CSR 水合生成另一个 → 不匹配
#[component]
fn WrongRandom() -> impl IntoView {
    // 在 SSR 和 CSR 中调用 rand 会产生不同值
    let random_value: u16 = rand::random();
    view! {
        <p>"随机数: " {random_value}</p>
    }
    // 水合时 Leptos 会发现服务端输出与客户端首次渲染不匹配
    // 控制台会显示水合不匹配警告
}

/// ✅ 修复后的组件
/// 使用信号将随机生成延迟到客户端首次渲染
#[component]
fn CorrectRandom() -> impl IntoView {
    let (random_value, set_random) = signal(None::<u16>);

    Effect::new(move |_| {
        // 只在客户端运行（Effect 在 hydrate 后执行）
        set_random.set(Some(rand::random()));
    });

    view! {
        <p>
            "随机数: "
            {move || random_value.map(|v| format!("{}", v)).unwrap_or_else(|| "加载中...".to_string())}
        </p>
    }
}

/// 另一种修复方式：使用 client-only 渲染
/// 浏览器特有 API 如 window.innerWidth 同理
#[component]
fn BrowserWidth() -> impl IntoView {
    let (width, set_width) = signal(None::<f64>);

    Effect::new(move |_| {
        if let Some(win) = leptos::web_sys::window() {
            let w = win.inner_width().ok().and_then(|v| v.as_f64());
            set_width.set(w);
        }
    });

    view! {
        <p>"浏览器宽度: " {move || width.map(|w| format!("{}px", w)).unwrap_or_else(|| "未知".to_string())}</p>
    }
}

#[component]
fn Exercise() -> impl IntoView {
    view! {
        <div>
            <h1>"练习 319: 水合陷阱"</h1>
            // 取消注释可体验水合错误（浏览器控制台会显示警告）：
            // <WrongRandom/>

            <CorrectRandom/>
            <BrowserWidth/>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}

// ponytail: 触发水合错误的条件：
//   - SSR 渲染值 A，CSR 水合时渲染值 B
//   - 修复方案：将浏览器特有/随机逻辑放入 Effect::new
//   延迟到水合完成后再执行
