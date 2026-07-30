use leptos::prelude::*;

fn main() {
    mount_to_body(|| {
        // TODO: 演示信号与 Scope 生命周期绑定，scope 结束时自动清理

        let (count, set_count) = signal(0);

        // on_cleanup 注册当前 scope 销毁时的回调
        // 当 scope 结束（例如组件卸载），其中创建的所有信号都会被自动清理
        on_cleanup(|| {
            // 此 scope 创建的信号（count, set_count）在此处已被释放
        });

        // 在 scope 存活期间，信号正常工作
        set_count.set(42);
        assert_eq!(count(), 42);

        view! {
            <p>"练习 37 — 信号 Drop (signal_drop)"</p>
            <p>"count = " {count()}</p>
            <p>"信号绑定到当前 reactive scope，scope 结束时自动清理"</p>
            <details>
                <summary>"参考答案"</summary>
                <pre>
"use leptos::prelude::*;

// 每个信号都绑定到创建它的 reactive scope
// scope 销毁时，所有关联的信号自动 Drop
let (count, set_count) = signal(0);

on_cleanup(|| {
    // 此 scope 的信号在此被自动清理，无需手动管理
    // ReadSignal / WriteSignal 的 Drop 实现会从响应式图中移除
});

set_count.set(1);
assert_eq!(count(), 1);"
                </pre>
            </details>
        }
    });
}
