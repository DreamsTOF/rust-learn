// ============================================================
// 练习 e05: 组件定义 — 自定义组件与传参
//
// 核心知识点:
//   - 函数组件: #[component] + impl IntoView
//   - 组件属性（Props）: 函数参数
//   - 组件组合: 在 view! 中调用其他组件
//
// 难度: ⭐⭐ (补全约 50%，关键位置有 TODO)
// ============================================================

use leptos::prelude::*;

// TODO: 定义 Greeting 组件，接收 name: String 属性
// 提示: 参数直接写在函数签名中
#[component]
fn Greeting(name: String) -> impl IntoView {
    // TODO: 在 view! 中显示 "你好，{name}！"
    // 提示: 使用 {变量名} 进行插值
    view! {
        <p></p>
    }
}

// TODO: 定义 ProfileCard 组件，接收 name 和 bio 两个属性
// 提示: 参数用逗号分隔
#[component]
fn ProfileCard(name: String, bio: String) -> impl IntoView {
    // TODO: 返回一个 <div>，内含 <h3>（显示 name）和 <p>（显示 bio）
    view! {
        <div class="">
            <h3></h3>
            <p></p>
        </div>
    }
}

#[component]
fn Exercise() -> impl IntoView {
    // TODO: 在 view! 中调用 Greeting 和 ProfileCard 组件
    // 提示: 使用 <组件名 属性="值"/> 语法
    view! {
        <Greeting name={String::from("Leptos")}/>
        <ProfileCard name={String::from("Rustacean")} bio={String::from("热爱 Rust 和 Web 开发")}/>
    }
}

fn main() {
    mount_to_body(Exercise);
}
