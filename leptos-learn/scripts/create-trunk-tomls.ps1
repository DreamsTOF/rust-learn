# create-trunk-tomls.ps1
# 按端口分配计划为所有练习/答案目录批量创建 Trunk.toml
#
# 规则:
#   - 00_preface   → 固定端口 3000
#   - Ch1-Ch6     → port = 3000 + 练习编号
#   - Ch7         → 跳过 (cargo leptos serve 自动分配)
#   - Ch8         → port = 2955 + 练习编号 (偏移公式)
#   - 所有答案目录  → 对应练习端口 + 1000
#   - projects/ 和 templates/ 不在计划表中, 不处理

$basePath = "C:\code\testruetlearn\leptos-learn"

function Get-ExercisePort {
    param([int]$exerciseNumber, [string]$chapterDirName)

    if ($chapterDirName -match '^08') {
        # Chapter 8: e346->3301, e385->3340
        # 3301 - 346 = 2955
        return 2955 + $exerciseNumber
    }

    # Chapters 01-06: port = 3000 + exerciseNumber
    # e01->3001, e96->3096, e201->3201, e251->3251
    return 3000 + $exerciseNumber
}

# -- 1. 00_preface - 固定端口 --
$prefaceDir = Join-Path $basePath "00_preface"
if (Test-Path $prefaceDir) {
    Set-Content -Path (Join-Path $prefaceDir "Trunk.toml") -Value "[serve]`nport = 3000" -NoNewline
    Write-Host "[OK] 00_preface/              -> port 3000"
}

# -- 2. 章节 01-08（跳过 07_ssr）--
$chapterDirs = @(
    "01_basics",
    "02_signals",
    "03_components",
    "04_async",
    "05_routing",
    "05_router",
    "06_style",
    "08_advanced"
)

$totalCreated = 0
foreach ($chDir in $chapterDirs) {
    $fullPath = Join-Path $basePath $chDir
    if (-not (Test-Path $fullPath)) {
        Write-Host "[SKIP] $chDir/ - 目录不存在"
        continue
    }

    $subDirs = Get-ChildItem -Directory $fullPath

    # 收集编号目录和额外目录
    $numberedDirs = @()
    $extraDirs = @()

    foreach ($item in $subDirs) {
        $dirName = $item.Name
        if ($dirName -match '^e(\d{2,3})_') {
            $numberedDirs += [PSCustomObject]@{
                FullName    = $item.FullName
                Name        = $dirName
                ExerciseNum = [int]$Matches[1]
                IsAnswer    = $dirName -match '_answer$'
            }
        }
        else {
            $extraDirs += $item
        }
    }

    # 处理编号目录
    foreach ($entry in $numberedDirs) {
        $exercisePort = Get-ExercisePort -exerciseNumber $entry.ExerciseNum -chapterDirName $chDir
        $port = $entry.IsAnswer ? ($exercisePort + 1000) : $exercisePort
        Set-Content -Path (Join-Path $entry.FullName "Trunk.toml") -Value "[serve]`nport = $port" -NoNewline
        Write-Host "[OK] $chDir/$($entry.Name)  -> port $port"
        $totalCreated++
    }

    # 处理额外目录 (如 03_components/_answer)
    foreach ($item in $extraDirs) {
        $dirName = $item.Name
        if ($dirName -eq '_answer') {
            # 取该章节第一个答案端口
            $firstAnswerPort = $null
            foreach ($entry in $numberedDirs) {
                if ($entry.IsAnswer) {
                    $firstAnswerPort = (Get-ExercisePort -exerciseNumber $entry.ExerciseNum -chapterDirName $chDir) + 1000
                    break
                }
            }
            if (-not $firstAnswerPort) {
                Write-Host "[SKIP] $chDir/$dirName - 无法推断端口"
                continue
            }
            Set-Content -Path (Join-Path $item.FullName "Trunk.toml") -Value "[serve]`nport = $firstAnswerPort" -NoNewline
            Write-Host "[OK] $chDir/$dirName  -> port $firstAnswerPort (合并答案)"
            $totalCreated++
        }
    }
}

# -- 汇总 --
Write-Host ""
Write-Host "======= 完成 ======="
Write-Host "共创建 $totalCreated 个 Trunk.toml 文件"
Write-Host ""
Write-Host "未处理的目录:"
Write-Host "  - 07_ssr/          (cargo leptos serve 自动分配)"
Write-Host "  - projects/        (不在端口分配计划中)"
Write-Host "  - templates/       (不在端口分配计划中)"
