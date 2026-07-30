use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
   let base = RwSignal::new(5);
   let derived: Vec<_> = (0..10).map(|i| move || base.get() + i).collect();

   view! {
       <div>
           <p>"练习 49: signal_array_derived"</p>
           <p>"base = " {base.get()}</p>
           <ul>
           {derived.into_iter().enumerate().map(|(idx, val)| {
               view! { <li>"[" {idx}"] = " {val}</li> }
           }).collect::<Vec<_>>()}
           </ul>
           <button on:click=move |_| base.set(base.get() + 1)>"base += 1"</button>
       </div>
   }
}

fn main() {
    mount_to_body(Exercise);
}
