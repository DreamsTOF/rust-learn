// ============================================================
// 练习 e349: 有限状态机 — 枚举 + 信号
//
// 核心知识点:
//   - 用枚举表示有限状态（Idle, Loading, Success, Error）
//   - 定义允许的状态转换规则
//   - 使用 RwSignal 驱动状态变更
//
// 难度: ⭐⭐
// ============================================================

use leptos::prelude::*;

#[derive(Clone)]
enum DataState {
    Idle,
    Loading,
    Success(String),
    Error(String),
}

impl DataState {
    fn transition(&self, next: &DataState) -> Result<DataState, String> {
        match (self, next) {
            (DataState::Idle, DataState::Loading) => Ok(next.clone()),
            (DataState::Loading, DataState::Success(_)) => Ok(next.clone()),
            (DataState::Loading, DataState::Error(_)) => Ok(next.clone()),
            (DataState::Success(_), DataState::Idle) => Ok(next.clone()),
            (DataState::Error(_), DataState::Idle) => Ok(next.clone()),
            _ => Err(format!("不允许的转换: {:?} -> {:?}", self, next)),
        }
    }
}

impl std::fmt::Debug for DataState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DataState::Idle => write!(f, "空闲"),
            DataState::Loading => write!(f, "加载中..."),
            DataState::Success(msg) => write!(f, "成功: {}", msg),
            DataState::Error(msg) => write!(f, "错误: {}", msg),
        }
    }
}

#[component]
fn Exercise() -> impl IntoView {
    let state = RwSignal::new(DataState::Idle);

    let transition_to = move |next: DataState| {
        if let Ok(new_state) = state.get().transition(&next) {
            state.set(new_state);
        }
    };

    let state_display = move || format!("{:?}", state.get());

    view! {
        <div>
            <h2>"有限状态机"</h2>
            <p>"当前状态: " {state_display}</p>
            <p>
                <button on:click=move |_| transition_to(DataState::Loading)>"开始加载"</button>
                <button on:click=move |_| transition_to(DataState::Success("数据加载完成".to_string()))>"加载成功"</button>
                <button on:click=move |_| transition_to(DataState::Error("网络连接失败".to_string()))>"加载失败"</button>
                <button on:click=move |_| transition_to(DataState::Idle)>"重置"</button>
            </p>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
