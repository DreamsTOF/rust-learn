use leptos::prelude::*;

// 本题目标：为表单添加验证逻辑，在提交时检查邮箱格式和密码长度。
//
// 步骤提示：
// 1. 创建 email、password 信号（RwSignal<String>）
// 2. 创建 errors 信号（Vec<String>）和 submitted 信号（bool）
// 3. 编写 validate 函数，检查邮箱是否包含 @、密码长度是否 >= 6
// 4. 在 on_submit 中调用 validate()，只有通过才设置 submitted
// 5. 显示错误列表或成功消息

#[component]
fn Exercise() -> impl IntoView {
    // TODO: 创建 email 信号
    // TODO: 创建 password 信号
    // TODO: 创建 errors 信号（Vec<String>）
    // TODO: 创建 submitted 信号（bool）

    // TODO: 编写 validate 函数
    // - 邮箱为空 => "邮箱不能为空"
    // - 邮箱不含 @ => "邮箱格式不正确"
    // - 密码为空 => "密码不能为空"
    // - 密码长度 < 6 => "密码长度不足"
    // - 将错误信息收集到 errors 中，返回是否无错误

    // TODO: 编写 on_submit 回调
    // - prevent_default
    // - 调用 validate()，通过则设置 submitted

    view! {
        <div>
            <h2>"表单验证示例"</h2>
            <form on:submit=on_submit>
                <div>
                    <label>"邮箱："</label>
                    {/*
                    TODO: 添加 <input type="text">
                    - prop:value 绑定 email
                    - on:input 更新 email
                    */}
                </div>
                <div>
                    <label>"密码："</label>
                    {/*
                    TODO: 添加 <input type="password">
                    - prop:value 绑定 password
                    - on:input 更新 password
                    */}
                </div>
                <button type="submit">"提交"</button>
            </form>
            {/*
            TODO: 条件渲染
            - errors 非空 => 显示错误列表（红色）
            - submitted 为 true => 显示 "验证通过！"（绿色）
            */}
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
