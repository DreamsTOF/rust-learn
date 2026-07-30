use leptos::prelude::*;

// 本题目标：在用户输入时即时校验，并实时显示错误/成功提示。
//
// 步骤提示：
// 1. 创建 username、email 信号（RwSignal<String>）
// 2. 编写 username_error 闭包：空 => 错误，长度 < 3 => 错误
// 3. 编写 email_error 闭包：空 => 错误，不含 @ => 错误
// 4. 用 is_valid 派生信号判断整体是否有效
// 5. 在每个输入框旁实时显示对应的错误信息（或留空表示通过）

#[component]
fn Exercise() -> impl IntoView {
    // TODO: 创建 username 信号
    // TODO: 创建 email 信号

    // TODO: 编写 username_error 闭包
    // - 返回 Option<String>
    // - 空 => "用户名不能为空"
    // - 长度 < 3 => "用户名至少需要 3 个字符"

    // TODO: 编写 email_error 闭包
    // - 返回 Option<String>
    // - 空 => "邮箱不能为空"
    // - 不含 @ => "邮箱格式不正确"

    // TODO: 编写 is_valid 闭包判断整体是否通过

    view! {
        <div>
            <h2>"实时验证示例"</h2>
            <div>
                <label>"用户名："</label>
                {/*
                TODO: 添加 <input type="text">
                - prop:value 绑定 username
                - on:input 更新 username
                */}
                {/*
                TODO: 实时显示 username_error（红色，左边距 8px）
                提示：username_error().map(|e| view! { <span>...<span> })
                */}
            </div>
            <div>
                <label>"邮箱："</label>
                {/*
                TODO: 添加 <input type="text">
                - prop:value 绑定 email
                - on:input 更新 email
                */}
                {/*
                TODO: 实时显示 email_error
                */}
            </div>
            <p>
                {/*
                TODO: 显示整体验证状态
                - is_valid() => 绿色 "所有字段验证通过！"
                - 否则 => 灰色 "请填写正确的信息"
                */}
            </p>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
