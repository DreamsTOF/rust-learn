use leptos::prelude::*;

fn main() {
    mount_to_body(|| view! { <Exercise/> });
}

#[component]
fn Exercise() -> impl IntoView {
   let (count, set_count) = signal(42);
   let formatted = move || format!("当前计数: {} ({}的二进制: {:b})", count.get(), count.get(), count.get());

   view! {
       <div>
           <p>"练习 47 (derived_format)"</p>
           <p>{formatted}</p>
           <button on:click=move |_| set_count.set(count.get() + 1)>"+1"</button>
           <button on:click=move |_| set_count.set(count.get() * 2)>"×2"</button>
       </div>
   }
}
