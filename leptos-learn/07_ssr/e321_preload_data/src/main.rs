use leptos::prelude::*;

// TODO: Create a Resource to simulate async SSR preloading
// The preload_data pattern in SSR loads data on the server before sending HTML.
// Use Resource::new(|| (), |_| async { ... }) to fetch data.
// Hints:
//   1. Define a #[component] fn Exercise() -> impl IntoView
//   2. Create a Resource inside the component
//   3. Use <Suspense fallback=|| view! { <p>... }> to handle loading state
//   4. Display the data with resource.map(|v| v.clone())

#[component]
fn Exercise() -> impl IntoView {
    // TODO: Create Resource here
    // let data = Resource::new(
    //     || (),                           // source: triggers refetch when changed
    //     |_| async { "Hello from preloaded data!" },  // fetcher
    // );

    // TODO: Use <Suspense> wrapping resource.map() to display data
    view! {
        <div>
            <h2>"Preload Data Pattern"</h2>
            // <Suspense fallback=|| view! { <p>"Loading..."</p> }>
            //     <p>{data.map(|d| d.map(|v| v.to_string()))}</p>
            // </Suspense>
            <p>"练习 321 (preload_data)"</p>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
