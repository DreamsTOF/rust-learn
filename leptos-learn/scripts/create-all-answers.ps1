param(
    [string]$WorkspaceRoot = (Resolve-Path "$PSScriptRoot\..")
)

# Read all exercise member paths from Cargo.toml
$cargoPath = Join-Path $WorkspaceRoot "Cargo.toml"
$cargoContent = Get-Content $cargoPath

$members = @()
$inMembers = $false
foreach ($line in $cargoContent) {
    if ($line -match '^members = \[') { $inMembers = $true; continue }
    if ($inMembers -and $line -match '^\]') { break }
    if ($inMembers -and $line -match '^\s+"(.+)"') {
        $members += $matches[1]
    }
}

$created = 0
$skipped = 0
foreach ($member in $members) {
    # Only process answer members
    if (-not $member.EndsWith("_answer")) { continue }
    
    $answerDir = Join-Path $WorkspaceRoot $member
    $srcDir = Join-Path $answerDir "src"
    
    if (Test-Path $answerDir) {
        $skipped++
        continue
    }
    
    # Create directory
    New-Item -ItemType Directory -Path $srcDir -Force | Out-Null
    
    # Extract the exercise number and name from the path
    $parts = $member -split '/'
    $folderName = $parts[-1]  # e.g. "e01_hello_world_answer"
    $chapterDir = $parts[0]   # e.g. "01_basics"
    $pkgName = $folderName
    
    $number = if ($folderName -match 'e(\d+)_') { $matches[1] } else { "??" }
    
    # Generate Cargo.toml
    $cargo = @"
[package]
name = "$pkgName"
version = "0.1.0"
edition = "2021"

[dependencies]
leptos.workspace = true
"@
    $cargo | Out-File -FilePath (Join-Path $answerDir "Cargo.toml") -Encoding UTF8
    
    # Generate index.html
    $index = @"
<!DOCTYPE html>
<html lang="zh-CN">
<head>
    <meta charset="utf-8"/>
    <meta name="viewport" content="width=device-width, initial-scale=1.0"/>
    <title>练习 $number — 参考答案</title>
</head>
<body></body>
</html>
"@
    $index | Out-File -FilePath (Join-Path $answerDir "index.html") -Encoding UTF8
    
    # Generate placeholder main.rs
    $placeholder = @"
// ============================================================
// Exercise $number - Answer
// ============================================================

use leptos::prelude::*;

#[component]
fn Exercise() -> impl IntoView {
    view! {
        <div>
            <p>"Exercise $number - TODO: fill answer"</p>
        </div>
    }
}

fn main() {
    mount_to_body(Exercise);
}
"@
    $placeholder | Out-File -FilePath (Join-Path $srcDir "main.rs") -Encoding UTF8
    
    $created++
}

Write-Host "=== Answer scaffold creation complete ==="
Write-Host "Created: $created answer folders"
Write-Host "Skipped (already exist): $skipped"
