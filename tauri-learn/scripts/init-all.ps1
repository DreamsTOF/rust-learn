# ============================================================
# init-all.ps1 — 批量初始化 Tauri v2 学习 workspace（84 练习 × 练习/答案 = 168 项目）
#
# 规划依据: .trae/documents/tauri-learn-plan.md / tauri-learn-agent-plan.md
# 端口规则: 全部项目按次序递增，从 1420 开始，每项目 +1（168 个项目 → 1420..1587）
# 前端模板: 01/02/03 块 Vanilla TS + Vite；04 超级项目 React
#
# 用法: pwsh scripts/init-all.ps1
# ============================================================

$ErrorActionPreference = "Stop"
$Root = Split-Path $PSScriptRoot -Parent

# 加载单项目生成器（dot-source，进程内调用）
. (Join-Path $PSScriptRoot "new-exercise.ps1")

# ---------- 1. 练习清单（四大块） ----------

$Blocks = @(
    @{ Dir = "01_getting_started"; Label = "入门"; Items = @(
        @{ Id = "e01"; Name = "hello_world";            Title = "环境准备与项目创建" },
        @{ Id = "e02"; Name = "project_structure";      Title = "项目结构" },
        @{ Id = "e03"; Name = "run_and_build";          Title = "运行与构建" },
        @{ Id = "e04"; Name = "first_command";          Title = "第一个命令" },
        @{ Id = "e05"; Name = "params_and_return";      Title = "参数与返回值" },
        @{ Id = "e06"; Name = "window_config";          Title = "窗口配置" },
        @{ Id = "e07"; Name = "debugging";              Title = "调试" },
        @{ Id = "e08"; Name = "packaging_and_icons";    Title = "打包与图标" }
    )},
    @{ Dir = "02_commands"; Label = "基本命令和语法"; Items = @(
        @{ Id = "e09"; Name = "async_command";          Title = "异步命令" },
        @{ Id = "e10"; Name = "dependency_injection";   Title = "依赖注入" },
        @{ Id = "e11"; Name = "error_handling";         Title = "错误处理" },
        @{ Id = "e12"; Name = "channel_stream";         Title = "Channel 流式传输" },
        @{ Id = "e13"; Name = "command_modules";        Title = "命令模块化" },
        @{ Id = "e14"; Name = "mutable_state";          Title = "可变状态" },
        @{ Id = "e15"; Name = "type_sync";              Title = "前后端类型同步" },
        @{ Id = "e16"; Name = "setup_hook";             Title = "setup 钩子" },
        @{ Id = "e17"; Name = "exit_interception";      Title = "退出拦截" },
        @{ Id = "e18"; Name = "path_api";               Title = "路径 API" },
        @{ Id = "e19"; Name = "background_tasks";       Title = "后台任务" },
        @{ Id = "e20"; Name = "single_instance";        Title = "单实例" },
        @{ Id = "e21"; Name = "frontend_events";        Title = "前端事件" },
        @{ Id = "e22"; Name = "window_events";          Title = "窗口级事件" },
        @{ Id = "e23"; Name = "backend_listen";         Title = "后端监听" },
        @{ Id = "e24"; Name = "window_operations";      Title = "创建与操作窗口" },
        @{ Id = "e25"; Name = "window_event_handling";  Title = "窗口事件" },
        @{ Id = "e26"; Name = "frameless_window";       Title = "无边框窗口" },
        @{ Id = "e27"; Name = "app_menu";               Title = "应用菜单" },
        @{ Id = "e28"; Name = "system_tray";            Title = "系统托盘" },
        @{ Id = "e29"; Name = "window_state";           Title = "窗口状态持久化" },
        @{ Id = "e30"; Name = "fs_plugin";              Title = "文件系统" },
        @{ Id = "e31"; Name = "dialog";                 Title = "对话框" },
        @{ Id = "e32"; Name = "shell_plugin";           Title = "Shell" },
        @{ Id = "e33"; Name = "sql_plugin";             Title = "SQL" },
        @{ Id = "e34"; Name = "store_plugin";           Title = "Store" },
        @{ Id = "e35"; Name = "notification";           Title = "通知" },
        @{ Id = "e36"; Name = "clipboard";              Title = "剪贴板" },
        @{ Id = "e37"; Name = "http_plugin";            Title = "HTTP" },
        @{ Id = "e38"; Name = "os_opener";              Title = "OS 与 Opener" },
        @{ Id = "e39"; Name = "global_shortcut";        Title = "全局快捷键" },
        @{ Id = "e40"; Name = "vite_hmr";               Title = "Vite 与 HMR" },
        @{ Id = "e41"; Name = "react_integration";      Title = "React 集成" },
        @{ Id = "e42"; Name = "theme_switch";           Title = "主题切换" },
        @{ Id = "e43"; Name = "static_assets";          Title = "静态资源" },
        @{ Id = "e44"; Name = "csp";                    Title = "内容安全策略" },
        @{ Id = "e45"; Name = "permissions";            Title = "权限系统" },
        @{ Id = "e46"; Name = "error_propagation";      Title = "自定义错误传播" },
        @{ Id = "e47"; Name = "packaging";              Title = "打包发布" },
        @{ Id = "e48"; Name = "updater";                Title = "自动更新" }
    )},
    @{ Dir = "03_simple_projects"; Label = "简单项目"; Items = @(
        @{ Id = "e49"; Name = "todo_list";              Title = "待办清单" },
        @{ Id = "e50"; Name = "password_generator";     Title = "密码生成器" },
        @{ Id = "e51"; Name = "file_notes";             Title = "文件笔记" },
        @{ Id = "e52"; Name = "system_monitor";         Title = "系统监视器" },
        @{ Id = "e53"; Name = "pomodoro_timer";         Title = "番茄计时器" },
        @{ Id = "e54"; Name = "image_viewer";           Title = "图片查看器" },
        @{ Id = "e55"; Name = "ledger";                 Title = "记账本" },
        @{ Id = "e56"; Name = "exchange_rate";          Title = "汇率查询" },
        @{ Id = "e57"; Name = "batch_rename";           Title = "批量重命名" },
        @{ Id = "e58"; Name = "rss_reader";             Title = "RSS 阅读器" }
    )},
    @{ Dir = "04_super_project"; Label = "超级项目（Markdown 编辑器）"; React = $true; Items = @(
        @{ Id = "p01"; Name = "project_init";           Title = "项目初始化" },
        @{ Id = "p02"; Name = "data_model";             Title = "数据模型与存储层" },
        @{ Id = "p03"; Name = "editor_core";            Title = "编辑器核心" },
        @{ Id = "p04"; Name = "file_management";        Title = "文件管理" },
        @{ Id = "p05"; Name = "live_preview";           Title = "实时预览" },
        @{ Id = "p06"; Name = "toolbar";                Title = "工具栏" },
        @{ Id = "p07"; Name = "tabs";                   Title = "多文件标签页" },
        @{ Id = "p08"; Name = "undo_redo_stats";        Title = "撤销/重做与字数统计" },
        @{ Id = "p09"; Name = "search_replace";         Title = "搜索替换" },
        @{ Id = "p10"; Name = "autosave";               Title = "自动保存" },
        @{ Id = "p11"; Name = "theme_system";           Title = "主题系统" },
        @{ Id = "p12"; Name = "drag_drop";              Title = "拖放支持" },
        @{ Id = "p13"; Name = "image_management";       Title = "图片管理" },
        @{ Id = "p14"; Name = "toc_navigation";         Title = "目录导航" },
        @{ Id = "p15"; Name = "tray";                   Title = "系统托盘" },
        @{ Id = "p16"; Name = "shortcut";               Title = "全局快捷键" },
        @{ Id = "p17"; Name = "multi_window";           Title = "多窗口" },
        @{ Id = "p18"; Name = "clipboard_notify";       Title = "剪贴板与通知" },
        @{ Id = "p19"; Name = "export";                 Title = "导出增强" },
        @{ Id = "p20"; Name = "cloud_sync";             Title = "云同步" },
        @{ Id = "p21"; Name = "spell_check";            Title = "拼写检查" },
        @{ Id = "p22"; Name = "settings";               Title = "设置面板" },
        @{ Id = "p23"; Name = "error_logging";          Title = "错误处理与日志" },
        @{ Id = "p24"; Name = "security";               Title = "安全加固" },
        @{ Id = "p25"; Name = "release";                Title = "打包发布" },
        @{ Id = "p26"; Name = "updater_acceptance";     Title = "自动更新与验收" }
    )}
)

