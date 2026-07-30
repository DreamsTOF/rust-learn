 <#
 .SYNOPSIS
     Tauri v2 练习项目脚手架脚本
 
 .DESCRIPTION
     从 vite-ts 模板复制并创建新的练习项目（含答案版）
 
 .PARAMETER Chapter
     章节目录名，如 "02_basics"
 
 .PARAMETER Number
     练习编号，如 1, 2, 3...
 
 .PARAMETER Name
     练习英文名 (kebab-case)，如 "hello_world"
 
 .PARAMETER Title
     练习中文标题，如 "Hello World"
 
 .EXAMPLE
     .\scripts\new-exercise.ps1 -Chapter "02_basics" -Number 11 -Name "hello_world" -Title "Hello World"
 #>
 
 param(
     [Parameter(Mandatory = $true)]
     [string]$Chapter,
 
     [Parameter(Mandatory = $true)]
     [int]$Number,
 
     [Parameter(Mandatory = $true)]
     [string]$Name,
 
     [Parameter(Mandatory = $true)]
     [string]$Title
 )
 
 $ProjectRoot = Split-Path -Parent $PSScriptRoot
 $ChapterDir = Join-Path $ProjectRoot $Chapter
 $NumberStr = $Number.ToString("D2")
 $DevPort = 1420 + $Number
 
 # 练习项目路径
 $ExerciseDir = Join-Path $ChapterDir "e${NumberStr}_${Name}"
 $AnswerDir = Join-Path $ChapterDir "e${NumberStr}_${Name}_answer"
 
 # 模板路径
 $TemplateDir = Join-Path $ProjectRoot "templates\vite-ts"
 
 function New-ExerciseFromTemplate {
     param([string]$TargetDir, [string]$Suffix)
 
     Write-Host "创建项目: $TargetDir" -ForegroundColor Green
 
     # 复制模板
     if (-not (Test-Path $TemplateDir)) {
         Write-Error "模板目录不存在: $TemplateDir"
         exit 1
     }
     Copy-Item -Path $TemplateDir -Destination $TargetDir -Recurse -Force
 
     # 更新 package.json name
     $PackageJson = Join-Path $TargetDir "package.json"
     if (Test-Path $PackageJson) {
         $json = Get-Content $PackageJson -Raw | ConvertFrom-Json
         $json.name = "e${NumberStr}_${Name}$Suffix"
         $json | ConvertTo-Json -Depth 10 | Set-Content $PackageJson
     }
 
     # 更新 Cargo.toml
     $CargoToml = Join-Path $TargetDir "src-tauri\Cargo.toml"
     if (Test-Path $CargoToml) {
         $content = Get-Content $CargoToml -Raw
         $libName = "e${NumberStr}_${Name}$Suffix" -replace "-", "_"
         $content = $content -replace "exercise-template-vite-ts", "e${NumberStr}_${Name}$Suffix"
         $content = $content -replace "exercise_template_vite_ts_lib", "${libName}_lib"
         Set-Content $CargoToml $content
     }
 
     # 更新 tauri.conf.json
     $TauriConf = Join-Path $TargetDir "src-tauri\tauri.conf.json"
     if (Test-Path $TauriConf) {
         $conf = Get-Content $TauriConf -Raw | ConvertFrom-Json
         $conf.productName = "e${NumberStr}_${Name}$Suffix"
         $conf.identifier = "com.taurilearn.e${NumberStr}${Suffix}"
         $conf.build.devUrl = "http://localhost:${DevPort}"
         $conf.app.windows[0].title = "练习 ${NumberStr}: ${Title}$(if ($Suffix -eq '_answer') { ' (答案)' } else { '' })"
         $conf | ConvertTo-Json -Depth 10 | Set-Content $TauriConf
     }
 
     # 更新 index.html title
     $IndexHtml = Join-Path $TargetDir "index.html"
     if (Test-Path $IndexHtml) {
         $html = Get-Content $IndexHtml -Raw
         $html = $html -replace "<title>.*</title>", "<title>练习 ${NumberStr}: ${Title}$(if ($Suffix -eq '_answer') { ' (答案)' } else { '' })</title>"
         Set-Content $IndexHtml $html
     }
 
    # 更新 workspace Cargo.toml members
    $WorkspaceToml = Join-Path $ProjectRoot "Cargo.toml"
    if (Test-Path $WorkspaceToml) {
        $memberPath = "${Chapter}/e${NumberStr}_${Name}${Suffix}/src-tauri"
        $content = Get-Content $WorkspaceToml -Raw
        if ($content -notmatch [regex]::Escape($memberPath)) {
            $content = $content -replace "(members\s*=\s*\[)", "`$1`n    `"${memberPath}`","
             Set-Content $WorkspaceToml $content
         }
     }
 
     Write-Host "  ✓ 完成: $TargetDir" -ForegroundColor Green
 }
 
 # ===== 主流程 =====
 
 # 确保章节目录存在
 if (-not (Test-Path $ChapterDir)) {
     New-Item -ItemType Directory -Path $ChapterDir -Force | Out-Null
     Write-Host "创建章节目录: $ChapterDir" -ForegroundColor Yellow
 }
 
 # 创建练习项目
 New-ExerciseFromTemplate -TargetDir $ExerciseDir -Suffix ""
 
 # 创建答案项目
 New-ExerciseFromTemplate -TargetDir $AnswerDir -Suffix "_answer"
 
 Write-Host ""
 Write-Host "========================================" -ForegroundColor Cyan
 Write-Host "练习 e${NumberStr}_${Name} 创建完成！" -ForegroundColor Cyan
 Write-Host "练习目录: $ExerciseDir" -ForegroundColor Cyan
 Write-Host "答案目录: $AnswerDir" -ForegroundColor Cyan
 Write-Host "开发端口: $DevPort" -ForegroundColor Cyan
 Write-Host "========================================" -ForegroundColor Cyan
 Write-Host ""
 Write-Host "下一步:" -ForegroundColor Yellow
 Write-Host "  1. 填充 $ExerciseDir/src-tauri/src/lib.rs 的 TODO 代码" -ForegroundColor Yellow
 Write-Host "  2. 填充 $ExerciseDir/src/main.ts 的 TODO 代码" -ForegroundColor Yellow
 Write-Host "  3. 填充 $AnswerDir 的完整参考答案" -ForegroundColor Yellow
 Write-Host "  4. cd $ExerciseDir && pnpm install && cargo tauri build --no-bundle" -ForegroundColor Yellow
