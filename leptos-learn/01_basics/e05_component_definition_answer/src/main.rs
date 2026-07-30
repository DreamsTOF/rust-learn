// ============================================================
// 练习 e05: 组件定义 — 参考答案
//
// 核心知识点:
//   - 函数组件: #[component] + impl IntoView
//   - 组件属性（Props）: 函数参数
//   - 组件组合: 在 view! 中调用其他组件
// ============================================================

use leptos::prelude::*;

// Greeting 组件，接收 name: String 属性
#[component]
fn Greeting(name: String) -> impl IntoView {
    // 使用 {变量名} 进行插值
    view! {
        <p>"你好，" {name} "！"</p>
    }
}

// ProfileCard 组件，接收 name 和 bio 两个属性
#[component]
fn ProfileCard(name: String, bio: String) -> impl IntoView {
    // 返回一个 <div>，内含 <h3>（显示 name）和 <p>（显示 bio）
    view! {
        <div class="profile-card">
            <h3>{name}</h3>
            <p>{bio}</p>
        </div>
    }
}

#[component]
fn Exercise() -> impl IntoView {
    // 调用 Greeting 和 ProfileCard 组件
    view! {
        <Greeting name={String::from("Leptos")}/>
        <ProfileCard name={String::from("Rustacean")} bio={String::from("热爱 Rust 和 Web 开发")}/>
    }
}

fn main() {
    mount_to_body(Exercise);
}
