use leptos::prelude::*;

fn main() {
    mount_to_body(move || {
        let (count, set_count) = signal(0);

        Effect::new(move || {
            let n = untrack(move || count());
            set_count(n + 1);
        });

        view! {
            <p>"count: " {count}</p>
        }
    });
}