# ---------- 2. 批量生成（端口全局递增） ----------

Write-Host "== 开始批量初始化 =="
$port = 1420
$rows = [System.Collections.Generic.List[object]]::new()

foreach ($block in $Blocks) {
    Write-Host ""
    Write-Host "--- $($block.Dir) ($($block.Label)) ---"
    foreach ($item in $block.Items) {
        $react = [bool]$block.React
        New-Exercise -Chapter $block.Dir -Id $item.Id -Name $item.Name -Title $item.Title -Port $port -React:$react
        $rows.Add([pscustomobject]@{ Dir = $block.Dir; Id = $item.Id; Name = $item.Name; Title = $item.Title; Port = $port; React = $react; Answer = $false })
        $port++
        New-Exercise -Chapter $block.Dir -Id $item.Id -Name $item.Name -Title $item.Title -Port $port -Answer -React:$react
        $rows.Add([pscustomobject]@{ Dir = $block.Dir; Id = $item.Id; Name = $item.Name; Title = $item.Title; Port = $port; React = $react; Answer = $true })
        $port++
    }
}

Write-Host ""
Write-Host "== 生成 $($rows.Count) 个项目，端口 $($rows[0].Port) - $($rows[-1].Port) =="

# ---------- 3. workspace 根文件 ----------

