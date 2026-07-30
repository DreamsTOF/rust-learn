use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
   let a = RwSignal::new(10);
   let b = RwSignal::new(10);
   let is_equal = move || a.get() == b.get();

   view! {
       <div>
           <p>"练习 50: reactive_eq"</p>
           <p>"a = " {a.get()}", b = " {b.get()}</p>
           <p>"a == b ? " {is_equal}</p>
           <button on:click=move |_| a.set(a.get() + 1)>"a += 1"</button>
           <button on:click=move |_| b.set(b.get() + 1)>"b += 1"</button>
       </div>
   }
}

fn main() {
    mount_to_body(Exercise);
}
