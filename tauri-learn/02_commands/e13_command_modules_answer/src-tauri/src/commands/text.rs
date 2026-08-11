// 文本处理命令子模块。

/// 转大写。
#[tauri::command]
pub fn to_upper(s: String) -> String {
    s.to_uppercase()
}

/// 统计单词数量（按空白切分）。
#[tauri::command]
pub fn word_count(s: String) -> usize {
    s.split_whitespace().count()
}