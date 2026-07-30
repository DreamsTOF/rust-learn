use leptos::prelude::*;

fn main() {
    mount_to_body(move || {
        let (count, set_count) = signal(0);

        // TODO 练习:
        // 1. (危险) 取消下方注释，运行观察页面卡死：
        //    在 Effect 中读写同一信号会导致无限死循环
        // 2. 恢复注释，用下方的「正确」方式运行

        // 🔴 反模式 — 读写同一信号 → 死循环
        // Effect::new(move || {
        //     let n = count();
        //     set_count(n + 1); // count 变化 → Effect 重跑 → count 再变 → 无限循环
        // });

        // 🟢 正确 — untrack 读取值，切断依赖链
        Effect::new(move || {
            let n = untrack(move || count());
            set_count(n + 1);
        });

        view! {
            <p>"count: " {count}</p>
            <details>
                <summary>"💡 答案与解释"</summary>
                <p>"在 Effect 中读写同一信号会导致死循环："</p>
                <p>"每次写入→信号变化→Effect 重跑→再次写入→…… 无限循环。"</p>
                <p>"解决：用 untrack() 读取值，切断响应式依赖链。"</p>
            </details>
        }
    });
}
