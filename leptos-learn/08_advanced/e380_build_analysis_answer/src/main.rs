// ============================================================
// 参考答案 e380: 构建分析工具
//
// 展示依赖树、编译时间统计、版本兼容性分析
// ============================================================

use leptos::prelude::*;

/// 依赖项信息
#[allow(dead_code)]
struct Dependency {
    name: &'static str,
    version: &'static str,
    depth: u32,
    is_transitive: bool,
    size_kb: f64,
    compat_score: u8, // 0-100
}

/// 编译阶段耗时 (秒)
struct BuildPhase {
    name: &'static str,
    duration_secs: f64,
}

fn get_dependency_tree() -> Vec<Dependency> {
    vec![
        Dependency { name: "leptos", version: "0.8.0-nightly", depth: 0, is_transitive: false, size_kb: 120.0, compat_score: 100 },
        Dependency { name: "  ├─ leptos_macro", version: "0.8.0-nightly", depth: 1, is_transitive: true, size_kb: 25.0, compat_score: 100 },
        Dependency { name: "  ├─ leptos_dom", version: "0.8.0-nightly", depth: 1, is_transitive: true, size_kb: 85.0, compat_score: 100 },
        Dependency { name: "  │   ├─ web-sys", version: "0.3.103", depth: 2, is_transitive: true, size_kb: 200.0, compat_score: 90 },
        Dependency { name: "  │   └─ js-sys", version: "0.2.7", depth: 2, is_transitive: true, size_kb: 50.0, compat_score: 70 },
        Dependency { name: "  ├─ serde", version: "1.0.217", depth: 1, is_transitive: true, size_kb: 40.0, compat_score: 95 },
        Dependency { name: "  │   └─ serde_derive", version: "1.0.217", depth: 2, is_transitive: true, size_kb: 25.0, compat_score: 95 },
        Dependency { name: "  ├─ wasm-bindgen", version: "0.2.126", depth: 1, is_transitive: true, size_kb: 50.0, compat_score: 85 },
        Dependency { name: "  └─ futures", version: "0.3.31", depth: 1, is_transitive: true, size_kb: 60.0, compat_score: 90 },
    ]
}

fn get_build_phases() -> Vec<BuildPhase> {
    vec![
        BuildPhase { name: "依赖下载与解析", duration_secs: 8.5 },
        BuildPhase { name: "编译 leptos 核心", duration_secs: 45.2 },
        BuildPhase { name: "编译 web-sys", duration_secs: 62.0 },
        BuildPhase { name: "编译 wasm-bindgen", duration_secs: 18.3 },
        BuildPhase { name: "编译 serde", duration_secs: 12.7 },
        BuildPhase { name: "编译用户代码", duration_secs: 5.1 },
        BuildPhase { name: "LTO 链接优化", duration_secs: 30.5 },
        BuildPhase { name: "WASM 生成与压缩", duration_secs: 4.2 },
    ]
}

/// 将秒数格式化为可读字符串
fn format_duration(secs: f64) -> String {
    if secs >= 60.0 {
        format!("{:.1} 分 {:.0} 秒", secs / 60.0, secs % 60.0)
    } else {
        format!("{:.1} 秒", secs)
    }
}


#[component]
fn Exercise() -> impl IntoView {
    let deps = get_dependency_tree();
    let phases = get_build_phases();
    let total_time: f64 = phases.iter().map(|p| p.duration_secs).sum();

    view! {
        <div style="padding: 20px; max-width: 700px; margin: 0 auto; font-family: system-ui, sans-serif;">
            <h2>"🔨 构建分析工具"</h2>
            <p style="color: #666;">
                "以下展示 Cargo 依赖树、各阶段编译时间统计及版本兼容性分析。"
            </p>

            {/* 依赖树 */}
            <div style="border: 1px solid #ddd; border-radius: 8px; padding: 16px; margin: 16px 0;">
                <h3 style="margin: 0 0 12px 0;">"🌳 依赖树"</h3>
                <pre style="font-size: 13px; line-height: 1.6; background: #fafafa; padding: 12px;
                           border-radius: 4px; overflow-x: auto;">
                    {deps.into_iter().map(|d| {
                        format!("{} {}  [{} KB, compat: {}]",
                            d.name, d.version, format!("{:.0}", d.size_kb), d.compat_score)
                    }).collect::<Vec<_>>().join("\n")}
                </pre>
            </div>

            {/* 编译时间统计 */}
            <div style="border: 1px solid #ddd; border-radius: 8px; padding: 16px; margin: 16px 0;">
                <h3 style="margin: 0 0 12px 0;">"⏱️ 编译时间统计"</h3>
                <div style="display: flex; justify-content: space-between; align-items: center;
                            margin-bottom: 12px;">
                    <span style="color: #666;">"总耗时:"</span>
                    <span style="font-size: 20px; font-weight: bold;">{format_duration(total_time)}</span>
                </div>
                {phases.into_iter().map(|p| {
                    let pct = p.duration_secs / total_time * 100.0;
                    let bar_width = format!("{}%", pct);
                    view! {
                        <div style="margin: 6px 0;">
                            <div style="display: flex; justify-content: space-between; font-size: 13px;">
                                <span>{p.name}</span>
                                <span style="font-weight: bold;">{format_duration(p.duration_secs)}</span>
                            </div>
                            <div style="height: 6px; background: #e0e0e0; border-radius: 3px; margin-top: 2px;">
                                <div style=move || format!("height: 6px; width: {}; background: #1976d2; border-radius: 3px;", bar_width)></div>
                            </div>
                        </div>
                    }
                }).collect::<Vec<_>>()}
            </div>

            {/* 版本兼容性 */}
            <div style="border: 1px solid #ddd; border-radius: 8px; padding: 16px; margin: 16px 0;">
                <h3 style="margin: 0 0 12px 0;">"🔗 版本兼容性分析"</h3>
                <div style="font-size: 14px; line-height: 1.8;">
                    <p>
                        <span style="color: #2e7d32;">"✅"</span>
                        " leptos 0.8.0-nightly — 100% 兼容（项目主框架）"
                    </p>
                    <p>
                        <span style="color: #e65100;">"⚠️"</span>
                        " js-sys 0.2.7 — 70% 兼容（与当前 Rust nightly 1.97 存在 FnMut extern 兼容性问题）"
                    </p>
                    <p>
                        <span style="color: #2e7d32;">"✅"</span>
                        " serde 1.0.217 — 95% 兼容（建议锁定版本）"
                    </p>
                    <p>
                        <span style="color: #2e7d32;">"✅"</span>
                        " wasm-bindgen 0.2.126 — 85% 兼容（需跟踪更新）"
                    </p>
                    <p>
                        <span style="color: #2e7d32;">"✅"</span>
                        " futures 0.3.31 — 90% 兼容"
                    </p>
                </div>
            </div>

            {/* 优化建议 */}
            <div style="border: 1px solid #1976d2; border-radius: 8px; padding: 16px; background: #e3f2fd;">
                <h3 style="margin: 0 0 8px 0; color: #1565c0;">"💡 构建优化建议"</h3>
                <ul style="line-height: 1.8; margin: 0; padding-left: 20px;">
                    <li>"使用 cargo check --timings 分析详细编译瓶颈。"</li>
                    <li>"启用 sccache 缓存编译产物，减少增量编译时间。"</li>
                    <li>"将大型依赖（如 web-sys）移至单独的 workspace crate。"</li>
                    <li>"在 CI 中缓存 target 目录以加速重复构建。"</li>
                    <li>"使用 cargo update 定期更新依赖以获取性能改进。"</li>
                </ul>
            </div>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
