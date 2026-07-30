use leptos::prelude::*;

// 本题目标：跟踪表单的 dirty（已修改）、submitted（已提交）、validating（验证中）状态。
//
// 步骤提示：
// 1. 创建 name、email 信号（RwSignal<String>）
// 2. 创建 dirty、submitted、validating 信号（RwSignal<bool>）
// 3. 用户输入时设置 dirty = true
// 4. 提交时设置 validating = true，验证通过后设置 submitted = true
// 5. 重置按钮将所有状态恢复初始值
// 6. 在界面中显示当前各状态

#[component]
fn Exercise() -> impl IntoView {
    // TODO: 创建 name 信号
    // TODO: 创建 email 信号
    // TODO: 创建 dirty 信号（bool）
    // TODO: 创建 submitted 信号（bool）
    // TODO: 创建 validating 信号（bool）

    // TODO: 编写 on_submit 回调
    // - prevent_default
    // - 设置 validating = true
    // - 简单验证：name 非空、email 非空且包含 @
    // - 通过则设置 submitted = true
    // - 设置 validating = false

    // TODO: 编写 reset 回调
    // - 清空 name、email
    // - 重置 dirty、submitted、validating 为 false

    view! {
        <div>
            <h2>"表单状态跟踪"</h2>
            <form on:submit=on_submit>
                <div>
                    <label>"姓名："</label>
                    {/*
                    TODO: 添加 <input type="text">
                    - prop:value 绑定 name
                    - on:input 更新 name 并设置 dirty = true
                    */}
                </div>
                <div>
                    <label>"邮箱："</label>
                    {/*
                    TODO: 添加 <input type="text">
                    - prop:value 绑定 email
                    - on:input 更新 email 并设置 dirty = true
                    */}
                </div>
                <button type="submit">"提交"</button>
                {/*
                TODO: 添加重置按钮 <button type="button" on:click=reset>
                */}
            </form>
            {/*
            TODO: 在边框 div 中显示表单状态：
            - validating => 橙色 "验证中..."
            - submitted => 绿色 "已提交"
            - dirty => 蓝色 "已修改（未提交）"
            - 否则 => 灰色 "未修改"
            - 分别显示 dirty/submitted/validating 的是/否状态
            */}
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
