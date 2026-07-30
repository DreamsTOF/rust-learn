// 练习 254 — 非受控输入框
//
// 目标: 使用 NodeRef 获取输入框的值，不直接绑定信号。
//
// 非受控模式 = 输入框的值由 DOM 自己维护:
// - NodeRef 持有 DOM 元素的引用
// - 在需要时（如点击按钮）通过 NodeRef 读取输入框的当前值
// - 不需要为每个按键更新信号

use leptos::prelude::*;
use leptos::html;

fn main() {
    mount_to_body(|| view! { <Exercise/> });
}

#[component]
fn Exercise() -> impl IntoView {
    // 创建 NodeRef 引用输入框 DOM 元素
    let input_ref: NodeRef<html::Input> = NodeRef::new();
    // 创建信号存储读取到的值
    let value = RwSignal::new(String::new());

    // 定义读取输入框值的函数
    // 提示: input_ref.get() 返回 Option<HtmlElement<html::Input>>
    //       然后调用 .value() 获取输入框的当前值
    let read_value = move |_: ev::Click| {
        if let Some(input) = todo!() {
            todo!()
        }
    };

    view! {
        <div>
            <h2>"练习 254: 非受控输入框"</h2>

            {/* 绑定 node_ref 到输入框 */}
            <input type="text" node_ref={todo!()} />

            {/* 读取值按钮 */}
            <button on:click=read_value>"读取值"</button>

            {/* 显示读取到的值 */}
            <p>"读取的值: " {todo!()}</p>
        </div>
    }
}
