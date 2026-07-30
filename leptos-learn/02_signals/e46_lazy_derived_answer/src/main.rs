use leptos::prelude::*;

fn main() {
    mount_to_body(|| view! { <Exercise/> });
}

#[component]
fn Exercise() -> impl IntoView {
   let (a, set_a) = signal(10);
   let derived = move || a.get() + 1;
   let memo = Memo::new(move |_| a.get() * 10);
   let (read_memo, set_read_memo) = signal(false);

   view! {
       <div>
           <p>"练习 46 (lazy_derived)"</p>
           <p>"信号 a = " {a.get()}</p>
           <p>"派生 a+1 = " {derived}</p>
           <p>"Memo a*10 = " {move || if read_memo.get() { memo.get().to_string() } else { "未读取（惰性）".to_string() }}</p>
           <button on:click=move |_| set_a.set(a.get() + 1)>"a + 1"</button>
           <button on:click=move |_| { set_read_memo.set(true); }>"读取 Memo"</button>
       </div>
   }
}
