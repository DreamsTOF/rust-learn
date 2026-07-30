use crate::types::TocItem;

pub fn extract_toc(md: &str) -> Vec<TocItem> {
    // TODO: 练习 - 从 Markdown 中提取目录
    // 提示: 遍历每一行，找到以 # 开头的行，计算标题层级(连续 # 数量)，
    //       提取标题文本，生成锚点 ID（转换为小写，空格替换为连字符，去除非字母数字字符）
    let mut toc = Vec::new();
    for line in md.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with('#') {
            let level = trimmed.chars().take_while(|c| *c == '#').count();
            let text = trimmed.trim_start_matches('#').trim().to_string();
            let anchor = text
                .to_lowercase()
                .replace(|c: char| !c.is_alphanumeric() && c != ' ', "")
                .replace(' ', "-");
            toc.push(TocItem {
                level,
                text,
                anchor,
            });
        }
    }
    toc
}