# Cargo.toml
$members = foreach ($r in $rows) {
    $suffix = if ($r.Answer) { "_answer" } else { "" }
    "    `"$($r.Dir)/$($r.Id)_$($r.Name)$suffix/src-tauri`","
}
$cargoRoot = @"
[workspace]
resolver = "2"
members = [
$($members -join "`n")
]

[workspace.package]
version = "0.1.0"
edition = "2024"

[workspace.dependencies]
tauri = { version = "2.11", features = [] }
tauri-build = "2.6"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
thiserror = "2"
tokio = { version = "1", features = ["full"] }
tauri-plugin-fs = "2.5"
tauri-plugin-dialog = "2.7"
tauri-plugin-shell = "2.3"
tauri-plugin-sql = { version = "2.4", features = ["sqlite"] }
tauri-plugin-store = "2.4"
tauri-plugin-notification = "2.3"
tauri-plugin-clipboard-manager = "2.3"
tauri-plugin-http = "2.5"
tauri-plugin-os = "2.3"
tauri-plugin-opener = "2.5"
tauri-plugin-global-shortcut = "2.3"
tauri-plugin-window-state = "2.3"
tauri-plugin-updater = "2.10"
tauri-plugin-single-instance = "2.3"
"@
[System.IO.File]::WriteAllText((Join-Path $Root "Cargo.toml"), $cargoRoot, [System.Text.UTF8Encoding]::new($false))

# rust-toolchain.toml
$toolchain = @"
[toolchain]
channel = "stable"
"@
[System.IO.File]::WriteAllText((Join-Path $Root "rust-toolchain.toml"), $toolchain, [System.Text.UTF8Encoding]::new($false))

# package.json（根，pnpm workspace 元数据）
$pkg = @{
    name = "tauri-learn"
    private = $true
    version = "0.1.0"
    description = "Tauri v2 练习项目（84 练习 × 练习/答案 = 168 个项目）"
}
[System.IO.File]::WriteAllText((Join-Path $Root "package.json"), ($pkg | ConvertTo-Json), [System.Text.UTF8Encoding]::new($false))

# pnpm-workspace.yaml
$pnpm = @"
packages:
  - "01_getting_started/*"
  - "02_commands/*"
  - "03_simple_projects/*"
  - "04_super_project/*"
"@
[System.IO.File]::WriteAllText((Join-Path $Root "pnpm-workspace.yaml"), $pnpm, [System.Text.UTF8Encoding]::new($false))

# .gitignore
$gitignore = @"
# Rust
target/
Cargo.lock

# Frontend
node_modules/
dist/

# Tauri
src-tauri/gen/

# IDE
.idea/
*.swp
*.swo
*~

# OS
.DS_Store
Thumbs.db

# Build artifacts
*.log
"@
[System.IO.File]::WriteAllText((Join-Path $Root ".gitignore"), $gitignore, [System.Text.UTF8Encoding]::new($false))

# README.md
$readme = @"
# Tauri v2 练习项目

共 **84 个练习**（每练习含练习版 + 答案版，共 168 个独立项目），按四大块组织：

| 块 | 目录 | 题量 | 前端 |
| :-: | ---- | :--: | :--: |
| 入门 | 01_getting_started/ | 8 | Vanilla TS + Vite |
| 基本命令和语法 | 02_commands/ | 40 | Vanilla TS + Vite |
| 简单项目 | 03_simple_projects/ | 10 | Vanilla TS + Vite |
| 超级项目 | 04_super_project/ | 26 步 | React |

