use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    let matrix = vec![
        vec![1, 2, 3],
        vec![4, 5, 6],
        vec![7, 8, 9],
    ];

    view! {
        <h3>"二维矩阵"</h3>
        <For each=move || matrix.clone() key=|row| row[0] let(row)>
            <div style="display: flex; gap: 6px; margin: 4px 0;">
                <For each=move || row.clone() key=|&x| x let(cell)>
                    <span style="border: 1px solid #888; padding: 4px 10px; min-width: 24px; text-align: center;">
                        {cell}
                    </span>
                </For>
            </div>
        </For>
    }
}

fn main() {
    mount_to_body(Exercise);
}
