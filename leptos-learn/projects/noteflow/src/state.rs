use crate::types::{DocNode, TabInfo, TagInfo, UserInfo};
use leptos::prelude::*;

#[derive(Clone)]
pub struct AppState {
    // TODO: 练习 - 添加应用状态字段
    // 提示: 需要文档列表(RwSignal<Vec<DocNode>>)、当前选中文档ID、打开的标签页、
    //       激活的标签页ID、当前用户、主题、标签、搜索查询、在线状态等信号
    pub docs: RwSignal<Vec<DocNode>>,
    pub selected_doc_id: RwSignal<Option<String>>,
    pub open_tabs: RwSignal<Vec<TabInfo>>,
    pub active_tab_id: RwSignal<Option<String>>,
    pub current_user: RwSignal<Option<UserInfo>>,
    pub theme: RwSignal<String>,
    pub tags: RwSignal<Vec<TagInfo>>,
    pub search_query: RwSignal<String>,
    pub is_online: RwSignal<bool>,
}

impl AppState {
    pub fn new() -> Self {
        // TODO: 练习 - 初始化所有状态字段
        // 提示: 使用 RwSignal::new() 创建每个信号，docs 和 tags 初始化为空 Vec，
        //       theme 初始化为 "light"，is_online 初始化为 true
        Self {
            docs: RwSignal::new(Vec::new()),
            selected_doc_id: RwSignal::new(None),
            open_tabs: RwSignal::new(Vec::new()),
            active_tab_id: RwSignal::new(None),
            current_user: RwSignal::new(None),
            theme: RwSignal::new(String::from("light")),
            tags: RwSignal::new(Vec::new()),
            search_query: RwSignal::new(String::new()),
            is_online: RwSignal::new(true),
        }
    }
}
