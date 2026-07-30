param(
    [Parameter(Mandatory)] [string]$Chapter,   # e.g. "01_basics"
    [Parameter(Mandatory)] [int]$Number,       # e.g. 21
    [Parameter(Mandatory)] [string]$Name,      # e.g. "signal_create"
    [string]$Difficulty = "⭐",                # ⭐ / ⭐⭐ / ⭐⭐⭐
    [string]$Template = "csr"                  # csr / ssr
)

$ExerciseDir = Join-Path $PSScriptRoot ".." "$Chapter\e$('{0:D2}' -f $Number)_$Name"
$ExerciseDir = Resolve-Path $ExerciseDir -ErrorAction SilentlyContinue
if (-not $ExerciseDir) {
    $ExerciseDir = Join-Path $PSScriptRoot ".." "$Chapter\e$('{0:D2}' -f $Number)_$Name"
}

# 1. 创建目录结构
New-Item -ItemType Directory -Path "$ExerciseDir\src" -Force | Out-Null

# 2. 生成 Cargo.toml
$CargoContent = @"
[package]
name = "e$('{0:D2}' -f $Number)_$Name"
version = "0.1.0"
edition = "2024"

[dependencies]
"@

if ($Template -eq "csr") {
    $CargoContent += @"
leptos.workspace = true
"@
} else {
    $CargoContent += @"
leptos.workspace = true
leptos_router.workspace = true
serde.workspace = true
"@
}

$CargoContent | Out-File -FilePath "$ExerciseDir\Cargo.toml" -Encoding UTF8

# 3. 生成 index.html
$IndexContent = @"
<!DOCTYPE html>
<html lang="zh-CN">
<head>
    <meta charset="utf-8"/>
    <meta name="viewport" content="width=device-width, initial-scale=1.0"/>
    <title>练习 $Number</title>
</head>
<body></body>
</html>
"@
$IndexContent | Out-File -FilePath "$ExerciseDir\index.html" -Encoding UTF8

# 4. 生成 main.rs 占位
$MainContent = @"
// ============================================================
// 练习 $Number
//
// TODO: 根据题目要求补全代码
// ============================================================

use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    view! {
        <div>
            <p>"练习 $Number - $Name"</p>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
"@
$MainContent | Out-File -FilePath "$ExerciseDir\src\main.rs" -Encoding UTF8

Write-Host "✓ 创建: $ExerciseDir"
Write-Host "  章节: $Chapter"
Write-Host "  编号: $Number"
Write-Host "  名称: $Name"
Write-Host "  模板: $Template"
