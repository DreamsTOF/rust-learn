 <#
 .SYNOPSIS
     批量初始化所有 Tauri v2 练习项目
 
 .DESCRIPTION
     根据 tauri-learn-plan.md 的定义，批量创建全部 355 道练习和答案项目
     每道题同时创建 exercise/ 和 exercise_answer/ 两个项目文件夹
 #>
 
 $ProjectRoot = Split-Path -Parent $PSScriptRoot
 $TemplateDir = Join-Path $ProjectRoot "templates\vite-ts"
 $StartTime = Get-Date
 
 # 验证模板存在
 if (-not (Test-Path $TemplateDir)) {
     Write-Error "模板目录不存在: $TemplateDir"
     exit 1
 }
 
 # ====================================================================
 # 练习定义
 # 格式: @{Chapter="章节目录"; Number=编号; Name="英文名"; Title="中文标题"}
 # ====================================================================
 $AllExercises = @(
     # ========== 第 1 章：环境准备 (01_environment, e01-e10) ==========
     @{Chapter="01_environment"; Number=1;  Name="check_rust";        Title="Rust 环境检查"},
     @{Chapter="01_environment"; Number=2;  Name="check_node";        Title="Node.js 环境检查"},
     @{Chapter="01_environment"; Number=3;  Name="check_webview";     Title="WebView2 环境检查"},
     @{Chapter="01_environment"; Number=4;  Name="install_cli";       Title="安装 Tauri CLI"},
     @{Chapter="01_environment"; Number=5;  Name="create_first_app";  Title="创建第一个 Tauri 应用"},
     @{Chapter="01_environment"; Number=6;  Name="project_structure"; Title="项目结构详解"},
     @{Chapter="01_environment"; Number=7;  Name="dev_server";       Title="开发服务器与热更新"},
     @{Chapter="01_environment"; Number=8;  Name="build_app";        Title="构建与产物分析"},
     @{Chapter="01_environment"; Number=9;  Name="cross_platform";   Title="跨平台开发注意事项"},
     @{Chapter="01_environment"; Number=10; Name="env_summary";      Title="综合：环境诊断工具"},
 
     # ========== 第 2 章：基础入门 (02_basics, e11-e30) ==========
     @{Chapter="02_basics"; Number=11; Name="hello_world";          Title="Hello World"},
     @{Chapter="02_basics"; Number=12; Name="html_view";            Title="HTML 视图与文本节点"},
     @{Chapter="02_basics"; Number=13; Name="html_elements";        Title="HTML 元素与属性"},
     @{Chapter="02_basics"; Number=14; Name="element_nesting";      Title="元素嵌套与层级结构"},
     @{Chapter="02_basics"; Number=15; Name="first_command";        Title="第一个 Rust 命令"},
     @{Chapter="02_basics"; Number=16; Name="frontend_invoke";      Title="从前端调用命令"},
     @{Chapter="02_basics"; Number=17; Name="config_tauri";         Title="tauri.conf.json 配置详解"},
     @{Chapter="02_basics"; Number=18; Name="dev_vs_prod";          Title="开发模式 vs 生产模式"},
     @{Chapter="02_basics"; Number=19; Name="build_commands";       Title="构建命令与 Vite 集成"},
     @{Chapter="02_basics"; Number=20; Name="window_properties";    Title="窗口基础属性配置"},
     @{Chapter="02_basics"; Number=21; Name="multi_window";         Title="多窗口配置"},
     @{Chapter="02_basics"; Number=22; Name="window_size";          Title="窗口大小与约束"},
     @{Chapter="02_basics"; Number=23; Name="window_position";      Title="窗口位置与居中"},
     @{Chapter="02_basics"; Number=24; Name="fullscreen";           Title="窗口全屏与最大化"},
     @{Chapter="02_basics"; Number=25; Name="cargo_tauri_cli";      Title="使用 cargo tauri CLI"},
     @{Chapter="02_basics"; Number=26; Name="web_inspector";        Title="调试：Web Inspector"},
     @{Chapter="02_basics"; Number=27; Name="rust_logging";         Title="调试：Rust 日志"},
     @{Chapter="02_basics"; Number=28; Name="csp_security";         Title="理解 CSP 内容安全策略"},
     @{Chapter="02_basics"; Number=29; Name="app_icons";            Title="应用图标与资源"},
     @{Chapter="02_basics"; Number=30; Name="tauri_vs_browser";     Title="浏览器与 Tauri 环境区分"},
 
     # ========== 第 3 章：命令与 IPC (03_commands_ipc, e31-e95) ==========
     # 基本命令 (e31-e40)
     @{Chapter="03_commands_ipc"; Number=31; Name="cmd_no_args";       Title="无参无返回值命令"},
     @{Chapter="03_commands_ipc"; Number=32; Name="cmd_string";        Title="字符串参数"},
     @{Chapter="03_commands_ipc"; Number=33; Name="cmd_number";        Title="数字参数"},
     @{Chapter="03_commands_ipc"; Number=34; Name="cmd_boolean";       Title="布尔参数"},
     @{Chapter="03_commands_ipc"; Number=35; Name="cmd_return_string"; Title="返回字符串"},
     @{Chapter="03_commands_ipc"; Number=36; Name="cmd_return_number"; Title="返回数字"},
     @{Chapter="03_commands_ipc"; Number=37; Name="cmd_return_bool";   Title="返回布尔值"},
     @{Chapter="03_commands_ipc"; Number=38; Name="cmd_return_struct"; Title="返回结构体"},
     @{Chapter="03_commands_ipc"; Number=39; Name="cmd_return_vec";    Title="返回 Vec 数组"},
     @{Chapter="03_commands_ipc"; Number=40; Name="cmd_return_option"; Title="返回 Option"},
     # 复合参数 (e41-e45)
     @{Chapter="03_commands_ipc"; Number=41; Name="cmd_struct_input";  Title="复合参数：结构体作为输入"},
     @{Chapter="03_commands_ipc"; Number=42; Name="cmd_optional_args"; Title="参数默认值与可选参数"},
     @{Chapter="03_commands_ipc"; Number=43; Name="cmd_mixed_args";    Title="多个参数混合"},
     @{Chapter="03_commands_ipc"; Number=44; Name="cmd_naming";        Title="命令命名规范"},
     @{Chapter="03_commands_ipc"; Number=45; Name="cmd_modular";       Title="模块化组织命令"},
     # 异步命令 (e46-e55)
     @{Chapter="03_commands_ipc"; Number=46; Name="async_cmd_basic";      Title="基础异步命令"},
     @{Chapter="03_commands_ipc"; Number=47; Name="async_read_file";      Title="异步文件读取"},
     @{Chapter="03_commands_ipc"; Number=48; Name="async_http_request";   Title="异步 HTTP 请求"},
     @{Chapter="03_commands_ipc"; Number=49; Name="async_db_query";       Title="异步数据库查询"},
     @{Chapter="03_commands_ipc"; Number=50; Name="async_timeout";        Title="异步超时控制"},
     @{Chapter="03_commands_ipc"; Number=51; Name="async_parallel";       Title="异步任务并行"},
     @{Chapter="03_commands_ipc"; Number=52; Name="async_app_handle";    Title="异步中访问 AppHandle"},
     @{Chapter="03_commands_ipc"; Number=53; Name="async_window_access"; Title="异步中访问窗口"},
     @{Chapter="03_commands_ipc"; Number=54; Name="async_borrow";        Title="异步中借用问题"},
     @{Chapter="03_commands_ipc"; Number=55; Name="async_error";         Title="异步命令错误处理"},
     # 异步高级 (e56-e60)
     @{Chapter="03_commands_ipc"; Number=56; Name="long_running_task";    Title="长时间运行任务"},
     @{Chapter="03_commands_ipc"; Number=57; Name="spawn_concurrency";    Title="使用 spawn 管理并发"},
     @{Chapter="03_commands_ipc"; Number=58; Name="cancel_task";          Title="取消长时间运行任务"},
     @{Chapter="03_commands_ipc"; Number=59; Name="rate_limiting";        Title="限流与节流"},
     @{Chapter="03_commands_ipc"; Number=60; Name="file_hash_calculator"; Title="综合：文件哈希计算器"},
     # 依赖注入 (e61-e70)
     @{Chapter="03_commands_ipc"; Number=61; Name="di_app_handle";      Title="访问 AppHandle"},
     @{Chapter="03_commands_ipc"; Number=62; Name="di_webview_window";  Title="访问 WebviewWindow"},
     @{Chapter="03_commands_ipc"; Number=63; Name="di_webview";         Title="访问 Webview"},
     @{Chapter="03_commands_ipc"; Number=64; Name="di_app";             Title="访问 App"},
     @{Chapter="03_commands_ipc"; Number=65; Name="di_managed_state";   Title="注入 Managed State"},
     @{Chapter="03_commands_ipc"; Number=66; Name="di_multiple";        Title="同时注入多个依赖"},
     @{Chapter="03_commands_ipc"; Number=67; Name="di_order";           Title="依赖注入顺序"},
     @{Chapter="03_commands_ipc"; Number=68; Name="di_window_label";    Title="访问窗口标签"},
     @{Chapter="03_commands_ipc"; Number=69; Name="di_resize_window";   Title="操作窗口 resize/move"},
     @{Chapter="03_commands_ipc"; Number=70; Name="di_minimize_window"; Title="操作窗口 minimize/maximize"},
     # 依赖注入高级 (e71-e75)
     @{Chapter="03_commands_ipc"; Number=71; Name="di_window_state";    Title="获取窗口状态"},
     @{Chapter="03_commands_ipc"; Number=72; Name="di_multi_window";    Title="多窗口间通信"},
     @{Chapter="03_commands_ipc"; Number=73; Name="di_custom_setup";    Title="自定义依赖注入"},
     @{Chapter="03_commands_ipc"; Number=74; Name="di_testing";         Title="依赖注入与测试"},
     @{Chapter="03_commands_ipc"; Number=75; Name="window_manager";     Title="综合：窗口管理器"},
     # 错误处理 (e76-e85)
     @{Chapter="03_commands_ipc"; Number=76; Name="error_result_basic";    Title="Result 基础"},
     @{Chapter="03_commands_ipc"; Number=77; Name="error_string";          Title="字符串错误"},
     @{Chapter="03_commands_ipc"; Number=78; Name="error_custom_enum";     Title="自定义错误枚举"},
     @{Chapter="03_commands_ipc"; Number=79; Name="error_code_message";    Title="错误码与错误消息"},
     @{Chapter="03_commands_ipc"; Number=80; Name="error_conversion";      Title="错误类型转换"},
     @{Chapter="03_commands_ipc"; Number=81; Name="error_from_trait";      Title="From trait 实现"},
     @{Chapter="03_commands_ipc"; Number=82; Name="error_chain";           Title="错误链"},
     @{Chapter="03_commands_ipc"; Number=83; Name="error_propagation";     Title="错误在命令链中传播"},
     @{Chapter="03_commands_ipc"; Number=84; Name="error_frontend";        Title="前端错误统一处理"},
     @{Chapter="03_commands_ipc"; Number=85; Name="file_read_error";       Title="综合：文件读取与错误处理"},
     # 高级 IPC (e86-e95)
     @{Chapter="03_commands_ipc"; Number=86; Name="ipc_batch_request";  Title="批量请求与去重"},
     @{Chapter="03_commands_ipc"; Number=87; Name="ipc_retry";          Title="命令重试机制"},
     @{Chapter="03_commands_ipc"; Number=88; Name="ipc_timeout";        Title="请求超时"},
     @{Chapter="03_commands_ipc"; Number=89; Name="ipc_cancel";         Title="请求取消"},
     @{Chapter="03_commands_ipc"; Number=90; Name="ipc_benchmark";      Title="IPC 性能基准测试"},
     @{Chapter="03_commands_ipc"; Number=91; Name="ipc_large_file";     Title="大文件传输优化"},
     @{Chapter="03_commands_ipc"; Number=92; Name="ipc_binary_data";    Title="二进制数据传输"},
     @{Chapter="03_commands_ipc"; Number=93; Name="ipc_channel";        Title="使用 Channel 流式传输"},
     @{Chapter="03_commands_ipc"; Number=94; Name="ipc_channel_progress"; Title="Channel 进度通知"},
     @{Chapter="03_commands_ipc"; Number=95; Name="ipc_channel_vs_event"; Title="Channel vs 事件对比"},
 
     # ========== 第 4 章：状态管理与配置 (04_state_config, e96-e130) ==========
     @{Chapter="04_state_config"; Number=96;  Name="state_register";       Title="注册 Managed State"},
     @{Chapter="04_state_config"; Number=97;  Name="state_read";           Title="在命令中读取 State"},
     @{Chapter="04_state_config"; Number=98;  Name="state_mutex";          Title="可变 State Mutex"},
     @{Chapter="04_state_config"; Number=99;  Name="state_rwlock";         Title="可变 State RwLock"},
     @{Chapter="04_state_config"; Number=100; Name="state_lifetime";       Title="State 生命周期"},
     @{Chapter="04_state_config"; Number=101; Name="state_multiple";       Title="多 State 管理"},
     @{Chapter="04_state_config"; Number=102; Name="state_complex";        Title="State 中的复杂类型"},
     @{Chapter="04_state_config"; Number=103; Name="state_async_init";     Title="State 初始化中执行异步"},
     @{Chapter="04_state_config"; Number=104; Name="state_hot_update";     Title="State 的热更新"},
     @{Chapter="04_state_config"; Number=105; Name="state_thread_safe";    Title="线程安全的 State"},
     @{Chapter="04_state_config"; Number=106; Name="state_plugin_interop"; Title="State 与插件交互"},
     @{Chapter="04_state_config"; Number=107; Name="state_default";        Title="State 的默认值"},
     @{Chapter="04_state_config"; Number=108; Name="state_testing";        Title="State 测试"},
     @{Chapter="04_state_config"; Number=109; Name="state_once_cell";      Title="使用 OnceCell / OnceLock"},
     @{Chapter="04_state_config"; Number=110; Name="config_manager";       Title="综合：应用配置管理器"},
     # 配置管理 (e111-e120)
     @{Chapter="04_state_config"; Number=111; Name="config_read";          Title="读取 tauri.conf.json"},
     @{Chapter="04_state_config"; Number=112; Name="config_custom";        Title="自定义配置字段"},
     @{Chapter="04_state_config"; Number=113; Name="config_deserialize";   Title="配置反序列化"},
     @{Chapter="04_state_config"; Number=114; Name="config_platform";      Title="平台特定配置"},
     @{Chapter="04_state_config"; Number=115; Name="config_env";           Title="环境变量覆盖配置"},
     @{Chapter="04_state_config"; Number=116; Name="config_runtime";       Title="运行时修改配置"},
     @{Chapter="04_state_config"; Number=117; Name="config_validation";    Title="配置文件验证"},
     @{Chapter="04_state_config"; Number=118; Name="config_custom_path";   Title="自定义配置文件路径"},
     @{Chapter="04_state_config"; Number=119; Name="config_hot_reload";    Title="配置热重载"},
     @{Chapter="04_state_config"; Number=120; Name="multi_env_config";     Title="综合：多环境配置管理器"},
     # 资源路径 (e121-e130)
     @{Chapter="04_state_config"; Number=121; Name="path_embed_resource";  Title="嵌入资源文件"},
     @{Chapter="04_state_config"; Number=122; Name="path_resource_dir";    Title="读取应用资源目录"},
     @{Chapter="04_state_config"; Number=123; Name="path_app_data";        Title="读取应用数据目录"},
     @{Chapter="04_state_config"; Number=124; Name="path_app_config";      Title="读取应用配置目录"},
     @{Chapter="04_state_config"; Number=125; Name="path_documents";       Title="读取文档目录"},
     @{Chapter="04_state_config"; Number=126; Name="path_downloads";       Title="读取下载目录"},
     @{Chapter="04_state_config"; Number=127; Name="path_desktop";         Title="读取桌面目录"},
     @{Chapter="04_state_config"; Number=128; Name="path_cross_platform";  Title="路径解析最佳实践"},
     @{Chapter="04_state_config"; Number=129; Name="path_temp";            Title="临时文件与目录"},
     @{Chapter="04_state_config"; Number=130; Name="path_cache";           Title="缓存目录与日志目录"},
 
     # ========== 第 5 章：事件与生命周期 (05_events_lifecycle, e131-e160) ==========
     @{Chapter="05_events_lifecycle"; Number=131; Name="event_emit_backend";     Title="后端发送全局事件"},
     @{Chapter="05_events_lifecycle"; Number=132; Name="event_listen_backend";   Title="后端监听事件"},
     @{Chapter="05_events_lifecycle"; Number=133; Name="event_global_vs_webview"; Title="全局事件 vs Webview 事件"},
     @{Chapter="05_events_lifecycle"; Number=134; Name="event_frontend_listen";  Title="前端监听后端事件"},
     @{Chapter="05_events_lifecycle"; Number=135; Name="event_frontend_emit";    Title="前端发送事件到后端"},
     @{Chapter="05_events_lifecycle"; Number=136; Name="event_unlisten";         Title="事件取消监听"},
     @{Chapter="05_events_lifecycle"; Number=137; Name="event_once";             Title="一次性监听"},
     @{Chapter="05_events_lifecycle"; Number=138; Name="event_payload";          Title="事件负载类型"},
     @{Chapter="05_events_lifecycle"; Number=139; Name="event_throttle";         Title="事件频率控制"},
     @{Chapter="05_events_lifecycle"; Number=140; Name="event_dedup";            Title="事件去重"},
     @{Chapter="05_events_lifecycle"; Number=141; Name="event_timeout";          Title="事件超时"},
     @{Chapter="05_events_lifecycle"; Number=142; Name="event_naming";           Title="事件命名规范"},
     @{Chapter="05_events_lifecycle"; Number=143; Name="event_debug";            Title="事件调试"},
     @{Chapter="05_events_lifecycle"; Number=144; Name="event_security";         Title="事件安全"},
     @{Chapter="05_events_lifecycle"; Number=145; Name="event_bus";              Title="综合：事件总线"},
     # 生命周期 (e146-e155)
     @{Chapter="05_events_lifecycle"; Number=146; Name="lifecycle_setup";          Title="Setup 钩子"},
     @{Chapter="05_events_lifecycle"; Number=147; Name="lifecycle_main_thread";    Title="RunOnMainThread"},
     @{Chapter="05_events_lifecycle"; Number=148; Name="lifecycle_exit_event";     Title="应用关闭事件"},
     @{Chapter="05_events_lifecycle"; Number=149; Name="lifecycle_prevent_exit";   Title="阻止应用退出"},
     @{Chapter="05_events_lifecycle"; Number=150; Name="lifecycle_window_close";   Title="窗口关闭控制"},
     @{Chapter="05_events_lifecycle"; Number=151; Name="lifecycle_background";     Title="应用后台运行"},
     @{Chapter="05_events_lifecycle"; Number=152; Name="lifecycle_single_instance"; Title="单实例检查"},
     @{Chapter="05_events_lifecycle"; Number=153; Name="lifecycle_focus";          Title="应用唤醒/焦点事件"},
     @{Chapter="05_events_lifecycle"; Number=154; Name="lifecycle_sleep";          Title="屏幕休眠控制"},
     @{Chapter="05_events_lifecycle"; Number=155; Name="lifecycle_manager";        Title="综合：应用生命周期管理器"},
     # 窗口事件 (e156-e160)
     @{Chapter="05_events_lifecycle"; Number=156; Name="wevent_move";        Title="窗口移动事件"},
     @{Chapter="05_events_lifecycle"; Number=157; Name="wevent_resize";      Title="窗口大小变化事件"},
     @{Chapter="05_events_lifecycle"; Number=158; Name="wevent_focus";       Title="窗口焦点事件"},
     @{Chapter="05_events_lifecycle"; Number=159; Name="wevent_close";       Title="窗口关闭事件"},
     @{Chapter="05_events_lifecycle"; Number=160; Name="wevent_state";       Title="综合：窗口状态管理器"},
 
     # ========== 第 6 章：前端集成 (06_frontend, e161-e200) ==========
     # Vite 集成 (e161-e170)
     @{Chapter="06_frontend"; Number=161; Name="vite_init";            Title="Vite + Vanilla TS 项目"},
     @{Chapter="06_frontend"; Number=162; Name="vite_config";          Title="Vite 配置与 Tauri 集成"},
     @{Chapter="06_frontend"; Number=163; Name="vite_env";             Title="环境变量管理"},
     @{Chapter="06_frontend"; Number=164; Name="vite_proxy";           Title="开发代理配置"},
     @{Chapter="06_frontend"; Number=165; Name="vite_css";             Title="CSS/SCSS 集成"},
     @{Chapter="06_frontend"; Number=166; Name="vite_typescript";      Title="TypeScript 类型安全"},
     @{Chapter="06_frontend"; Number=167; Name="vite_type_gen";        Title="类型生成工具"},
     @{Chapter="06_frontend"; Number=168; Name="vite_hmr";             Title="热更新体验"},
     @{Chapter="06_frontend"; Number=169; Name="vite_build_opt";       Title="前端构建优化"},
     @{Chapter="06_frontend"; Number=170; Name="vite_template";        Title="综合：Vite + TS 模板项目"},
     # React 集成 (e171-e180)
     @{Chapter="06_frontend"; Number=171; Name="react_init";           Title="React + Tauri 初始化"},
     @{Chapter="06_frontend"; Number=172; Name="react_invoke";         Title="React 调用 Tauri 命令"},
     @{Chapter="06_frontend"; Number=173; Name="react_event_hook";     Title="React 事件监听 Hooks"},
     @{Chapter="06_frontend"; Number=174; Name="react_state_ipc";      Title="React 组件状态与 IPC"},
     @{Chapter="06_frontend"; Number=175; Name="react_router";         Title="React Router + Tauri"},
     @{Chapter="06_frontend"; Number=176; Name="react_window";         Title="React 窗口管理"},
     @{Chapter="06_frontend"; Number=177; Name="react_form";           Title="表单与 Tauri 命令"},
     @{Chapter="06_frontend"; Number=178; Name="react_dragdrop";       Title="文件拖放"},
     @{Chapter="06_frontend"; Number=179; Name="react_animation";      Title="动画与 Tauri 命令"},
     @{Chapter="06_frontend"; Number=180; Name="react_crud";           Title="综合：React + Tauri CRUD"},
     # Vue/Svelte (e181-e190)
     @{Chapter="06_frontend"; Number=181; Name="vue_init";             Title="Vue + Tauri 初始化"},
     @{Chapter="06_frontend"; Number=182; Name="vue_reactive";         Title="Vue 响应式状态与 Tauri"},
     @{Chapter="06_frontend"; Number=183; Name="vue_pinia";            Title="Pinia + Tauri 持久化"},
     @{Chapter="06_frontend"; Number=184; Name="svelte_init";          Title="Svelte + Tauri 初始化"},
     @{Chapter="06_frontend"; Number=185; Name="svelte_store";         Title="Svelte stores + Tauri"},
     @{Chapter="06_frontend"; Number=186; Name="solid_init";           Title="Solid + Tauri 初始化"},
     @{Chapter="06_frontend"; Number=187; Name="framework_agnostic";   Title="框架无关原则"},
     @{Chapter="06_frontend"; Number=188; Name="state_persist";        Title="前端状态持久化"},
     @{Chapter="06_frontend"; Number=189; Name="error_boundary";       Title="前端错误边界"},
     @{Chapter="06_frontend"; Number=190; Name="framework_comparison"; Title="综合：多框架对比示例"},
     # 资源与样式 (e191-e200)
     @{Chapter="06_frontend"; Number=191; Name="static_assets";        Title="静态资源加载"},
     @{Chapter="06_frontend"; Number=192; Name="css_variables";        Title="自定义 CSS 变量和主题"},
     @{Chapter="06_frontend"; Number=193; Name="dark_mode";            Title="暗黑模式"},
     @{Chapter="06_frontend"; Number=194; Name="tailwind";             Title="Tailwind CSS 集成"},
     @{Chapter="06_frontend"; Number=195; Name="icon_system";          Title="图标系统"},
     @{Chapter="06_frontend"; Number=196; Name="font_loading";         Title="字体加载与管理"},
     @{Chapter="06_frontend"; Number=197; Name="responsive_layout";    Title="响应式布局"},
     @{Chapter="06_frontend"; Number=198; Name="i18n";                 Title="多语言支持"},
     @{Chapter="06_frontend"; Number=199; Name="custom_titlebar";      Title="无边框窗口自定义标题栏"},
     @{Chapter="06_frontend"; Number=200; Name="context_menu";         Title="右键菜单"},
 
     # ========== 第 7 章：插件系统 (07_plugins, e201-e270) ==========
     # FS 插件 (e201-e210)
     @{Chapter="07_plugins"; Number=201; Name="fs_install";           Title="FS 插件安装与配置"},
     @{Chapter="07_plugins"; Number=202; Name="fs_read_file";         Title="读取文件内容"},
     @{Chapter="07_plugins"; Number=203; Name="fs_write_file";        Title="写入文件内容"},
     @{Chapter="07_plugins"; Number=204; Name="fs_directory";         Title="目录操作"},
     @{Chapter="07_plugins"; Number=205; Name="fs_metadata";          Title="文件元信息"},
     @{Chapter="07_plugins"; Number=206; Name="fs_copy_move";         Title="文件复制/移动/删除"},
     @{Chapter="07_plugins"; Number=207; Name="fs_exists";            Title="文件存在性检查"},
     @{Chapter="07_plugins"; Number=208; Name="fs_glob";              Title="文件通配符匹配"},
     @{Chapter="07_plugins"; Number=209; Name="fs_scope";             Title="文件系统权限范围"},
     @{Chapter="07_plugins"; Number=210; Name="fs_file_manager";      Title="综合：文件管理器基础功能"},
     # Dialog 插件 (e211-e220)
     @{Chapter="07_plugins"; Number=211; Name="dialog_install";       Title="Dialog 插件安装与配置"},
     @{Chapter="07_plugins"; Number=212; Name="dialog_open";          Title="打开文件对话框"},
     @{Chapter="07_plugins"; Number=213; Name="dialog_save";          Title="保存文件对话框"},
     @{Chapter="07_plugins"; Number=214; Name="dialog_multi_file";    Title="多文件选择"},
     @{Chapter="07_plugins"; Number=215; Name="dialog_directory";     Title="目录选择器"},
     @{Chapter="07_plugins"; Number=216; Name="dialog_message";       Title="消息对话框"},
     @{Chapter="07_plugins"; Number=217; Name="dialog_confirm";       Title="确认对话框"},
     @{Chapter="07_plugins"; Number=218; Name="dialog_custom";        Title="自定义对话框样式"},
     @{Chapter="07_plugins"; Number=219; Name="dialog_result";        Title="对话框结果处理"},
     @{Chapter="07_plugins"; Number=220; Name="file_import_export";   Title="综合：文件导入导出助手"},
     # Shell 插件 (e221-e230)
     @{Chapter="07_plugins"; Number=221; Name="shell_install";        Title="Shell 插件安装与配置"},
     @{Chapter="07_plugins"; Number=222; Name="shell_execute";        Title="执行系统命令"},
     @{Chapter="07_plugins"; Number=223; Name="shell_args";           Title="命令参数"},
     @{Chapter="07_plugins"; Number=224; Name="shell_output";         Title="获取命令输出"},
     @{Chapter="07_plugins"; Number=225; Name="shell_async";          Title="异步执行与等待"},
     @{Chapter="07_plugins"; Number=226; Name="shell_timeout";        Title="命令超时控制"},
     @{Chapter="07_plugins"; Number=227; Name="shell_sidecar";        Title="Sidecar 模式"},
     @{Chapter="07_plugins"; Number=228; Name="shell_sidecar_ipc";    Title="Sidecar 参数与通信"},
     @{Chapter="07_plugins"; Number=229; Name="shell_nodejs";         Title="Node.js 作为 Sidecar"},
     @{Chapter="07_plugins"; Number=230; Name="command_palette";      Title="综合：系统命令面板"},
     # SQL 插件 (e231-e240)
     @{Chapter="07_plugins"; Number=231; Name="sql_install";          Title="SQL 插件安装与配置"},
     @{Chapter="07_plugins"; Number=232; Name="sql_create";           Title="创建数据库和数据表"},
     @{Chapter="07_plugins"; Number=233; Name="sql_insert";           Title="插入数据"},
     @{Chapter="07_plugins"; Number=234; Name="sql_query";            Title="查询数据"},
     @{Chapter="07_plugins"; Number=235; Name="sql_update_delete";    Title="更新与删除"},
     @{Chapter="07_plugins"; Number=236; Name="sql_transaction";      Title="事务处理"},
     @{Chapter="07_plugins"; Number=237; Name="sql_migration";        Title="迁移管理"},
     @{Chapter="07_plugins"; Number=238; Name="sql_join";             Title="关联查询"},
     @{Chapter="07_plugins"; Number=239; Name="sql_encryption";       Title="数据库加密"},
     @{Chapter="07_plugins"; Number=240; Name="sql_notes_app";        Title="综合：SQLite 笔记管理器"},
     # Store 插件 (e241-e245)
     @{Chapter="07_plugins"; Number=241; Name="store_install";        Title="Store 插件安装与配置"},
     @{Chapter="07_plugins"; Number=242; Name="store_kv";             Title="键值对读写"},
     @{Chapter="07_plugins"; Number=243; Name="store_persist";        Title="持久化存储"},
     @{Chapter="07_plugins"; Number=244; Name="store_watch";          Title="监听存储变化"},
     @{Chapter="07_plugins"; Number=245; Name="store_settings";       Title="综合：应用设置持久化"},
     # 通知与剪贴板 (e246-e250)
     @{Chapter="07_plugins"; Number=246; Name="notify_send";          Title="发送系统通知"},
     @{Chapter="07_plugins"; Number=247; Name="notify_permission";    Title="通知权限请求"},
     @{Chapter="07_plugins"; Number=248; Name="notify_click";         Title="通知点击事件"},
     @{Chapter="07_plugins"; Number=249; Name="clipboard_rw";         Title="读写系统剪贴板"},
     @{Chapter="07_plugins"; Number=250; Name="clipboard_watch";      Title="剪贴板监听"},
     # HTTP 与网络 (e251-e260)
     @{Chapter="07_plugins"; Number=251; Name="http_install";         Title="HTTP 插件安装与配置"},
     @{Chapter="07_plugins"; Number=252; Name="http_get";             Title="GET 请求"},
     @{Chapter="07_plugins"; Number=253; Name="http_post";            Title="POST 请求"},
     @{Chapter="07_plugins"; Number=254; Name="http_headers";         Title="请求头与认证"},
     @{Chapter="07_plugins"; Number=255; Name="http_upload";          Title="文件上传"},
     @{Chapter="07_plugins"; Number=256; Name="http_websocket";       Title="WebSocket 连接"},
     @{Chapter="07_plugins"; Number=257; Name="http_websocket_reconnect"; Title="WebSocket 重连"},
     @{Chapter="07_plugins"; Number=258; Name="http_network_status";  Title="网络状态检测"},
     @{Chapter="07_plugins"; Number=259; Name="http_timeout";         Title="HTTP 请求超时"},
     @{Chapter="07_plugins"; Number=260; Name="http_rest_client";     Title="综合：REST API 客户端"},
     # 其他官方插件 (e261-e270)
     @{Chapter="07_plugins"; Number=261; Name="plugin_os_info";       Title="OS 信息"},
     @{Chapter="07_plugins"; Number=262; Name="plugin_process";       Title="Process 管理"},
     @{Chapter="07_plugins"; Number=263; Name="plugin_global_shortcut"; Title="全局快捷键"},
     @{Chapter="07_plugins"; Number=264; Name="plugin_autostart";     Title="自启动"},
     @{Chapter="07_plugins"; Number=265; Name="plugin_logging";       Title="日志系统"},
     @{Chapter="07_plugins"; Number=266; Name="plugin_geolocation";   Title="位置服务"},
     @{Chapter="07_plugins"; Number=267; Name="plugin_deep_link";     Title="深度链接"},
     @{Chapter="07_plugins"; Number=268; Name="plugin_window_state";  Title="窗口状态持久化"},
     @{Chapter="07_plugins"; Number=269; Name="plugin_opener";        Title="启动器"},
     @{Chapter="07_plugins"; Number=270; Name="plugin_multi_collab";  Title="综合：多插件协作"},
 
     # ========== 第 8 章：窗口、菜单与托盘 (08_window_menu_tray, e271-e305) ==========
     # 窗口管理 (e271-e285)
     @{Chapter="08_window_menu_tray"; Number=271; Name="window_create";           Title="创建新窗口"},
     @{Chapter="08_window_menu_tray"; Number=272; Name="window_label";            Title="窗口标签与引用"},
     @{Chapter="08_window_menu_tray"; Number=273; Name="window_set_size";          Title="设置窗口大小"},
     @{Chapter="08_window_menu_tray"; Number=274; Name="window_set_position";      Title="设置窗口位置"},
     @{Chapter="08_window_menu_tray"; Number=275; Name="window_resize_control";    Title="窗口缩放控制"},
     @{Chapter="08_window_menu_tray"; Number=276; Name="window_title";             Title="窗口标题控制"},
     @{Chapter="08_window_menu_tray"; Number=277; Name="window_visibility";        Title="窗口可见性"},
     @{Chapter="08_window_menu_tray"; Number=278; Name="window_focus";             Title="窗口焦点控制"},
     @{Chapter="08_window_menu_tray"; Number=279; Name="window_fullscreen";        Title="全屏与最大化"},
     @{Chapter="08_window_menu_tray"; Number=280; Name="window_decorations";       Title="窗口装饰控制"},
     @{Chapter="08_window_menu_tray"; Number=281; Name="window_transparency";      Title="窗口透明度"},
     @{Chapter="08_window_menu_tray"; Number=282; Name="window_background";        Title="窗口背景色"},
     @{Chapter="08_window_menu_tray"; Number=283; Name="window_child";             Title="子窗口管理"},
     @{Chapter="08_window_menu_tray"; Number=284; Name="window_communication";     Title="窗口间通信"},
     @{Chapter="08_window_menu_tray"; Number=285; Name="multi_window_editor";      Title="综合：多窗口编辑器"},
     # 菜单系统 (e286-e295)
     @{Chapter="08_window_menu_tray"; Number=286; Name="menu_create";              Title="创建应用菜单"},
     @{Chapter="08_window_menu_tray"; Number=287; Name="menu_item_types";          Title="菜单项类型"},
     @{Chapter="08_window_menu_tray"; Number=288; Name="menu_separator";           Title="菜单分隔符"},
     @{Chapter="08_window_menu_tray"; Number=289; Name="menu_submenu";             Title="子菜单嵌套"},
     @{Chapter="08_window_menu_tray"; Number=290; Name="menu_events";              Title="菜单事件处理"},
     @{Chapter="08_window_menu_tray"; Number=291; Name="menu_shortcuts";           Title="快捷键绑定"},
     @{Chapter="08_window_menu_tray"; Number=292; Name="menu_state";               Title="菜单状态控制"},
     @{Chapter="08_window_menu_tray"; Number=293; Name="menu_platform";            Title="平台特殊菜单"},
     @{Chapter="08_window_menu_tray"; Number=294; Name="menu_predefined";          Title="预定义菜单项"},
     @{Chapter="08_window_menu_tray"; Number=295; Name="menu_dynamic";             Title="动态菜单"},
     @{Chapter="08_window_menu_tray"; Number=296; Name="menu_context";             Title="上下文菜单（右键）"},
     @{Chapter="08_window_menu_tray"; Number=297; Name="menu_window_assoc";        Title="菜单与窗口关联"},
     @{Chapter="08_window_menu_tray"; Number=298; Name="menu_i18n";                Title="菜单国际化"},
     @{Chapter="08_window_menu_tray"; Number=299; Name="menu_icons";               Title="菜单图标"},
     @{Chapter="08_window_menu_tray"; Number=300; Name="full_menu_system";         Title="综合：完整菜单系统"},
     # 系统托盘 (e301-e305)
     @{Chapter="08_window_menu_tray"; Number=301; Name="tray_create";              Title="创建系统托盘"},
     @{Chapter="08_window_menu_tray"; Number=302; Name="tray_icon";                Title="系统托盘图标"},
     @{Chapter="08_window_menu_tray"; Number=303; Name="tray_menu";                Title="系统托盘菜单"},
     @{Chapter="08_window_menu_tray"; Number=304; Name="tray_click";               Title="托盘点击事件"},
     @{Chapter="08_window_menu_tray"; Number=305; Name="tray_tooltip";             Title="托盘提示文字"},
 
     # ========== 第 9 章：验证与测试 (09_testing, e306-e320) ==========
     @{Chapter="09_testing"; Number=306; Name="test_unit";              Title="Rust 单元测试"},
     @{Chapter="09_testing"; Number=307; Name="test_integration";       Title="命令集成测试"},
     @{Chapter="09_testing"; Number=308; Name="test_webdriver";         Title="WebDriver 端到端测试"},
     @{Chapter="09_testing"; Number=309; Name="test_e2e";               Title="E2E 测试用例编写"},
     @{Chapter="09_testing"; Number=310; Name="test_frontend_unit";     Title="前端单元测试"},
     @{Chapter="09_testing"; Number=311; Name="test_frontend_component"; Title="前端组件测试"},
     @{Chapter="09_testing"; Number=312; Name="test_ci";                Title="持续集成配置"},
     @{Chapter="09_testing"; Number=313; Name="test_cross_platform";    Title="跨平台测试策略"},
     @{Chapter="09_testing"; Number=314; Name="test_benchmark";         Title="性能基准测试"},
     @{Chapter="09_testing"; Number=315; Name="test_pipeline";          Title="综合：自动化测试流水线"},
     # 打包发布 (e316-e320)
     @{Chapter="09_testing"; Number=316; Name="build_optimize";         Title="构建配置优化"},
     @{Chapter="09_testing"; Number=317; Name="build_windows";          Title="Windows 打包 NSIS/MSI"},
     @{Chapter="09_testing"; Number=318; Name="build_macos";            Title="macOS 打包 DMG/AppBundle"},
     @{Chapter="09_testing"; Number=319; Name="build_linux";            Title="Linux 打包 AppImage/deb"},
     @{Chapter="09_testing"; Number=320; Name="build_icons";            Title="应用图标与资源"},
 
     # ========== 第 10 章：高级主题 (10_advanced, e321-e355) ==========
     # 安全模型 (e321-e330)
     @{Chapter="10_advanced"; Number=321; Name="security_architecture";   Title="Tauri 安全架构"},
     @{Chapter="10_advanced"; Number=322; Name="security_permissions";    Title="权限系统基础"},
     @{Chapter="10_advanced"; Number=323; Name="security_capabilities";   Title="Capabilities 配置"},
     @{Chapter="10_advanced"; Number=324; Name="security_custom_perms";   Title="自定义命令权限"},
     @{Chapter="10_advanced"; Number=325; Name="security_platform_perms"; Title="平台特定权限"},
     @{Chapter="10_advanced"; Number=326; Name="security_scope";          Title="Scope 系统"},
     @{Chapter="10_advanced"; Number=327; Name="security_remote_access";  Title="远程 API 访问控制"},
     @{Chapter="10_advanced"; Number=328; Name="security_csp";            Title="内容安全策略 CSP"},
     @{Chapter="10_advanced"; Number=329; Name="security_isolation";      Title="隔离模式"},
     @{Chapter="10_advanced"; Number=330; Name="security_coding";         Title="安全意识编码实践"},
     @{Chapter="10_advanced"; Number=331; Name="security_sensitive_data"; Title="敏感数据处理"},
     @{Chapter="10_advanced"; Number=332; Name="security_updates";        Title="安全更新机制"},
     @{Chapter="10_advanced"; Number=333; Name="security_audit";          Title="审计和日志"},
     @{Chapter="10_advanced"; Number=334; Name="security_dependencies";   Title="第三方依赖安全"},
     @{Chapter="10_advanced"; Number=335; Name="security_review_tool";    Title="综合：安全审查工具"},
     # 发布 (e336-e345)
     @{Chapter="10_advanced"; Number=336; Name="publish_update";          Title="自动更新配置"},
     @{Chapter="10_advanced"; Number=337; Name="publish_codesign";        Title="代码签名"},
     @{Chapter="10_advanced"; Number=338; Name="publish_size_optimize";   Title="应用大小优化"},
     @{Chapter="10_advanced"; Number=339; Name="publish_static_build";    Title="静态编译与运行时依赖"},
     @{Chapter="10_advanced"; Number=340; Name="publish_pipeline";        Title="综合：完整发布流水线"},
     # 移动端 (e341-e350)
     @{Chapter="10_advanced"; Number=341; Name="mobile_setup";           Title="移动端开发环境配置"},
     @{Chapter="10_advanced"; Number=342; Name="mobile_android";         Title="Android 项目结构"},
     @{Chapter="10_advanced"; Number=343; Name="mobile_ios";             Title="iOS 项目结构"},
     @{Chapter="10_advanced"; Number=344; Name="mobile_config";          Title="移动端 Tauri 配置"},
     @{Chapter="10_advanced"; Number=345; Name="mobile_commands";        Title="移动端命令开发"},
     @{Chapter="10_advanced"; Number=346; Name="mobile_permissions";     Title="移动端权限处理"},
     @{Chapter="10_advanced"; Number=347; Name="mobile_ui";              Title="移动端适配 UI"},
     @{Chapter="10_advanced"; Number=348; Name="mobile_plugins";         Title="移动端特有插件"},
     @{Chapter="10_advanced"; Number=349; Name="mobile_android_build";   Title="Android 构建与签名"},
     @{Chapter="10_advanced"; Number=350; Name="mobile_ios_build";       Title="iOS 构建与 App Store"},
     # 收尾 (e351-e355)
     @{Chapter="10_advanced"; Number=351; Name="advanced_tray_icon";     Title="托盘动画图标"},
     @{Chapter="10_advanced"; Number=352; Name="advanced_tray_hide";     Title="窗口隐藏到托盘"},
     @{Chapter="10_advanced"; Number=353; Name="advanced_tray_restore";  Title="托盘恢复窗口"},
     @{Chapter="10_advanced"; Number=354; Name="advanced_tray_platform"; Title="平台特定托盘行为"},
     @{Chapter="10_advanced"; Number=355; Name="advanced_tray_app";      Title="综合：托盘应用"}
 )
 
 # ====================================================================
 # 初始化计数器
 # ====================================================================
 $TotalExercises = $AllExercises.Count
 $Completed = 0
 $Failed = 0
 
 Write-Host "========================================" -ForegroundColor Cyan
 Write-Host "Tauri v2 练习项目批量初始化" -ForegroundColor Cyan
 Write-Host "共 $TotalExercises 道练习" -ForegroundColor Cyan
 Write-Host "========================================" -ForegroundColor Cyan
 Write-Host ""
 
 # ====================================================================
 # 辅助函数：从模板创建单个练习项目（含答案）
 # ====================================================================
 function New-ExercisePair {
     param(
         [string]$Chapter,
         [int]$Number,
         [string]$Name,
         [string]$Title
     )
 
     $NumStr = $Number.ToString("D3")
     $DevPort = 1420 + $Number
     $ChapterDir = Join-Path $ProjectRoot $Chapter
 
     # 确保章节目录存在
     if (-not (Test-Path $ChapterDir)) {
         New-Item -ItemType Directory -Path $ChapterDir -Force | Out-Null
     }
 
     # 练习和答案目录
     $ExerciseDir = Join-Path $ChapterDir "e${NumStr}_${Name}"
     $AnswerDir = Join-Path $ChapterDir "e${NumStr}_${Name}_answer"
 
     # === 创建练习项目 ===
     if (-not (Test-Path $ExerciseDir)) {
         Copy-Item -Path $TemplateDir -Destination $ExerciseDir -Recurse -Force
         Update-ProjectFiles -TargetDir $ExerciseDir -Suffix "" -NumStr $NumStr -Name $Name -Title $Title -DevPort $DevPort -Chapter $Chapter
         Write-Host "  ✓ 练习" -NoNewline -ForegroundColor Green
         Write-Host " e${NumStr}_${Name}"
     } else {
         Write-Host "  · 跳过 e${NumStr}_${Name} (已存在)" -ForegroundColor Yellow
     }
 
     # === 创建答案项目 ===
     if (-not (Test-Path $AnswerDir)) {
         Copy-Item -Path $TemplateDir -Destination $AnswerDir -Recurse -Force
         Update-ProjectFiles -TargetDir $AnswerDir -Suffix "_answer" -NumStr $NumStr -Name $Name -Title $Title -DevPort $DevPort -Chapter $Chapter
         Write-Host "  ✓ 答案" -NoNewline -ForegroundColor Green
         Write-Host " e${NumStr}_${Name}_answer"
     } else {
         Write-Host "  · 跳过 e${NumStr}_${Name}_answer (已存在)" -ForegroundColor Yellow
     }
 }
 
 # ====================================================================
 # 辅助函数：更新项目文件
 # ====================================================================
 function Update-ProjectFiles {
     param($TargetDir, $Suffix, $NumStr, $Name, $Title, $DevPort, $Chapter)
 
     # 1. 更新 package.json
     $PkgPath = Join-Path $TargetDir "package.json"
     if (Test-Path $PkgPath) {
         $json = Get-Content $PkgPath -Raw | ConvertFrom-Json
         $json.name = "e${NumStr}_${Name}$Suffix"
         $json | ConvertTo-Json -Depth 10 | Set-Content $PkgPath
     }
 
     # 2. 更新 Cargo.toml
     $CargoPath = Join-Path $TargetDir "src-tauri\Cargo.toml"
     if (Test-Path $CargoPath) {
         $content = Get-Content $CargoPath -Raw
         $libName = "e${NumStr}_${Name}$Suffix" -replace "-", "_"
         $content = $content -replace "exercise-template-vite-ts", "e${NumStr}_${Name}$Suffix"
         $content = $content -replace "exercise_template_vite_ts_lib", "${libName}_lib"
         Set-Content $CargoPath $content
     }
 
     # 3. 更新 tauri.conf.json
     $ConfPath = Join-Path $TargetDir "src-tauri\tauri.conf.json"
     if (Test-Path $ConfPath) {
         $conf = Get-Content $ConfPath -Raw | ConvertFrom-Json
         $conf.productName = "e${NumStr}_${Name}$Suffix"
         $conf.identifier = "com.taurilearn.e${NumStr}${Suffix}"
         $conf.build.devUrl = "http://localhost:${DevPort}"
         $conf.app.windows[0].title = "练习 ${NumStr}: ${Title}$(if ($Suffix -eq '_answer') { ' (答案)' } else { '' })"
         $conf | ConvertTo-Json -Depth 10 | Set-Content $ConfPath
     }
 
     # 4. 更新 index.html title
     $IdxPath = Join-Path $TargetDir "index.html"
     if (Test-Path $IdxPath) {
         $html = Get-Content $IdxPath -Raw
         $titleText = "练习 ${NumStr}: ${Title}$(if ($Suffix -eq '_answer') { ' (答案)' } else { '' })"
         $html = $html -replace "<title>.*</title>", "<title>${titleText}</title>"
         Set-Content $IdxPath $html
     }
 
     # 5. 更新 workspace Cargo.toml members
     $WsPath = Join-Path $ProjectRoot "Cargo.toml"
     if (Test-Path $WsPath) {
         $memberPath = "${Chapter}/e${NumStr}_${Name}${Suffix}/src-tauri"
         $wsContent = Get-Content $WsPath -Raw
         if ($wsContent -notmatch [regex]::Escape($memberPath)) {
             $wsContent = $wsContent -replace "(members\s*=\s*\[)", "`$1`n    `"${memberPath}`","
             Set-Content $WsPath $wsContent
         }
     }
 }
 
 # ====================================================================
 # 主循环
 # ====================================================================
 foreach ($Ex in $AllExercises) {
     New-ExercisePair -Chapter $Ex.Chapter -Number $Ex.Number -Name $Ex.Name -Title $Ex.Title
     $Completed++
     if ($Completed % 10 -eq 0) {
         $Elapsed = (Get-Date) - $StartTime
         Write-Host "  [进度: $Completed / $TotalExercises, 耗时: $($Elapsed.Minutes)m]" -ForegroundColor Magenta
     }
 }
 
 # ====================================================================
 # 完成
 # ====================================================================
 $TotalElapsed = (Get-Date) - $StartTime
 Write-Host ""
 Write-Host "========================================" -ForegroundColor Cyan
 Write-Host "批量初始化完成!" -ForegroundColor Cyan
 Write-Host "总练习数: $TotalExercises" -ForegroundColor Cyan
 Write-Host "总项目数: $($TotalExercises * 2) (练习 + 答案)" -ForegroundColor Cyan
 Write-Host "成功: $Completed" -ForegroundColor Cyan
 Write-Host "失败: $Failed" -ForegroundColor Cyan
 Write-Host "总耗时: $($TotalElapsed.Minutes)m $($TotalElapsed.Seconds)s" -ForegroundColor Cyan
 Write-Host "========================================" -ForegroundColor Cyan
 Write-Host ""
 Write-Host "下一步: 使用编写 agent 批量填充各练习的 TODO 和参考答案代码" -ForegroundColor Yellow
