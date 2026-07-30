use leptos::prelude::*;

// 本题目标：显示多条验证错误消息，如「必填」「格式错误」「长度不足」。
//
// 步骤提示：
// 1. 创建 email、password、errors（Vec<String>）、submitted 信号
// 2. 为每个字段定义多条验证规则
// 3. 每条规则生成不同的错误消息（必填 / 格式错误 / 长度不足 / 需包含数字 / 需包含大写字母）
// 4. 提交时执行 validate()，收集所有错误到 errors 中
// 5. 显示完整错误列表或成功消息

#[component]
fn Exercise() -> impl IntoView {
    // TODO: 创建 email 信号
    // TODO: 创建 password 信号
    // TODO: 创建 errors 信号（Vec<String>）
    // TODO: 创建 submitted 信号（bool）

    // TODO: 编写 validate 函数
    // 邮箱规则：
    // - 空 => "邮箱：必填"
    // - 不含 @ => "邮箱：格式错误（缺少 @）"
    // - 不含 . => "邮箱：格式错误（缺少域名后缀）"
    // 密码规则：
    // - 空 => "密码：必填"
    // - 长度 < 6 => "密码：长度不足（至少 6 位）"
    // - 不含数字 => "密码：需包含数字"
    // - 不含大写字母 => "密码：需包含大写字母"

    // TODO: 编写 on_submit 回调
    // - prevent_default
    // - 调用 validate()，通过则设置 submitted

    view! {
        <div>
            <h2>"验证消息示例"</h2>
            <form on:submit=on_submit>
                <div>
                    <label>"邮箱："</label>
                    {/*
                    TODO: 添加 <input type="text">
                    - prop:value 绑定 email
                    - on:input 更新 email，同时重置 submitted
                    */}
                </div>
                <div>
                    <label>"密码："</label>
                    {/*
                    TODO: 添加 <input type="password">
                    - prop:value 绑定 password
                    - on:input 更新 password，同时重置 submitted
                    */}
                </div>
                <button type="submit">"提交"</button>
            </form>
            {/*
            TODO: 显示验证结果
            - errors 非空 => 红色 div，标题 "请修正以下错误：" + 带 <li> 的列表
            - submitted 为 true => 绿色 "验证通过！所有字段符合要求。"
            */}
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
