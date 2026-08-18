# ============================================================
# check-all.ps1 — 编译验证全部练习 crate
# 用法: pwsh scripts/check-all.ps1
# ============================================================

$ErrorActionPreference = "Stop"

Write-Host "== cargo check --workspace =="
& cargo check --workspace 2>&1 | Select-String -Pattern '^error|^warning: unused|^warning:.*generated|Finished|Checking|Compiling'
Write-Host "exit: $LASTEXITCODE"
