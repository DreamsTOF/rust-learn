use leptos::prelude::*;

fn main() {
    mount_to_body(|| {
        // TODO: 使用 move || 闭包从信号派生新值

        let (count, set_count) = signal(5);

        // move || 闭包捕获 count 信号，每次调用时重新计算
        let double = move || count() * 2;

        // 验证派生逻辑
        assert_eq!(double(), 10);

        set_count.set(10);
        assert_eq!(double(), 20);

        view! {
            <p>"练习 38 — 派生闭包 (derived_closure)"</p>
            <p>"count = " {count()} "，double = " {double()}</p>
            <details>
                <summary>"参考答案"</summary>
                <pre>
"use leptos::prelude::*;

let (count, set_count) = signal(5);
// move || 闭包捕获信号，每次调用时读取最新值
let double = move || count() * 2;

assert_eq!(double(), 10);
set_count.set(10);
assert_eq!(double(), 20);"
                </pre>
            </details>
        }
    });
}
