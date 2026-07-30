 /**
 * Tauri v2 练习项目 - 批量初始化脚本
 * 
 * 根据 tauri-learn-plan.md 的定义，批量创建全部 355 道练习和答案项目。
 * 每个练习同时创建 exercise/ 和 exercise_answer/ 两个项目文件夹。
 * 
 * 用法: node scripts/bulk-init-all.js
 */
 
 const fs = require("fs");
 const path = require("path");
 
 const PROJECT_ROOT = path.resolve(__dirname, "..");
 const TEMPLATE_DIR = path.join(PROJECT_ROOT, "templates", "vite-ts");
 
 // ====================================================================
 // 练习数据定义
 // ====================================================================
 const ALL_EXERCISES = [
   // === 第 1 章: 环境准备 (01_environment, e001-e010) ===
   { chapter: "01_environment", number: 1,  name: "check_rust",        title: "Rust 环境检查" },
   { chapter: "01_environment", number: 2,  name: "check_node",        title: "Node.js 环境检查" },
   { chapter: "01_environment", number: 3,  name: "check_webview",     title: "WebView2 环境检查" },
   { chapter: "01_environment", number: 4,  name: "install_cli",       title: "安装 Tauri CLI" },
   { chapter: "01_environment", number: 5,  name: "create_first_app",  title: "创建第一个 Tauri 应用" },
   { chapter: "01_environment", number: 6,  name: "project_structure", title: "项目结构详解" },
   { chapter: "01_environment", number: 7,  name: "dev_server",        title: "开发服务器与热更新" },
   { chapter: "01_environment", number: 8,  name: "build_app",         title: "构建与产物分析" },
   { chapter: "01_environment", number: 9,  name: "cross_platform",    title: "跨平台开发注意事项" },
   { chapter: "01_environment", number: 10, name: "env_summary",       title: "综合: 环境诊断工具" },
 
   // === 第 2 章: 基础入门 (02_basics, e011-e030) ===
   { chapter: "02_basics", number: 11, name: "hello_world",         title: "Hello World" },
   { chapter: "02_basics", number: 12, name: "html_view",           title: "HTML 视图与文本节点" },
   { chapter: "02_basics", number: 13, name: "html_elements",       title: "HTML 元素与属性" },
   { chapter: "02_basics", number: 14, name: "element_nesting",     title: "元素嵌套与层级结构" },
   { chapter: "02_basics", number: 15, name: "first_command",       title: "第一个 Rust 命令" },
   { chapter: "02_basics", number: 16, name: "frontend_invoke",     title: "从前端调用命令" },
   { chapter: "02_basics", number: 17, name: "config_tauri",        title: "tauri.conf.json 配置详解" },
   { chapter: "02_basics", number: 18, name: "dev_vs_prod",         title: "开发模式 vs 生产模式" },
   { chapter: "02_basics", number: 19, name: "build_commands",      title: "构建命令与 Vite 集成" },
   { chapter: "02_basics", number: 20, name: "window_properties",   title: "窗口基础属性配置" },
   { chapter: "02_basics", number: 21, name: "multi_window",        title: "多窗口配置" },
   { chapter: "02_basics", number: 22, name: "window_size",         title: "窗口大小与约束" },
   { chapter: "02_basics", number: 23, name: "window_position",     title: "窗口位置与居中" },
   { chapter: "02_basics", number: 24, name: "fullscreen",          title: "窗口全屏与最大化" },
   { chapter: "02_basics", number: 25, name: "cargo_tauri_cli",     title: "使用 cargo tauri CLI" },
   { chapter: "02_basics", number: 26, name: "web_inspector",       title: "调试: Web Inspector" },
   { chapter: "02_basics", number: 27, name: "rust_logging",        title: "调试: Rust 日志" },
   { chapter: "02_basics", number: 28, name: "csp_security",        title: "理解 CSP 内容安全策略" },
   { chapter: "02_basics", number: 29, name: "app_icons",           title: "应用图标与资源" },
   { chapter: "02_basics", number: 30, name: "tauri_vs_browser",    title: "浏览器与 Tauri 环境区分" },
 
   // === 第 3 章: 命令与 IPC (03_commands_ipc, e031-e095) ===
   { chapter: "03_commands_ipc", number: 31, name: "cmd_no_args",           title: "无参无返回值命令" },
   { chapter: "03_commands_ipc", number: 32, name: "cmd_string_param",      title: "字符串参数" },
   { chapter: "03_commands_ipc", number: 33, name: "cmd_number_param",      title: "数字参数" },
   { chapter: "03_commands_ipc", number: 34, name: "cmd_boolean_param",     title: "布尔参数" },
   { chapter: "03_commands_ipc", number: 35, name: "cmd_return_string",     title: "返回字符串" },
   { chapter: "03_commands_ipc", number: 36, name: "cmd_return_number",     title: "返回数字" },
   { chapter: "03_commands_ipc", number: 37, name: "cmd_return_bool",       title: "返回布尔值" },
   { chapter: "03_commands_ipc", number: 38, name: "cmd_return_struct",     title: "返回结构体" },
   { chapter: "03_commands_ipc", number: 39, name: "cmd_return_vec",        title: "返回 Vec/数组" },
   { chapter: "03_commands_ipc", number: 40, name: "cmd_return_option",     title: "返回 Option" },
   { chapter: "03_commands_ipc", number: 41, name: "cmd_struct_input",      title: "复合参数: 结构体作为输入" },
   { chapter: "03_commands_ipc", number: 42, name: "cmd_optional_args",     title: "参数默认值与可选参数" },
   { chapter: "03_commands_ipc", number: 43, name: "cmd_mixed_args",        title: "多个参数混合" },
   { chapter: "03_commands_ipc", number: 44, name: "cmd_naming",            title: "命令命名规范" },
   { chapter: "03_commands_ipc", number: 45, name: "cmd_modular",           title: "模块化组织命令" },
   { chapter: "03_commands_ipc", number: 46, name: "async_cmd_basic",       title: "基础异步命令" },
   { chapter: "03_commands_ipc", number: 47, name: "async_read_file",       title: "异步文件读取" },
   { chapter: "03_commands_ipc", number: 48, name: "async_http_req",        title: "异步 HTTP 请求" },
   { chapter: "03_commands_ipc", number: 49, name: "async_db_query",        title: "异步数据库查询" },
   { chapter: "03_commands_ipc", number: 50, name: "async_timeout",         title: "异步超时控制" },
   { chapter: "03_commands_ipc", number: 51, name: "async_parallel",        title: "异步任务并行" },
   { chapter: "03_commands_ipc", number: 52, name: "async_app_handle",      title: "异步中访问 AppHandle" },
   { chapter: "03_commands_ipc", number: 53, name: "async_window_access",   title: "异步中访问窗口" },
   { chapter: "03_commands_ipc", number: 54, name: "async_borrow_issue",    title: "异步中借用问题" },
   { chapter: "03_commands_ipc", number: 55, name: "async_error_handling",  title: "异步命令错误处理" },
   { chapter: "03_commands_ipc", number: 56, name: "long_running_task",     title: "长时间运行任务" },
   { chapter: "03_commands_ipc", number: 57, name: "spawn_concurrency",     title: "使用 spawn 管理并发" },
   { chapter: "03_commands_ipc", number: 58, name: "cancel_task",           title: "取消长时间运行任务" },
   { chapter: "03_commands_ipc", number: 59, name: "rate_limiting",         title: "限流与节流" },
   { chapter: "03_commands_ipc", number: 60, name: "file_hash_calculator",  title: "综合: 文件哈希计算器" },
   { chapter: "03_commands_ipc", number: 61, name: "di_app_handle",         title: "访问 AppHandle" },
   { chapter: "03_commands_ipc", number: 62, name: "di_webview_window",     title: "访问 WebviewWindow" },
   { chapter: "03_commands_ipc", number: 63, name: "di_webview",            title: "访问 Webview" },
   { chapter: "03_commands_ipc", number: 64, name: "di_app",                title: "访问 App" },
   { chapter: "03_commands_ipc", number: 65, name: "di_managed_state",      title: "注入 Managed State" },
   { chapter: "03_commands_ipc", number: 66, name: "di_multiple_deps",      title: "同时注入多个依赖" },
   { chapter: "03_commands_ipc", number: 67, name: "di_order",              title: "依赖注入顺序" },
   { chapter: "03_commands_ipc", number: 68, name: "di_window_label",       title: "访问窗口标签" },
   { chapter: "03_commands_ipc", number: 69, name: "di_resize_window",      title: "操作窗口 resize/move" },
   { chapter: "03_commands_ipc", number: 70, name: "di_minimize_window",    title: "操作窗口 minimize/maximize" },
   { chapter: "03_commands_ipc", number: 71, name: "di_window_state",       title: "获取窗口状态" },
   { chapter: "03_commands_ipc", number: 72, name: "di_multi_window_com",   title: "多窗口间通信" },
   { chapter: "03_commands_ipc", number: 73, name: "di_custom_setup",       title: "自定义依赖注入" },
   { chapter: "03_commands_ipc", number: 74, name: "di_testing",            title: "依赖注入与测试" },
   { chapter: "03_commands_ipc", number: 75, name: "window_manager",        title: "综合: 窗口管理器" },
   { chapter: "03_commands_ipc", number: 76, name: "error_result_basic",    title: "Result 基础" },
   { chapter: "03_commands_ipc", number: 77, name: "error_string",          title: "字符串错误" },
   { chapter: "03_commands_ipc", number: 78, name: "error_custom_enum",     title: "自定义错误枚举" },
   { chapter: "03_commands_ipc", number: 79, name: "error_code_msg",        title: "错误码与错误消息" },
   { chapter: "03_commands_ipc", number: 80, name: "error_conversion",      title: "错误类型转换" },
   { chapter: "03_commands_ipc", number: 81, name: "error_from_trait",      title: "From trait 实现" },
   { chapter: "03_commands_ipc", number: 82, name: "error_chain",           title: "错误链" },
   { chapter: "03_commands_ipc", number: 83, name: "error_propagation",     title: "错误在命令链中传播" },
   { chapter: "03_commands_ipc", number: 84, name: "error_frontend",        title: "前端错误统一处理" },
   { chapter: "03_commands_ipc", number: 85, name: "file_read_error",       title: "综合: 文件读取与错误处理" },
   { chapter: "03_commands_ipc", number: 86, name: "ipc_batch_request",     title: "批量请求与去重" },
   { chapter: "03_commands_ipc", number: 87, name: "ipc_retry",             title: "命令重试机制" },
   { chapter: "03_commands_ipc", number: 88, name: "ipc_timeout",           title: "请求超时" },
   { chapter: "03_commands_ipc", number: 89, name: "ipc_cancel",            title: "请求取消" },
   { chapter: "03_commands_ipc", number: 90, name: "ipc_benchmark",         title: "IPC 性能基准测试" },
   { chapter: "03_commands_ipc", number: 91, name: "ipc_large_file",        title: "大文件传输优化" },
   { chapter: "03_commands_ipc", number: 92, name: "ipc_binary_data",       title: "二进制数据传输" },
   { chapter: "03_commands_ipc", number: 93, name: "ipc_channel",           title: "使用 Channel 流式传输" },
   { chapter: "03_commands_ipc", number: 94, name: "ipc_channel_progress",  title: "Channel 进度通知" },
   { chapter: "03_commands_ipc", number: 95, name: "ipc_channel_vs_event",  title: "Channel vs 事件对比" },
 
   // === 第 4 章: 状态管理与配置 (04_state_config, e096-e130) ===
   { chapter: "04_state_config", number: 96,  name: "state_register",        title: "注册 Managed State" },
   { chapter: "04_state_config", number: 97,  name: "state_read",            title: "在命令中读取 State" },
   { chapter: "04_state_config", number: 98,  name: "state_mutex",           title: "可变 State Mutex" },
   { chapter: "04_state_config", number: 99,  name: "state_rwlock",          title: "可变 State RwLock" },
   { chapter: "04_state_config", number: 100, name: "state_lifetime",        title: "State 生命周期" },
   { chapter: "04_state_config", number: 101, name: "state_multiple",        title: "多 State 管理" },
   { chapter: "04_state_config", number: 102, name: "state_complex_type",    title: "State 中的复杂类型" },
   { chapter: "04_state_config", number: 103, name: "state_async_init",      title: "State 初始化中执行异步" },
   { chapter: "04_state_config", number: 104, name: "state_hot_update",      title: "State 的热更新" },
   { chapter: "04_state_config", number: 105, name: "state_thread_safe",     title: "线程安全的 State" },
   { chapter: "04_state_config", number: 106, name: "state_plugin_interop",  title: "State 与插件交互" },
   { chapter: "04_state_config", number: 107, name: "state_default",         title: "State 的默认值" },
   { chapter: "04_state_config", number: 108, name: "state_testing",         title: "State 测试" },
   { chapter: "04_state_config", number: 109, name: "state_once_cell",       title: "使用 OnceCell/OnceLock" },
   { chapter: "04_state_config", number: 110, name: "config_manager",        title: "综合: 应用配置管理器" },
   { chapter: "04_state_config", number: 111, name: "config_read",           title: "读取 tauri.conf.json" },
   { chapter: "04_state_config", number: 112, name: "config_custom_field",   title: "自定义配置字段" },
   { chapter: "04_state_config", number: 113, name: "config_deserialize",    title: "配置反序列化" },
   { chapter: "04_state_config", number: 114, name: "config_platform",       title: "平台特定配置" },
   { chapter: "04_state_config", number: 115, name: "config_env_override",   title: "环境变量覆盖配置" },
   { chapter: "04_state_config", number: 116, name: "config_runtime",        title: "运行时修改配置" },
   { chapter: "04_state_config", number: 117, name: "config_validation",     title: "配置文件验证" },
   { chapter: "04_state_config", number: 118, name: "config_custom_path",    title: "自定义配置文件路径" },
   { chapter: "04_state_config", number: 119, name: "config_hot_reload",     title: "配置热重载" },
   { chapter: "04_state_config", number: 120, name: "multi_env_config",      title: "综合: 多环境配置管理器" },
   { chapter: "04_state_config", number: 121, name: "path_embed_resource",   title: "嵌入资源文件" },
   { chapter: "04_state_config", number: 122, name: "path_resource_dir",     title: "读取应用资源目录" },
   { chapter: "04_state_config", number: 123, name: "path_app_data",         title: "读取应用数据目录" },
   { chapter: "04_state_config", number: 124, name: "path_app_config",       title: "读取应用配置目录" },
   { chapter: "04_state_config", number: 125, name: "path_documents",        title: "读取文档目录" },
   { chapter: "04_state_config", number: 126, name: "path_downloads",        title: "读取下载目录" },
   { chapter: "04_state_config", number: 127, name: "path_desktop",          title: "读取桌面目录" },
   { chapter: "04_state_config", number: 128, name: "path_cross_platform",   title: "路径解析最佳实践" },
   { chapter: "04_state_config", number: 129, name: "path_temp",             title: "临时文件与目录" },
   { chapter: "04_state_config", number: 130, name: "path_cache_log",        title: "缓存目录与日志目录" },
 
   // === 第 5 章: 事件与生命周期 (05_events_lifecycle, e131-e160) ===
   { chapter: "05_events_lifecycle", number: 131, name: "event_emit_backend",       title: "后端发送全局事件" },
   { chapter: "05_events_lifecycle", number: 132, name: "event_listen_backend",     title: "后端监听事件" },
   { chapter: "05_events_lifecycle", number: 133, name: "event_global_vs_webview",  title: "全局事件 vs Webview 事件" },
   { chapter: "05_events_lifecycle", number: 134, name: "event_frontend_listen",    title: "前端监听后端事件" },
   { chapter: "05_events_lifecycle", number: 135, name: "event_frontend_emit",      title: "前端发送事件到后端" },
   { chapter: "05_events_lifecycle", number: 136, name: "event_unlisten",           title: "事件取消监听" },
   { chapter: "05_events_lifecycle", number: 137, name: "event_once",               title: "一次性监听" },
   { chapter: "05_events_lifecycle", number: 138, name: "event_payload",            title: "事件负载类型" },
   { chapter: "05_events_lifecycle", number: 139, name: "event_throttle",           title: "事件频率控制" },
   { chapter: "05_events_lifecycle", number: 140, name: "event_dedup",              title: "事件去重" },
   { chapter: "05_events_lifecycle", number: 141, name: "event_timeout",            title: "事件超时" },
   { chapter: "05_events_lifecycle", number: 142, name: "event_naming",             title: "事件命名规范" },
   { chapter: "05_events_lifecycle", number: 143, name: "event_debug",              title: "事件调试" },
   { chapter: "05_events_lifecycle", number: 144, name: "event_security",           title: "事件安全" },
   { chapter: "05_events_lifecycle", number: 145, name: "event_bus",                title: "综合: 事件总线" },
   { chapter: "05_events_lifecycle", number: 146, name: "lifecycle_setup",          title: "Setup 钩子" },
   { chapter: "05_events_lifecycle", number: 147, name: "lifecycle_main_thread",    title: "RunOnMainThread" },
   { chapter: "05_events_lifecycle", number: 148, name: "lifecycle_exit_event",     title: "应用关闭事件" },
   { chapter: "05_events_lifecycle", number: 149, name: "lifecycle_prevent_exit",   title: "阻止应用退出" },
   { chapter: "05_events_lifecycle", number: 150, name: "lifecycle_window_close",   title: "窗口关闭控制" },
   { chapter: "05_events_lifecycle", number: 151, name: "lifecycle_background",     title: "应用后台运行" },
   { chapter: "05_events_lifecycle", number: 152, name: "lifecycle_single_instance", title: "单实例检查" },
   { chapter: "05_events_lifecycle", number: 153, name: "lifecycle_focus",          title: "应用唤醒/焦点事件" },
   { chapter: "05_events_lifecycle", number: 154, name: "lifecycle_sleep",          title: "屏幕休眠控制" },
   { chapter: "05_events_lifecycle", number: 155, name: "lifecycle_manager",        title: "综合: 应用生命周期管理器" },
   { chapter: "05_events_lifecycle", number: 156, name: "wevent_move",              title: "窗口移动事件" },
   { chapter: "05_events_lifecycle", number: 157, name: "wevent_resize",            title: "窗口大小变化事件" },
   { chapter: "05_events_lifecycle", number: 158, name: "wevent_focus",             title: "窗口焦点事件" },
   { chapter: "05_events_lifecycle", number: 159, name: "wevent_close",             title: "窗口关闭事件" },
   { chapter: "05_events_lifecycle", number: 160, name: "wevent_state_manager",     title: "综合: 窗口状态管理器" },
 
   // === 第 6 章: 前端集成 (06_frontend, e161-e200) ===
   { chapter: "06_frontend", number: 161, name: "vite_init",           title: "Vite + Vanilla TS 项目" },
   { chapter: "06_frontend", number: 162, name: "vite_config",         title: "Vite 配置与 Tauri 集成" },
   { chapter: "06_frontend", number: 163, name: "vite_env",            title: "环境变量管理" },
   { chapter: "06_frontend", number: 164, name: "vite_proxy",          title: "开发代理配置" },
   { chapter: "06_frontend", number: 165, name: "vite_css",            title: "CSS/SCSS 集成" },
   { chapter: "06_frontend", number: 166, name: "vite_typescript",     title: "TypeScript 类型安全" },
   { chapter: "06_frontend", number: 167, name: "vite_type_gen",       title: "类型生成工具" },
   { chapter: "06_frontend", number: 168, name: "vite_hmr",            title: "热更新体验" },
   { chapter: "06_frontend", number: 169, name: "vite_build_opt",      title: "前端构建优化" },
   { chapter: "06_frontend", number: 170, name: "vite_template",       title: "综合: Vite + TS 模板项目" },
   { chapter: "06_frontend", number: 171, name: "react_init",          title: "React + Tauri 初始化" },
   { chapter: "06_frontend", number: 172, name: "react_invoke",        title: "React 调用 Tauri 命令" },
   { chapter: "06_frontend", number: 173, name: "react_event_hook",    title: "React 事件监听 Hooks" },
   { chapter: "06_frontend", number: 174, name: "react_state_ipc",     title: "React 组件状态与 IPC" },
   { chapter: "06_frontend", number: 175, name: "react_router",        title: "React Router + Tauri" },
   { chapter: "06_frontend", number: 176, name: "react_window_mgr",    title: "React 窗口管理" },
   { chapter: "06_frontend", number: 177, name: "react_form",          title: "表单与 Tauri 命令" },
   { chapter: "06_frontend", number: 178, name: "react_dragdrop",      title: "文件拖放" },
   { chapter: "06_frontend", number: 179, name: "react_animation",     title: "动画与 Tauri 命令" },
   { chapter: "06_frontend", number: 180, name: "react_crud",          title: "综合: React + Tauri CRUD" },
   { chapter: "06_frontend", number: 181, name: "vue_init",            title: "Vue + Tauri 初始化" },
   { chapter: "06_frontend", number: 182, name: "vue_reactive",        title: "Vue 响应式状态与 Tauri" },
   { chapter: "06_frontend", number: 183, name: "vue_pinia",           title: "Pinia + Tauri 持久化" },
   { chapter: "06_frontend", number: 184, name: "svelte_init",         title: "Svelte + Tauri 初始化" },
   { chapter: "06_frontend", number: 185, name: "svelte_stores",       title: "Svelte stores + Tauri" },
   { chapter: "06_frontend", number: 186, name: "solid_init",          title: "Solid + Tauri 初始化" },
   { chapter: "06_frontend", number: 187, name: "framework_agnostic",  title: "框架无关原则" },
   { chapter: "06_frontend", number: 188, name: "state_persist_front", title: "前端状态持久化" },
   { chapter: "06_frontend", number: 189, name: "error_boundary",      title: "前端错误边界" },
   { chapter: "06_frontend", number: 190, name: "framework_compare",   title: "综合: 多框架对比示例" },
   { chapter: "06_frontend", number: 191, name: "static_assets",       title: "静态资源加载" },
   { chapter: "06_frontend", number: 192, name: "css_variables",       title: "自定义 CSS 变量和主题" },
   { chapter: "06_frontend", number: 193, name: "dark_mode",           title: "暗黑模式" },
   { chapter: "06_frontend", number: 194, name: "tailwind",            title: "Tailwind CSS 集成" },
   { chapter: "06_frontend", number: 195, name: "icon_system",         title: "图标系统" },
   { chapter: "06_frontend", number: 196, name: "font_loading",        title: "字体加载与管理" },
   { chapter: "06_frontend", number: 197, name: "responsive_layout",   title: "响应式布局" },
   { chapter: "06_frontend", number: 198, name: "i18n",                title: "多语言支持" },
   { chapter: "06_frontend", number: 199, name: "custom_titlebar",     title: "无边框窗口自定义标题栏" },
   { chapter: "06_frontend", number: 200, name: "context_menu",        title: "右键菜单" },
 
   // === 第 7 章: 插件系统 (07_plugins, e201-e270) ===
   { chapter: "07_plugins", number: 201, name: "fs_install",          title: "FS 插件安装与配置" },
   { chapter: "07_plugins", number: 202, name: "fs_read_file",        title: "读取文件内容" },
   { chapter: "07_plugins", number: 203, name: "fs_write_file",       title: "写入文件内容" },
   { chapter: "07_plugins", number: 204, name: "fs_directory",        title: "目录操作" },
   { chapter: "07_plugins", number: 205, name: "fs_metadata",         title: "文件元信息" },
   { chapter: "07_plugins", number: 206, name: "fs_copy_move",        title: "文件复制/移动/删除" },
   { chapter: "07_plugins", number: 207, name: "fs_exists",           title: "文件存在性检查" },
   { chapter: "07_plugins", number: 208, name: "fs_glob",             title: "文件通配符匹配" },
   { chapter: "07_plugins", number: 209, name: "fs_scope",            title: "文件系统权限范围" },
   { chapter: "07_plugins", number: 210, name: "fs_manager",          title: "综合: 文件管理器基础功能" },
   { chapter: "07_plugins", number: 211, name: "dialog_install",      title: "Dialog 插件安装与配置" },
   { chapter: "07_plugins", number: 212, name: "dialog_open",         title: "打开文件对话框" },
   { chapter: "07_plugins", number: 213, name: "dialog_save",         title: "保存文件对话框" },
   { chapter: "07_plugins", number: 214, name: "dialog_multi_file",   title: "多文件选择" },
   { chapter: "07_plugins", number: 215, name: "dialog_directory",    title: "目录选择器" },
   { chapter: "07_plugins", number: 216, name: "dialog_message",      title: "消息对话框" },
   { chapter: "07_plugins", number: 217, name: "dialog_confirm",      title: "确认对话框" },
   { chapter: "07_plugins", number: 218, name: "dialog_kind",         title: "自定义对话框样式" },
   { chapter: "07_plugins", number: 219, name: "dialog_result",       title: "对话框结果处理" },
   { chapter: "07_plugins", number: 220, name: "file_import_export",  title: "综合: 文件导入导出助手" },
   { chapter: "07_plugins", number: 221, name: "shell_install",       title: "Shell 插件安装与配置" },
   { chapter: "07_plugins", number: 222, name: "shell_execute",       title: "执行系统命令" },
   { chapter: "07_plugins", number: 223, name: "shell_args",          title: "命令参数" },
   { chapter: "07_plugins", number: 224, name: "shell_output",        title: "获取命令输出" },
   { chapter: "07_plugins", number: 225, name: "shell_async",         title: "异步执行与等待" },
   { chapter: "07_plugins", number: 226, name: "shell_timeout",       title: "命令超时控制" },
   { chapter: "07_plugins", number: 227, name: "shell_sidecar",       title: "Sidecar 模式" },
   { chapter: "07_plugins", number: 228, name: "shell_sidecar_ipc",   title: "Sidecar 参数与通信" },
   { chapter: "07_plugins", number: 229, name: "shell_nodejs",        title: "Node.js 作为 Sidecar" },
   { chapter: "07_plugins", number: 230, name: "command_palette",     title: "综合: 系统命令面板" },
   { chapter: "07_plugins", number: 231, name: "sql_install",         title: "SQL 插件安装与配置" },
   { chapter: "07_plugins", number: 232, name: "sql_create",          title: "创建数据库和数据表" },
   { chapter: "07_plugins", number: 233, name: "sql_insert",          title: "插入数据" },
   { chapter: "07_plugins", number: 234, name: "sql_query",           title: "查询数据" },
   { chapter: "07_plugins", number: 235, name: "sql_update_delete",   title: "更新与删除" },
   { chapter: "07_plugins", number: 236, name: "sql_transaction",     title: "事务处理" },
   { chapter: "07_plugins", number: 237, name: "sql_migration",       title: "迁移管理" },
   { chapter: "07_plugins", number: 238, name: "sql_join",            title: "关联查询" },
   { chapter: "07_plugins", number: 239, name: "sql_encryption",      title: "数据库加密" },
   { chapter: "07_plugins", number: 240, name: "sql_notes_app",       title: "综合: SQLite 笔记管理器" },
   { chapter: "07_plugins", number: 241, name: "store_install",       title: "Store 插件安装与配置" },
   { chapter: "07_plugins", number: 242, name: "store_kv",            title: "键值对读写" },
   { chapter: "07_plugins", number: 243, name: "store_persist",       title: "持久化存储" },
   { chapter: "07_plugins", number: 244, name: "store_watch",         title: "监听存储变化" },
   { chapter: "07_plugins", number: 245, name: "store_settings",      title: "综合: 应用设置持久化" },
   { chapter: "07_plugins", number: 246, name: "notify_send",         title: "发送系统通知" },
   { chapter: "07_plugins", number: 247, name: "notify_permission",   title: "通知权限请求" },
   { chapter: "07_plugins", number: 248, name: "notify_click",        title: "通知点击事件" },
   { chapter: "07_plugins", number: 249, name: "clipboard_rw",        title: "读写系统剪贴板" },
   { chapter: "07_plugins", number: 250, name: "clipboard_watch",     title: "剪贴板监听" },
   { chapter: "07_plugins", number: 251, name: "http_install",        title: "HTTP 插件安装与配置" },
   { chapter: "07_plugins", number: 252, name: "http_get",            title: "GET 请求" },
   { chapter: "07_plugins", number: 253, name: "http_post",           title: "POST 请求" },
   { chapter: "07_plugins", number: 254, name: "http_headers",        title: "请求头与认证" },
   { chapter: "07_plugins", number: 255, name: "http_upload",         title: "文件上传" },
   { chapter: "07_plugins", number: 256, name: "websocket_connect",   title: "WebSocket 连接" },
   { chapter: "07_plugins", number: 257, name: "websocket_reconnect", title: "WebSocket 重连" },
   { chapter: "07_plugins", number: 258, name: "network_status",      title: "网络状态检测" },
   { chapter: "07_plugins", number: 259, name: "http_timeout",        title: "HTTP 请求超时" },
   { chapter: "07_plugins", number: 260, name: "rest_client",         title: "综合: REST API 客户端" },
   { chapter: "07_plugins", number: 261, name: "plugin_os",           title: "OS 信息" },
   { chapter: "07_plugins", number: 262, name: "plugin_process",      title: "Process 管理" },
   { chapter: "07_plugins", number: 263, name: "plugin_shortcut",     title: "全局快捷键" },
   { chapter: "07_plugins", number: 264, name: "plugin_autostart",    title: "自启动" },
   { chapter: "07_plugins", number: 265, name: "plugin_logging",      title: "日志系统" },
   { chapter: "07_plugins", number: 266, name: "plugin_geolocation",  title: "位置服务" },
   { chapter: "07_plugins", number: 267, name: "plugin_deep_link",    title: "深度链接" },
   { chapter: "07_plugins", number: 268, name: "plugin_window_state", title: "窗口状态持久化" },
   { chapter: "07_plugins", number: 269, name: "plugin_opener",       title: "启动器" },
   { chapter: "07_plugins", number: 270, name: "plugin_multi",        title: "综合: 多插件协作" },
 
   // === 第 8 章: 窗口、菜单与托盘 (08_window_menu_tray, e271-e305) ===
   { chapter: "08_window_menu_tray", number: 271, name: "window_create",          title: "创建新窗口" },
   { chapter: "08_window_menu_tray", number: 272, name: "window_label",           title: "窗口标签与引用" },
   { chapter: "08_window_menu_tray", number: 273, name: "window_size",            title: "设置窗口大小" },
   { chapter: "08_window_menu_tray", number: 274, name: "window_position",        title: "设置窗口位置" },
   { chapter: "08_window_menu_tray", number: 275, name: "window_resize",          title: "窗口缩放控制" },
   { chapter: "08_window_menu_tray", number: 276, name: "window_title",           title: "窗口标题控制" },
   { chapter: "08_window_menu_tray", number: 277, name: "window_visibility",      title: "窗口可见性" },
   { chapter: "08_window_menu_tray", number: 278, name: "window_focus",           title: "窗口焦点控制" },
   { chapter: "08_window_menu_tray", number: 279, name: "window_fullscreen",      title: "全屏与最大化" },
   { chapter: "08_window_menu_tray", number: 280, name: "window_decorations",     title: "窗口装饰控制" },
   { chapter: "08_window_menu_tray", number: 281, name: "window_transparency",    title: "窗口透明度" },
   { chapter: "08_window_menu_tray", number: 282, name: "window_background",      title: "窗口背景色" },
   { chapter: "08_window_menu_tray", number: 283, name: "window_child",           title: "子窗口管理" },
   { chapter: "08_window_menu_tray", number: 284, name: "window_communication",   title: "窗口间通信" },
   { chapter: "08_window_menu_tray", number: 285, name: "window_multi_editor",    title: "综合: 多窗口编辑器" },
   { chapter: "08_window_menu_tray", number: 286, name: "menu_create",            title: "创建应用菜单" },
   { chapter: "08_window_menu_tray", number: 287, name: "menu_item_types",        title: "菜单项类型" },
   { chapter: "08_window_menu_tray", number: 288, name: "menu_separator",         title: "菜单分隔符" },
   { chapter: "08_window_menu_tray", number: 289, name: "menu_submenu",           title: "子菜单嵌套" },
   { chapter: "08_window_menu_tray", number: 290, name: "menu_events",            title: "菜单事件处理" },
   { chapter: "08_window_menu_tray", number: 291, name: "menu_shortcuts",         title: "快捷键绑定" },
   { chapter: "08_window_menu_tray", number: 292, name: "menu_state",             title: "菜单状态控制" },
   { chapter: "08_window_menu_tray", number: 293, name: "menu_platform",          title: "平台特殊菜单" },
   { chapter: "08_window_menu_tray", number: 294, name: "menu_predefined",        title: "预定义菜单项" },
   { chapter: "08_window_menu_tray", number: 295, name: "menu_dynamic",           title: "动态菜单" },
   { chapter: "08_window_menu_tray", number: 296, name: "menu_context",           title: "上下文菜单(右键)" },
   { chapter: "08_window_menu_tray", number: 297, name: "menu_window_assoc",      title: "菜单与窗口关联" },
   { chapter: "08_window_menu_tray", number: 298, name: "menu_i18n",              title: "菜单国际化" },
   { chapter: "08_window_menu_tray", number: 299, name: "menu_icons",             title: "菜单图标" },
   { chapter: "08_window_menu_tray", number: 300, name: "full_menu_system",       title: "综合: 完整菜单系统" },
   { chapter: "08_window_menu_tray", number: 301, name: "tray_create",            title: "创建系统托盘" },
   { chapter: "08_window_menu_tray", number: 302, name: "tray_icon",              title: "系统托盘图标" },
   { chapter: "08_window_menu_tray", number: 303, name: "tray_menu",              title: "系统托盘菜单" },
   { chapter: "08_window_menu_tray", number: 304, name: "tray_click_event",       title: "托盘点击事件" },
   { chapter: "08_window_menu_tray", number: 305, name: "tray_tooltip",           title: "托盘提示文字" },
 
   // === 第 9 章: 验证与测试 (09_testing, e306-e320) ===
   { chapter: "09_testing", number: 306, name: "test_unit",               title: "Rust 单元测试" },
   { chapter: "09_testing", number: 307, name: "test_integration",        title: "命令集成测试" },
   { chapter: "09_testing", number: 308, name: "test_webdriver",          title: "WebDriver E2E 测试" },
   { chapter: "09_testing", number: 309, name: "test_e2e_case",           title: "E2E 测试用例编写" },
   { chapter: "09_testing", number: 310, name: "test_frontend_unit",      title: "前端单元测试" },
   { chapter: "09_testing", number: 311, name: "test_frontend_component", title: "前端组件测试" },
   { chapter: "09_testing", number: 312, name: "test_ci",                 title: "持续集成配置" },
   { chapter: "09_testing", number: 313, name: "test_cross_platform",     title: "跨平台测试策略" },
   { chapter: "09_testing", number: 314, name: "test_benchmark",          title: "性能基准测试" },
   { chapter: "09_testing", number: 315, name: "test_pipeline",           title: "综合: 自动化测试流水线" },
   { chapter: "09_testing", number: 316, name: "build_optimize",          title: "构建配置优化" },
   { chapter: "09_testing", number: 317, name: "build_windows",           title: "Windows 打包(NSIS/MSI)" },
   { chapter: "09_testing", number: 318, name: "build_macos",             title: "macOS 打包(DMG/AppBundle)" },
   { chapter: "09_testing", number: 319, name: "build_linux",             title: "Linux 打包(AppImage/deb)" },
   { chapter: "09_testing", number: 320, name: "build_icons_assets",      title: "应用图标与资源" },
 
   // === 第 10 章: 高级主题 (10_advanced, e321-e355) ===
   { chapter: "10_advanced", number: 321, name: "security_arch",          title: "Tauri 安全架构" },
   { chapter: "10_advanced", number: 322, name: "security_permissions",   title: "权限系统基础" },
   { chapter: "10_advanced", number: 323, name: "security_capabilities",  title: "Capabilities 配置" },
   { chapter: "10_advanced", number: 324, name: "security_custom_perms",  title: "自定义命令权限" },
   { chapter: "10_advanced", number: 325, name: "security_platform",      title: "平台特定权限" },
   { chapter: "10_advanced", number: 326, name: "security_scope",         title: "Scope 系统" },
   { chapter: "10_advanced", number: 327, name: "security_remote_access", title: "远程 API 访问控制" },
   { chapter: "10_advanced", number: 328, name: "security_csp",           title: "内容安全策略 CSP" },
   { chapter: "10_advanced", number: 329, name: "security_isolation",     title: "隔离模式" },
   { chapter: "10_advanced", number: 330, name: "security_coding",        title: "安全意识编码实践" },
   { chapter: "10_advanced", number: 331, name: "security_sensitive_data",title: "敏感数据处理" },
   { chapter: "10_advanced", number: 332, name: "security_updates",       title: "安全更新机制" },
   { chapter: "10_advanced", number: 333, name: "security_audit_log",     title: "审计和日志" },
   { chapter: "10_advanced", number: 334, name: "security_dependencies",  title: "第三方依赖安全" },
   { chapter: "10_advanced", number: 335, name: "security_review_tool",   title: "综合: 安全审查工具" },
   { chapter: "10_advanced", number: 336, name: "publish_update",         title: "自动更新配置" },
   { chapter: "10_advanced", number: 337, name: "publish_codesign",       title: "代码签名" },
   { chapter: "10_advanced", number: 338, name: "publish_size_optimize",  title: "应用大小优化" },
   { chapter: "10_advanced", number: 339, name: "publish_static_build",   title: "静态编译与运行时依赖" },
   { chapter: "10_advanced", number: 340, name: "publish_pipeline",       title: "综合: 完整发布流水线" },
   { chapter: "10_advanced", number: 341, name: "mobile_setup",           title: "移动端开发环境配置" },
   { chapter: "10_advanced", number: 342, name: "mobile_android",         title: "Android 项目结构" },
   { chapter: "10_advanced", number: 343, name: "mobile_ios",             title: "iOS 项目结构" },
   { chapter: "10_advanced", number: 344, name: "mobile_config",          title: "移动端 Tauri 配置" },
   { chapter: "10_advanced", number: 345, name: "mobile_commands",        title: "移动端命令开发" },
   { chapter: "10_advanced", number: 346, name: "mobile_permissions",     title: "移动端权限处理" },
   { chapter: "10_advanced", number: 347, name: "mobile_ui",              title: "移动端适配 UI" },
   { chapter: "10_advanced", number: 348, name: "mobile_plugins",         title: "移动端特有插件" },
   { chapter: "10_advanced", number: 349, name: "mobile_android_build",   title: "Android 构建与签名" },
   { chapter: "10_advanced", number: 350, name: "mobile_ios_build",       title: "iOS 构建与 App Store" },
   { chapter: "10_advanced", number: 351, name: "tray_animate",           title: "托盘动画图标" },
   { chapter: "10_advanced", number: 352, name: "tray_hide_to",           title: "窗口隐藏到托盘" },
   { chapter: "10_advanced", number: 353, name: "tray_restore",           title: "托盘恢复窗口" },
   { chapter: "10_advanced", number: 354, name: "tray_platform_diff",     title: "平台特定托盘行为" },
   { chapter: "10_advanced", number: 355, name: "tray_app",               title: "综合: 托盘应用" },
 ];
 
 // ====================================================================
 // 辅助函数
 // ====================================================================
 
 function padNumber(num, width) {
   return String(num).padStart(width, "0");
 }
 
 function createExercise(ex) {
   const numStr = padNumber(ex.number, 3);
   const devPort = 1420 + ex.number;
   const chapterDir = path.join(PROJECT_ROOT, ex.chapter);
 
   // 确保章节目录存在
   if (!fs.existsSync(chapterDir)) {
     fs.mkdirSync(chapterDir, { recursive: true });
   }
 
   const pairs = [
     { suffix: "", dirName: `e${numStr}_${ex.name}` },
     { suffix: "_answer", dirName: `e${numStr}_${ex.name}_answer` },
   ];
 
   for (const pair of pairs) {
     const targetDir = path.join(chapterDir, pair.dirName);
     if (fs.existsSync(targetDir)) {
       console.log(`  · 跳过 ${pair.dirName} (已存在)`);
       continue;
     }
 
     // 复制模板
     copyTemplate(TEMPLATE_DIR, targetDir);
 
     // 更新文件
     updateProject(targetDir, ex, numStr, devPort, pair.suffix);
 
     // 更新 workspace members
     addWorkspaceMember(ex.chapter, pair.dirName);
 
     console.log(`  ✓ ${pair.dirName}`);
   }
 }
 
 function copyTemplate(src, dest) {
   function copyRecursive(s, d) {
     if (!fs.existsSync(d)) {
       fs.mkdirSync(d, { recursive: true });
     }
     const entries = fs.readdirSync(s, { withFileTypes: true });
     for (const entry of entries) {
       const srcPath = path.join(s, entry.name);
       const destPath = path.join(d, entry.name);
       if (entry.isDirectory()) {
         if (entry.name !== "node_modules") {
           copyRecursive(srcPath, destPath);
         }
       } else {
         fs.copyFileSync(srcPath, destPath);
       }
     }
   }
   copyRecursive(src, dest);
 }
 
 function updateProject(targetDir, ex, numStr, devPort, suffix) {
   const libName = `e${numStr}_${ex.name}${suffix}`.replace(/-/g, "_");
   const packageName = `e${numStr}_${ex.name}${suffix}`;
   const identifier = `com.taurilearn.e${numStr}${suffix}`;
   const windowTitle = `练习 ${numStr}: ${ex.title}${suffix === "_answer" ? " (答案)" : ""}`;
 
   // 1. package.json
   const pkgPath = path.join(targetDir, "package.json");
   if (fs.existsSync(pkgPath)) {
     let pkg = JSON.parse(fs.readFileSync(pkgPath, "utf8"));
     pkg.name = packageName;
     fs.writeFileSync(pkgPath, JSON.stringify(pkg, null, 2) + "\n");
   }
 
   // 2. Cargo.toml
   const cargoPath = path.join(targetDir, "src-tauri", "Cargo.toml");
   if (fs.existsSync(cargoPath)) {
     let content = fs.readFileSync(cargoPath, "utf8");
     content = content.replace(/exercise-template-vite-ts/g, packageName);
     content = content.replace(/exercise_template_vite_ts_lib/g, `${libName}_lib`);
     fs.writeFileSync(cargoPath, content);
   }
 
   // 3. tauri.conf.json
   const confPath = path.join(targetDir, "src-tauri", "tauri.conf.json");
   if (fs.existsSync(confPath)) {
     let conf = JSON.parse(fs.readFileSync(confPath, "utf8"));
     conf.productName = packageName;
     conf.identifier = identifier;
     conf.build.devUrl = `http://localhost:${devPort}`;
     conf.app.windows[0].title = windowTitle;
     fs.writeFileSync(confPath, JSON.stringify(conf, null, 2) + "\n");
   }
 
   // 4. index.html title
   const htmlPath = path.join(targetDir, "index.html");
   if (fs.existsSync(htmlPath)) {
     let html = fs.readFileSync(htmlPath, "utf8");
     html = html.replace(/<title>.*<\/title>/, `<title>${windowTitle}</title>`);
     fs.writeFileSync(htmlPath, html);
   }
 }
 
 function addWorkspaceMember(chapter, dirName) {
   const wsPath = path.join(PROJECT_ROOT, "Cargo.toml");
   const memberPath = `${chapter}/${dirName}/src-tauri`;
   let content = fs.readFileSync(wsPath, "utf8");
   if (!content.includes(memberPath)) {
     content = content.replace(
       /(members\s*=\s*\[)/,
       `$1\n    "${memberPath}",`
     );
     fs.writeFileSync(wsPath, content);
   }
 }
 
 // ====================================================================
 // 主流程
 // ====================================================================
 
 console.log("========================================");
 console.log("Tauri v2 练习项目批量初始化");
 console.log(`共 ${ALL_EXERCISES.length} 道练习，将创建 ${ALL_EXERCISES.length * 2} 个项目目录`);
 console.log("========================================");
 console.log("");
 
 const startTime = Date.now();
 let completed = 0;
 
 for (const ex of ALL_EXERCISES) {
   createExercise(ex);
   completed++;
   if (completed % 10 === 0) {
     const elapsed = Math.round((Date.now() - startTime) / 1000);
     console.log(`  [进度: ${completed}/${ALL_EXERCISES.length}, 已用: ${elapsed}s]`);
   }
 }
 
 const totalElapsed = Math.round((Date.now() - startTime) / 1000);
 console.log("");
 console.log("========================================");
 console.log("批量初始化完成!");
 console.log(`总练习数: ${ALL_EXERCISES.length}`);
 console.log(`总项目数: ${ALL_EXERCISES.length * 2} (练习 + 答案)`);
 console.log(`成功: ${completed}`);
 console.log(`总耗时: ${totalElapsed}s`);
 console.log("========================================");
 console.log("");
 console.log("下一步: 使用编写 Agent 批量填充各练习的 TODO 和参考答案代码");
