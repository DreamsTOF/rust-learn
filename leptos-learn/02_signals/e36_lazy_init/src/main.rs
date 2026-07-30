use leptos::prelude::*;
use std::cell::Cell;
use std::rc::Rc;

fn main() {
    mount_to_body(|| {
        // TODO: 使用 Option 信号实现懒初始化，首次访问时计算并缓存

        let call_count = Rc::new(Cell::new(0u32));

        // 初始值为 None，首次访问时通过闭包计算并 set 为 Some
        let (value, set_value) = signal(None::<i32>);

        // 派生闭包：首次访问时计算，后续直接返回缓存值
        let cc = call_count.clone();
        let get_value = move || {
            let current = value.get();
            match current {
                Some(v) => v,
                None => {
                    cc.set(cc.get() + 1);
                    let computed = 42;
                    set_value.set(Some(computed));
                    computed
                }
            }
        };

        // 此时闭包尚未执行
        assert_eq!(call_count.get(), 0);
        assert_eq!(get_value(), 42); // 首次访问 → 闭包执行，call_count = 1
        assert_eq!(call_count.get(), 1);
        assert_eq!(get_value(), 42); // 使用缓存值，闭包不再执行
        assert_eq!(call_count.get(), 1); // 仍为 1

        view! {
            <p>"练习 36 — 信号懒初始化 (lazy_init)"</p>
            <p>"值 = " {get_value()} "，闭包执行次数 = " {call_count.get()}</p>
            <details>
                <summary>"参考答案"</summary>
                <pre>
"use leptos::prelude::*;
use std::rc::Rc;
use std::cell::Cell;

let call_count = Rc::new(Cell::new(0u32));
let (value, set_value) = signal(None::<i32>);

let cc = call_count.clone();
let get_value = move || {
    match value.get() {
        Some(v) => v,
        None => {
            cc.set(cc.get() + 1);
            set_value.set(Some(42));
            42
        }
    }
};
assert_eq!(get_value(), 42);
assert_eq!(call_count.get(), 1);"
                </pre>
            </details>
        }
    });
}
