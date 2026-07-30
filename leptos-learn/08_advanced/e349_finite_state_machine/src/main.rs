// ============================================================
// 练习 e349: 有限状态机 — 枚举 + 信号
//
// 核心知识点:
//   - 用枚举表示有限状态（Idle, Loading, Success, Error）
//   - 定义允许的状态转换规则
//   - 使用 RwSignal 驱动状态变更
//
// 难度: ⭐⭐ (关键位置有 TODO，补全约 50%)
// ============================================================

use leptos::prelude::*;

// TODO: 定义 DataState 枚举（Idle, Loading, Success(String), Error(String)）
// 提示: 需要派生 Clone 以便在闭包中使用

// TODO: 为 DataState 实现一个 transition 方法，检查状态转换是否允许
// 允许的转换规则:
//   Idle -> Loading
//   Loading -> Success | Error
//   Success -> Idle
//   Error -> Idle
// 签名: fn transition(&self, next: &DataState) -> Result<DataState, String>

// TODO: 为 DataState 实现 std::fmt::Debug，用中文描述状态
// Idle -> "空闲", Loading -> "加载中...", Success(msg) -> "成功: {msg}", Error(msg) -> "错误: {msg}"

#[component]
fn Exercise() -> impl IntoView {
    // TODO: 使用 RwSignal 持有当前状态，初始为 DataState::Idle

    // TODO: 创建 transition_to 辅助函数，接收下一状态，若允许则转换

    // TODO: 显示当前状态和四个转换按钮

    view! {
        <div>
            <h2>"有限状态机"</h2>
            // TODO: 显示当前状态
            // TODO: 添加"开始加载"、"加载成功"、"加载失败"、"重置"按钮
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}

// <details>
// 参考答案（去除注释后的纯净版本）:
//
// use leptos::prelude::*;
//
// #[derive(Clone)]
// enum DataState {
//     Idle,
//     Loading,
//     Success(String),
//     Error(String),
// }
//
// impl DataState {
//     fn transition(&self, next: &DataState) -> Result<DataState, String> {
//         match (self, next) {
//             (DataState::Idle, DataState::Loading) => Ok(next.clone()),
//             (DataState::Loading, DataState::Success(_)) => Ok(next.clone()),
//             (DataState::Loading, DataState::Error(_)) => Ok(next.clone()),
//             (DataState::Success(_), DataState::Idle) => Ok(next.clone()),
//             (DataState::Error(_), DataState::Idle) => Ok(next.clone()),
//             _ => Err(format!("不允许的转换: {:?} -> {:?}", self, next)),
//         }
//     }
// }
//
// impl std::fmt::Debug for DataState {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             DataState::Idle => write!(f, "空闲"),
//             DataState::Loading => write!(f, "加载中..."),
//             DataState::Success(msg) => write!(f, "成功: {}", msg),
//             DataState::Error(msg) => write!(f, "错误: {}", msg),
//         }
//     }
// }
//
// #[component]
// fn Exercise() -> impl IntoView {
//     let state = RwSignal::new(DataState::Idle);
//
//     let transition_to = move |next: DataState| {
//         if let Ok(new_state) = state.get().transition(&next) {
//             state.set(new_state);
//         }
//     };
//
//     let state_display = move || format!("{:?}", state.get());
//
//     view! {
//         <div>
//             <h2>"有限状态机"</h2>
//             <p>"当前状态: " {state_display}</p>
//             <p>
//                 <button on:click=move |_| transition_to(DataState::Loading)>"开始加载"</button>
//                 <button on:click=move |_| transition_to(DataState::Success("数据加载完成".to_string()))>"加载成功"</button>
//                 <button on:click=move |_| transition_to(DataState::Error("网络连接失败".to_string()))>"加载失败"</button>
//                 <button on:click=move |_| transition_to(DataState::Idle)>"重置"</button>
//             </p>
//         </div>
//     }
// }
//
// fn main() {
//     mount_to_body(Exercise);
// }
//
// ### 知识点
// - 枚举是表达有限状态集合的自然方式
// - transition 方法强制校验状态转换合法性，防止非法状态
// - RwSignal 驱动状态变更，视图自动响应
// - 有限状态机确保程序不会进入未定义状态
// - 在 GUI 中常用于管理异步操作的生命周期
// </details>
