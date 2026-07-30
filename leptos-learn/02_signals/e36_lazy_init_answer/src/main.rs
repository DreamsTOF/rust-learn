use leptos::prelude::*;
use std::cell::Cell;
use std::rc::Rc;

fn main() {
    mount_to_body(|| {
        let call_count = Rc::new(Cell::new(0u32));

        // 使用 Option 信号实现懒初始化：初始值为 None，首次访问时计算
        let (value, set_value) = signal(None::<i32>);

        // 派生闭包：首次访问时计算并缓存值，后续直接返回缓存值
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

        view! {
            <p>"练习 36 — 信号懒初始化 (lazy_init)"</p>
            <p>"值 = " {get_value()} "，闭包执行次数 = " {call_count.get()}</p>
        }
    });
}
