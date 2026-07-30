use crate::types::TocItem;

pub fn extract_toc(md: &str) -> Vec<TocItem> {
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
