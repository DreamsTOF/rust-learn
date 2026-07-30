// ============================================================
// 练习 e376: 功能标志系统 — 运行时开关控制功能可见性
//
// 核心知识点:
//   - RwSignal<HashSet<FeatureFlag>> 管理多个运行时功能开关
//   - 条件渲染控制功能可见性
//   - 管理面板统一调控的设计模式
//
// 难度: ⭐⭐ (需补全 is_enabled、toggle 逻辑及新增标志，约 50%)
// ============================================================

use leptos::prelude::*;
use std::collections::HashSet;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum FeatureFlag {
    NewUI,
    DarkMode,
    BetaFeature,
    // TODO 1: 添加一个新功能标志，例如 AnalyticsV2
}

impl FeatureFlag {
    fn all() -> Vec<FeatureFlag> {
        vec![
            FeatureFlag::NewUI,
            FeatureFlag::DarkMode,
            FeatureFlag::BetaFeature,
            // TODO 2: 在列表中加入新标志
        ]
    }

    fn label(&self) -> &'static str {
        match self {
            FeatureFlag::NewUI => "新 UI 界面",
            FeatureFlag::DarkMode => "深色模式",
            FeatureFlag::BetaFeature => "Beta 功能",
            // TODO 3: 为新标志添加中文标签
        }
    }
}

#[component]
fn Exercise() -> impl IntoView {
    let flags = RwSignal::new(HashSet::<FeatureFlag>::new());

    let is_enabled = move |flag: FeatureFlag| -> bool {
        // TODO 4: 判断 flag 是否已启用
        // 提示: flags.get().contains(&flag)
        false // placeholder，替换为实际逻辑
    };

    let toggle = move |flag: FeatureFlag| {
        // TODO 5: 切换 flag 状态—存在则移除，否则插入
        // 提示: flags.update(|set| { ... })
    };

    view! {
        <div style="padding: 20px; max-width: 600px; margin: 0 auto; font-family: system-ui, sans-serif;">
            <h2>"⚙️ 功能标志管理系统"</h2>
            <p style="color: #666;">"勾选下方的复选框来控制各项功能的可见性。"</p>

            {/* 控制面板 */}
            <div style="border: 1px solid #ddd; padding: 16px; border-radius: 8px; margin-bottom: 20px;">
                <h3>"控制面板"</h3>
                {FeatureFlag::all().into_iter().map(|flag| {
                    view! {
                        <label style="display: flex; align-items: center; gap: 8px; margin: 8px 0;
                                  cursor: pointer; padding: 4px 0;">
                            <input
                                type="checkbox"
                                prop:checked=is_enabled(flag)
                                on:click=move |_| toggle(flag)
                            />
                            <span>{flag.label()}</span>
                        </label>
                    }
                }).collect::<Vec<_>>()}
            </div>

            {/* 功能展示区 */}
            <div style="border: 1px solid #ddd; padding: 16px; border-radius: 8px;">
                <h3>"功能展示区"</h3>

                <Show when=move || flags.get().contains(&FeatureFlag::NewUI)>
                    <div style="background: #e3f2fd; border: 1px solid #90caf9; padding: 12px;
                                border-radius: 6px; margin: 8px 0;">
                        <strong>"✨ 新版 UI"</strong>
                        <p style="margin: 4px 0 0; font-size: 14px;">"更现代化的界面风格已启用。"</p>
                    </div>
                </Show>

                <Show when=move || flags.get().contains(&FeatureFlag::DarkMode)>
                    <div style="background: #263238; color: #eceff1; border: 1px solid #37474f;
                                padding: 12px; border-radius: 6px; margin: 8px 0;">
                        <strong>"🌙 深色模式"</strong>
                        <p style="margin: 4px 0 0; font-size: 14px;">"护眼的深色主题已生效。"</p>
                    </div>
                </Show>

                <Show when=move || flags.get().contains(&FeatureFlag::BetaFeature)>
                    <div style="background: #fff3e0; border: 1px solid #ffcc80; padding: 12px;
                                border-radius: 6px; margin: 8px 0;">
                        <strong>"🧪 Beta 功能"</strong>
                        <p style="margin: 4px 0 0; font-size: 14px; color: #e65100;">
                            "预览功能，仅供测试，请谨慎使用。"
                        </p>
                    </div>
                </Show>

                <Show when=move || !is_enabled(FeatureFlag::NewUI)
                    && !is_enabled(FeatureFlag::DarkMode)
                    && !is_enabled(FeatureFlag::BetaFeature)>
                    <p style="color: #aaa; text-align: center; padding: 20px;">
                        "请在上方的控制面板中启用至少一项功能。"
                    </p>
                </Show>
            </div>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
