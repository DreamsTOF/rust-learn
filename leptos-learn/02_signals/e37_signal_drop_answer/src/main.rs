use leptos::prelude::*;

fn main() {
    mount_to_body(|| {
        let (count, set_count) = signal(0);

        on_cleanup(|| {});

        set_count.set(42);
        assert_eq!(count(), 42);

        view! {
            <p>"练习 37 — 信号 Drop (signal_drop)"</p>
            <p>"count = " {count()}</p>
            <p>"信号绑定到当前 reactive scope，scope 结束时自动清理"</p>
        }
    });
}
