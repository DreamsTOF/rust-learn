# ============================================================
# init-all.ps1 — 批量初始化 Tauri v2 学习 workspace（v2 直观版）
#
# 规划依据: .trae/documents/tauri-learn-book-v2.md
# 端口规则: 练习版从 1420 开始 +1，答案版再 +1，全局递增
# 前端模板: 01 阶段与 a01 用 Vanilla TS + Vite；a02 起与超级项目用 React
# 幂等: 已存在的项目跳过（只补生成缺失项目）
#
# 用法: pwsh scripts/init-all.ps1
# ============================================================

$ErrorActionPreference = "Stop"
$Root = Split-Path $PSScriptRoot -Parent

# 加载单项目生成器（dot-source，进程内调用）
. (Join-Path $PSScriptRoot "new-exercise.ps1")

# ---------- 1. 练习清单（阶段 1/2/3） ----------

$Blocks = @(
    @{ Dir = "01_first_app"; Label = "环境与第一个窗口"; Items = @(
        @{ Id = "e01"; Name = "counter"; Title = "计数器" }
    )},
    @{ Dir = "02_mini_apps"; Label = "八道小菜"; Items = @(
        @{ Id = "a01"; Name = "todo";               Title = "待办清单" },
        @{ Id = "a02"; Name = "notepad";            Title = "记事本" },
        @{ Id = "a03"; Name = "pomodoro";           Title = "番茄钟" },
        @{ Id = "a04"; Name = "image_viewer";       Title = "图片查看器" },
        @{ Id = "a05"; Name = "expense_tracker";    Title = "记账本" },
        @{ Id = "a06"; Name = "exchange_rate";      Title = "汇率查询" },
        @{ Id = "a07"; Name = "batch_rename";       Title = "批量重命名" },
        @{ Id = "a08"; Name = "clipboard_history";  Title = "剪贴板历史" }
    )},
    @{ Dir = "03_super_project"; Label = "超级项目（Markdown 编辑器）"; React = $true; Items = @(
        @{ Id = "p01"; Name = "project_init";       Title = "项目初始化" },
        @{ Id = "p02"; Name = "data_model";         Title = "数据模型与存储层" },
        @{ Id = "p03"; Name = "editor_core";        Title = "编辑器核心" },
        @{ Id = "p04"; Name = "file_management";    Title = "文件管理" },
        @{ Id = "p05"; Name = "live_preview";       Title = "实时预览" },
        @{ Id = "p06"; Name = "toolbar";            Title = "工具栏" },
        @{ Id = "p07"; Name = "tabs";               Title = "多文件标签页" },
        @{ Id = "p08"; Name = "undo_redo_stats";    Title = "撤销/重做与字数统计" },
        @{ Id = "p09"; Name = "search_replace";     Title = "搜索替换" },
        @{ Id = "p10"; Name = "autosave";           Title = "自动保存" },
        @{ Id = "p11"; Name = "theme_system";       Title = "主题系统" },
        @{ Id = "p12"; Name = "drag_drop";          Title = "拖放支持" },
        @{ Id = "p13"; Name = "image_management";   Title = "图片管理" },
        @{ Id = "p14"; Name = "toc_navigation";     Title = "目录导航" },
        @{ Id = "p15"; Name = "tray";               Title = "系统托盘" },
        @{ Id = "p16"; Name = "shortcut";           Title = "全局快捷键" },
        @{ Id = "p17"; Name = "multi_window";       Title = "多窗口" },
        @{ Id = "p18"; Name = "clipboard_notify";   Title = "剪贴板与通知" },
        @{ Id = "p19"; Name = "export";             Title = "导出增强" },
        @{ Id = "p20"; Name = "cloud_sync";         Title = "云同步" },
        @{ Id = "p21"; Name = "spell_check";        Title = "拼写检查" },
        @{ Id = "p22"; Name = "settings";           Title = "设置面板" },
        @{ Id = "p23"; Name = "error_logging";      Title = "错误处理与日志" },
        @{ Id = "p24"; Name = "security";           Title = "安全加固" },
        @{ Id = "p25"; Name = "release";            Title = "打包发布" },
        @{ Id = "p26"; Name = "updater_acceptance"; Title = "自动更新与验收" }
    )}
)

# a01 用 Vanilla，a02 起 React（按计划）
$reactFrom = @{ Dir = "02_mini_apps"; Id = "a02" }

# ---------- 2. 批量生成（端口全局递增，跳过已存在） ----------

Write-Host "== 开始批量初始化 =="
$port = 1420
$rows = [System.Collections.Generic.List[object]]::new()

foreach ($block in $Blocks) {
    Write-Host ""
    Write-Host "--- $($block.Dir) ($($block.Label)) ---"
    foreach ($item in $block.Items) {
        $react = [bool]$block.React
        if (-not $react -and $block.Dir -eq $reactFrom.Dir) {
            $react = $item.Id -ge $reactFrom.Id
        }
        $exDir = Join-Path $Root (Join-Path $block.Dir "${item.Id}_${item.Name}")
        $ansDir = Join-Path $Root (Join-Path $block.Dir "${item.Id}_${item.Name}_answer")
        if (-not (Test-Path $exDir)) {
            New-Exercise -Chapter $block.Dir -Id $item.Id -Name $item.Name -Title $item.Title -Port $port -React:$react
        } else {
            Write-Host ("~ 跳过 {0}（已存在）" -f ($block.Dir + "\" + $item.Id + "_" + $item.Name))
        }
        $rows.Add([pscustomobject]@{ Dir = $block.Dir; Id = $item.Id; Name = $item.Name; Title = $item.Title; Port = $port; React = $react; Answer = $false })
        $port++
        if (-not (Test-Path $ansDir)) {
            New-Exercise -Chapter $block.Dir -Id $item.Id -Name $item.Name -Title $item.Title -Port $port -Answer -React:$react
        } else {
            Write-Host ("~ 跳过 {0}（已存在）" -f ($block.Dir + "\" + $item.Id + "_" + $item.Name + "_answer"))
        }
        $rows.Add([pscustomobject]@{ Dir = $block.Dir; Id = $item.Id; Name = $item.Name; Title = $item.Title; Port = $port; React = $react; Answer = $true })
        $port++
    }
}

Write-Host ""
Write-Host "== 已登记 $($rows.Count) 个项目（端口 $($rows[0].Port) - $($rows[-1].Port)）=="

# ---------- 3. 重写 workspace 根 Cargo.toml（members = 实际项目） ----------

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

Write-Host "== 完成：workspace 根 Cargo.toml 已更新 =="
