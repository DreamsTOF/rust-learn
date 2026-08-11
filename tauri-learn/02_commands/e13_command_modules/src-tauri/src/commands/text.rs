// ============================================================
// 练习 E13: 命令模块化（text 子模块）
// TODO: 按照注释提示补全代码
// ============================================================

// 练习版允许存在未使用代码（TODO 尚未完成），
// 完成全部填空后该属性可删除。
#![allow(dead_code)]

// === 步骤 4: 实现转大写命令 ————————————————————————————————————
// TODO: 添加 #[tauri::command] 属性并补全函数体
// 提示: pub fn to_upper(s: String) -> String { s.to_uppercase() }
pub fn to_upper(_s: String) -> String {
    // TODO: 返回大写字符串
    String::new()
}

// === 步骤 5: 实现单词计数命令 ————————————————————————————————————
// TODO: 添加 #[tauri::command] 属性并补全函数体
// 提示: pub fn word_count(s: String) -> usize { s.split_whitespace().count() }
pub fn word_count(_s: String) -> usize {
    // TODO: 按空白切分统计单词数
    0
}