## 端口分配

全部项目端口按次序递增（练习版与答案版各占一个），从 1420 到 $($rows[-1].Port)。完整端口表见 [00_preface/index.html](00_preface/index.html)。

## 运行单个练习

````bash
cd 02_commands/e10_dependency_injection
pnpm install
cargo tauri dev
````

## 目录约定

- 每个练习两个项目：练习版（`eNN_name`，含 TODO）与答案版（`eNN_name_answer`）
- 超级项目 26 步为串行递进（p01 → p26），每步依赖前一步代码
- 所有项目已预注册为 Cargo workspace members

## 相关文档

- 内容规划: .trae/documents/tauri-learn-plan.md
- Agent 编写流水线: .trae/documents/tauri-learn-agent-plan.md
"@
[System.IO.File]::WriteAllText((Join-Path $Root "README.md"), $readme, [System.Text.UTF8Encoding]::new($false))

# ---------- 4. 00_preface 导航首页 ----------

$grouped = $rows | Group-Object Dir | Sort-Object { $_.Name }
$navHtml = [System.Text.StringBuilder]::new()
foreach ($g in $grouped) {
    $label = switch ($g.Name) {
        "01_getting_started" { "入门" }
        "02_commands" { "基本命令和语法" }
        "03_simple_projects" { "简单项目" }
        "04_super_project" { "超级项目（Markdown 编辑器，React）" }
        default { $g.Name }
    }
    [void]$navHtml.AppendLine("<h2>$label <span class=`"dir`">$($g.Name)/</span></h2>")
    [void]$navHtml.AppendLine('<table><thead><tr><th>题号</th><th>名称</th><th>练习版</th><th>答案版</th></tr></thead><tbody>')
    $items = $g.Group | Sort-Object { [int]($_.Id -replace "[ep]", "") }
    for ($i = 0; $i -lt $items.Count; $i += 2) {
        $ex = $items[$i]; $ans = $items[$i + 1]
        $exDir = "$($ex.Dir)/$($ex.Id)_$($ex.Name)"
        $ansDir = "$($ans.Dir)/$($ans.Id)_$($ans.Name)_answer"
        [void]$navHtml.AppendLine("<tr><td>$($ex.Id.ToUpper())</td><td>$($ex.Title)</td><td><code>$($ex.Port)</code> <span class=`"dir`">$exDir</span></td><td><code>$($ans.Port)</code> <span class=`"dir`">$ansDir</span></td></tr>")
    }
    [void]$navHtml.AppendLine('</tbody></table>')
}

$preface = @"
<!doctype html>
<html lang="zh-CN">
  <head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>Tauri v2 练习导航</title>
    <style>
      body { font-family: system-ui, -apple-system, "Segoe UI", "Microsoft YaHei", sans-serif; max-width: 960px; margin: 0 auto; padding: 2rem 1.5rem; color: #1d1d1f; }
      h1 { font-size: 1.6rem; }
      h2 { font-size: 1.15rem; margin-top: 2rem; border-bottom: 1px solid #e5e5ea; padding-bottom: .4rem; }
      .dir { color: #6e6e73; font-size: .85rem; }
      code { background: #f2f2f4; padding: .1rem .35rem; border-radius: 4px; font-size: .85rem; }
      table { border-collapse: collapse; width: 100%; margin-top: .5rem; }
      th, td { text-align: left; padding: .35rem .5rem; border-bottom: 1px solid #eee; font-size: .9rem; vertical-align: top; }
      th { font-weight: 600; color: #6e6e73; }
      .note { color: #6e6e73; font-size: .9rem; }
    </style>
  </head>
  <body>
    <h1>Tauri v2 练习导航</h1>
    <p class="note">84 个练习 ×（练习版 + 答案版）= 168 个项目 · 端口按次序递增 $($rows[0].Port) – $($rows[-1].Port)</p>
$navHtml
  </body>
</html>
"@
New-Item -ItemType Directory -Path (Join-Path $Root "00_preface") -Force | Out-Null
[System.IO.File]::WriteAllText((Join-Path $Root "00_preface\index.html"), $preface, [System.Text.UTF8Encoding]::new($false))

Write-Host ""
Write-Host "== 完成：workspace 根文件 + 00_preface 导航页已生成 =="