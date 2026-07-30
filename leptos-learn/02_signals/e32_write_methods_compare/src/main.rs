// ============================================================
// 练习 e32: write_methods_compare — 四种写操作方式对比
//
// 核心知识点:
//   - set(value): 直接设置新值
//   - fn_call(value): 函数调用语法糖
//   - update(closure): 通过闭包获得 &mut T 修改
//   - write(): 获取写守卫 WriteGuard（DerefMut）
//
// 难度: ⭐⭐
// ============================================================

use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    let (count, set_count) = signal(0);

    let do_set = move |_| {
        // TODO: 使用 set() 直接设置新值
        set_count.set(100);
    };

    let do_fn_call = move |_| {
        // TODO: 使用函数调用语法（等价于 set）
        set_count(200);
    };

    let do_update = move |_| {
        // TODO: 使用 update() 通过闭包修改
        set_count.update(|n| *n += 1);
    };

    let do_write = move |_| {
        // TODO: 使用 write() 获取写守卫
        let mut guard = set_count.write();
        *guard += 10;
    };

    view! {
        <div>
            <p>"当前值: " {count}</p>
            <button on:click=do_set>"set(100)"</button>
            <button on:click=do_fn_call>"fn_call(200)"</button>
            <button on:click=do_update>"update(+1)"</button>
            <button on:click=do_write>"write(+10)"</button>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}

// <details>
// 参考答案（去除注释后的纯净版本）:
//
// use leptos::prelude::*;
//
// #[component]
// fn Exercise() -> impl IntoView {
//     let (count, set_count) = signal(0);
//
//     let do_set = move |_| set_count.set(100);
//     let do_fn_call = move |_| set_count(200);
//     let do_update = move |_| set_count.update(|n| *n += 1);
//     let do_write = move |_| {
//         let mut guard = set_count.write();
//         *guard += 10;
//     };
//
//     view! {
//         <div>
//             <p>"当前值: " {count}</p>
//             <button on:click=do_set>"set(100)"</button>
//             <button on:click=do_fn_call>"fn_call(200)"</button>
//             <button on:click=do_update>"update(+1)"</button>
//             <button on:click=do_write>"write(+10)"</button>
//         </div>
//     }
// }
//
// fn main() {
//     mount_to_body(Exercise);
// }
//
// 知识点:
// 1. set(value) 直接替换整个值
// 2. fn_call(value) 是 set 的语法糖
// 3. update(closure) 通过 &mut T 灵活修改，适合自增等操作
// 4. write() 返回 WriteGuard 守卫，通过 DerefMut 实现修改
// </details>
