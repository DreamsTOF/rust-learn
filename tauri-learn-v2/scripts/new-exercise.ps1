# ============================================================
# new-exercise.ps1 — 生成单个 Tauri v2 练习项目（练习版/答案版）
#
# 用法（独立运行）:
#   pwsh scripts/new-exercise.ps1 -Chapter "02_commands" -Id "e10" `
#       -Name "dependency_injection" -Title "依赖注入" -Port 1422
#   pwsh scripts/new-exercise.ps1 ... -Answer          # 答案版
#   pwsh scripts/new-exercise.ps1 ... -React           # React 前端
#
# 也可被 init-all.ps1 dot-source 后以 New-Exercise 函数方式进程内调用。
# ============================================================

param(
    [string]$Chapter,   # 例如 "02_commands"
    [string]$Id,        # 例如 "e10" / "p01"
    [string]$Name,      # 例如 "dependency_injection"
    [string]$Title,     # 例如 "依赖注入"（中文）
    [int]$Port,         # devUrl 端口，全 workspace 唯一
    [switch]$Answer,    # 答案版（目录名加 _answer）
    [switch]$React      # React 前端模板（超级项目）
)

$ErrorActionPreference = "Stop"

# ---------- 工具 ----------

function Write-Utf8File {
    param([string]$Path, [string]$Content)
    $dir = Split-Path $Path -Parent
    New-Item -ItemType Directory -Path $dir -Force | Out-Null
    [System.IO.File]::WriteAllText($Path, $Content, [System.Text.UTF8Encoding]::new($false))
}

# ---------- 图标 ----------

# 纯字节构造 ICO（BMP 内嵌，无外部依赖）：深蓝底 + 白色圆占位图标
function New-IcoBytes {
    param([int[]]$Sizes = @(16, 32, 48))
    $payloads = [System.Collections.Generic.List[byte[]]]::new()
    $lens = [System.Collections.Generic.List[int]]::new()
    foreach ($s in $Sizes) {
        $xor = New-Object byte[] ($s * $s * 4)
        $cx = $s / 2.0; $cy = $s / 2.0; $r = $s * 0.28
        for ($y = 0; $y -lt $s; $y++) {
            for ($x = 0; $x -lt $s; $x++) {
                $idx = ($y * $s + $x) * 4
                $dx = $x + 0.5 - $cx; $dy = $y + 0.5 - $cy
                if ($dx * $dx + $dy * $dy -le $r * $r) {
                    $xor[$idx + 0] = 0xFF; $xor[$idx + 1] = 0xFF; $xor[$idx + 2] = 0xFF; $xor[$idx + 3] = 0xFF
                } else {
                    $xor[$idx + 0] = 0xE5; $xor[$idx + 1] = 0x46; $xor[$idx + 2] = 0x4F; $xor[$idx + 3] = 0xFF
                }
            }
        }
        $maskRow = [math]::Ceiling($s / 8.0)
        $maskRowPad = [math]::Ceiling($maskRow / 4.0) * 4
        $and = New-Object byte[] ($maskRowPad * $s)  # 全 0 = 不透明
        $ms = [System.IO.MemoryStream]::new()
        $bw = [System.IO.BinaryWriter]::new($ms)
        $bw.Write([int32]40); $bw.Write([int32]$s); $bw.Write([int32]($s * 2))
        $bw.Write([int16]1); $bw.Write([int16]32)
        $bw.Write([int32]0); $bw.Write([int32]($s * $s * 4)); $bw.Write([int32]0)
        $bw.Write([int32]0); $bw.Write([int32]0); $bw.Write([int32]0)
        $bw.Flush()
        $hdr = $ms.ToArray()
        $payload = New-Object byte[] ($hdr.Length + $xor.Length + $and.Length)
        [Array]::Copy($hdr, 0, $payload, 0, $hdr.Length)
        [Array]::Copy($xor, 0, $payload, $hdr.Length, $xor.Length)
        [Array]::Copy($and, 0, $payload, $hdr.Length + $xor.Length, $and.Length)
        $payloads.Add($payload)
        $lens.Add($payload.Length)
    }
    $ms = [System.IO.MemoryStream]::new()
    $bw = [System.IO.BinaryWriter]::new($ms)
    $bw.Write([uint16]0); $bw.Write([uint16]1); $bw.Write([uint16]$Sizes.Count)
    $off = 6 + 16 * $Sizes.Count
    for ($i = 0; $i -lt $Sizes.Count; $i++) {
        $s = $Sizes[$i]
        $bw.Write([byte]$s); $bw.Write([byte]$s); $bw.Write([byte]0); $bw.Write([byte]0)
        $bw.Write([uint16]1); $bw.Write([uint16]32)
        $bw.Write([uint32]$lens[$i]); $bw.Write([uint32]$off)
        $off += $lens[$i]
    }
    foreach ($p in $payloads) { $bw.Write($p) }
    $bw.Flush()
    return $ms.ToArray()
}

# System.Drawing 生成占位 PNG（失败时返回 $false，调用方降级 bundle.icon）
function New-PngFile {
    param([string]$Path, [int]$Size)
    try {
        Add-Type -AssemblyName System.Drawing
        $bmp = [System.Drawing.Bitmap]::new($Size, $Size)
        $g = [System.Drawing.Graphics]::FromImage($bmp)
        $g.SmoothingMode = [System.Drawing.Drawing2D.SmoothingMode]::AntiAlias
        $g.Clear([System.Drawing.Color]::FromArgb(255, 79, 70, 229))
        $r = [float]($Size * 0.28)
        $brush = [System.Drawing.SolidBrush]::new([System.Drawing.Color]::White)
        $g.FillEllipse($brush, ($Size / 2 - $r), ($Size / 2 - $r), ($r * 2), ($r * 2))
        $g.Dispose(); $brush.Dispose()
        $bmp.Save($Path, [System.Drawing.Imaging.ImageFormat]::Png)
        $bmp.Dispose()
        return $true
    } catch {
        Write-Warning "PNG 图标生成失败（$Path）: $($_.Exception.Message)"
        return $false
    }
}

# 图标缓存（scripts/.icon-cache/），所有项目共用同一份占位图标
function Ensure-IconCache {
    param([string]$CacheDir)
    if (-not (Test-Path "$CacheDir\icon.ico")) {
        New-Item -ItemType Directory -Path $CacheDir -Force | Out-Null
        [System.IO.File]::WriteAllBytes("$CacheDir\icon.ico", (New-IcoBytes))
        $okPng = $true
        if (-not (New-PngFile "$CacheDir\icon.png" 256)) { $okPng = $false }
        if (-not (New-PngFile "$CacheDir\128x128@2x.png" 256)) { $okPng = $false }
        if (-not (New-PngFile "$CacheDir\128x128.png" 128)) { $okPng = $false }
        if (-not (New-PngFile "$CacheDir\32x32.png" 32)) { $okPng = $false }
        if ($okPng) {
            Set-Content -Path "$CacheDir\.png-ok" -Value "ok" -Encoding utf8NoBOM
        }
    }
    return (Test-Path "$CacheDir\.png-ok")
}

# ---------- 主生成函数 ----------

function New-Exercise {
    param(
        [string]$Chapter,
        [string]$Id,
        [string]$Name,
        [string]$Title,
        [int]$Port,
        [switch]$Answer,
        [switch]$React
    )
    if (-not $Chapter -or -not $Id -or -not $Name -or $Port -le 0) {
        throw "缺少必要参数: -Chapter / -Id / -Name / -Port"
    }

    $Root = Split-Path $PSScriptRoot -Parent
    $suffix = if ($Answer) { "_answer" } else { "" }
    $display = $Id.ToUpper()
    $kind = if ($Id.StartsWith("p")) { "超级项目" } else { "练习" }
    $windowTitle = "$kind ${display}: $Title$(if ($Answer) { '（答案）' } else { '' })"
    $crateName = "$Id-$Name$(if ($Answer) { '-answer' } else { '' })"
    $libName = "${Id}_${Name}$(if ($Answer) { '_answer' } else { '' })_lib"
    $identifier = "com.taurilearn.$Id$(if ($Answer) { 'a' } else { '' })"
    $projectDir = Join-Path $Root (Join-Path $Chapter "${Id}_${Name}$suffix")

    # 图标（缓存复用）
    $cacheDir = Join-Path $PSScriptRoot ".icon-cache"
    $hasPng = Ensure-IconCache -CacheDir $cacheDir
    $iconDir = Join-Path $projectDir "src-tauri\icons"
    New-Item -ItemType Directory -Path $iconDir -Force | Out-Null
    Copy-Item (Join-Path $cacheDir "icon.ico") (Join-Path $iconDir "icon.ico") -Force
    if ($hasPng) {
        Copy-Item (Join-Path $cacheDir "icon.png") (Join-Path $iconDir "icon.png") -Force
        Copy-Item (Join-Path $cacheDir "128x128@2x.png") (Join-Path $iconDir "128x128@2x.png") -Force
        Copy-Item (Join-Path $cacheDir "128x128.png") (Join-Path $iconDir "128x128.png") -Force
        Copy-Item (Join-Path $cacheDir "32x32.png") (Join-Path $iconDir "32x32.png") -Force
    }

    # ---------- 模板 ----------

    $tpl = @{}

    $tpl["package.json"] = @'
{
  "name": "__CRATE__",
  "private": true,
  "version": "0.1.0",
  "type": "module",
  "scripts": {
    "dev": "vite",
    "build": "tsc && vite build",
    "preview": "vite preview"
  },
  "dependencies": {
    __FRONTDEPS__
  },
  "devDependencies": {
    __FRONTDEVDEPS__
  }
}
'@

    $tpl["tsconfig.json"] = @'
{
  "compilerOptions": {
    "target": "ES2021",
    "useDefineForClassFields": true,
    "module": "ESNext",
    "lib": ["ES2021", "DOM", "DOM.Iterable"],
    "skipLibCheck": true,
    "moduleResolution": "bundler",
    "allowImportingTsExtensions": true,
    "resolveJsonModule": true,
    "isolatedModules": true,
    "noEmit": true,
    "strict": true,
    "noUnusedLocals": true,
    "noUnusedParameters": true,
    "noFallthroughCasesInSwitch": true
  },
  "include": ["src"]
}
'@

    $tpl["vite.config.ts"] = @'
import { defineConfig } from "vite";
__REACT_IMPORT__
const host = process.env.TAURI_DEV_HOST;

export default defineConfig({
  __REACT_PLUGINS__clearScreen: false,
  server: {
    port: __PORT__,
    strictPort: true,
    host: host || false,
    hmr: host ? { protocol: "ws", host, port: 1421 } : undefined,
    watch: { ignored: ["**/src-tauri/**"] },
  },
});
'@

    $tpl["index.html"] = @'
<!doctype html>
<html lang="zh-CN">
  <head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>__WINDOW_TITLE__</title>
  </head>
  <body>
    <div id="__APP_ID__"></div>
    <script type="module" src="/src/__ENTRY__"></script>
  </body>
</html>
'@

    $tpl["src/main.ts"] = @'
// ============================================================
// __KIND__ __ID__: __TITLE__
// 目标: 由练习 Agent 按规划文档编写
// 状态: 项目骨架（由 scripts/ 初始化脚本生成）
// ============================================================

const app = document.querySelector<HTMLDivElement>("#app");

if (app) {
  app.innerHTML = `
    <h1>__KIND__ __ID__: __TITLE__</h1>
    <p>项目骨架已就绪，等待练习内容。</p>
  `;
}
'@

    $tpl["src/main.tsx"] = @'
// ============================================================
// __KIND__ __ID__: __TITLE__
// 目标: 由练习 Agent 按规划文档编写
// 状态: 项目骨架（由 scripts/ 初始化脚本生成）
// ============================================================

import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import "./styles.css";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>
);
'@

    $tpl["src/App.tsx"] = @'
// ============================================================
// __KIND__ __ID__: __TITLE__
// 目标: 由练习 Agent 按规划文档编写
// 状态: 项目骨架（由 scripts/ 初始化脚本生成）
// ============================================================

export default function App() {
  return (
    <main>
      <h1>__KIND__ __ID__: __TITLE__</h1>
      <p>项目骨架已就绪，等待练习内容。</p>
    </main>
  );
}
'@

    $tpl["src/styles.css"] = @'
:root {
  font-family: system-ui, -apple-system, "Segoe UI", "Microsoft YaHei", sans-serif;
  color-scheme: light dark;
}

body {
  margin: 0;
  min-height: 100vh;
  display: flex;
  align-items: center;
  justify-content: center;
  background: #f5f5f7;
}

main {
  text-align: center;
  padding: 2rem;
}

h1 {
  font-size: 1.5rem;
  color: #1d1d1f;
}

p {
  color: #6e6e73;
}
'@

    $tpl["src-tauri/Cargo.toml"] = @'
[package]
name = "__CRATE__"
version = "0.1.0"
edition = "2024"

[lib]
name = "__LIB__"
crate-type = ["lib", "cdylib", "staticlib"]

[build-dependencies]
tauri-build = { workspace = true }

[dependencies]
tauri = { workspace = true }
serde = { workspace = true }
serde_json = { workspace = true }
thiserror = { workspace = true }
'@

    $tpl["src-tauri/tauri.conf.json"] = @'
{
  "$schema": "../gen/schemas/desktop-schema.json",
  "productName": "__CRATE__",
  "version": "0.1.0",
  "identifier": "__IDENTIFIER__",
  "build": {
    "frontendDist": "../dist",
    "devUrl": "http://localhost:__PORT__",
    "beforeDevCommand": "pnpm dev",
    "beforeBuildCommand": "pnpm build"
  },
  "app": {
    "windows": [
      {
        "title": "__WINDOW_TITLE__",
        "width": 800,
        "height": 600
      }
    ],
    "security": {
      "csp": null
    }
  },
  "bundle": {
    "active": true,
    "targets": "all",
    "icon": [__ICONS__]
  }
}
'@

    $tpl["src-tauri/build.rs"] = @'
fn main() {
    tauri_build::build()
}
'@

    $tpl["src-tauri/capabilities/default.json"] = @'
{
  "identifier": "default",
  "description": "Default capability for the main window",
  "windows": ["main"],
  "permissions": ["core:default"]
}
'@

    $tpl["src-tauri/src/lib.rs"] = @'
// ============================================================
// __KIND__ __ID__: __TITLE__
// 目标: 由练习 Agent 按规划文档编写
// 状态: 项目骨架（由 scripts/ 初始化脚本生成）
// ============================================================

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("启动 Tauri 应用失败");
}
'@

    $tpl["src-tauri/src/main.rs"] = @'
// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    __LIB__::run()
}
'@

    $tpl["README.md"] = @'
# __KIND__ __ID__: __TITLE__

**状态：** 项目骨架（练习 Agent 编写中）

## 运行

```bash
pnpm install
cargo tauri dev
```

## 信息

- devUrl: http://localhost:__PORT__
- identifier: __IDENTIFIER__
'@

    # ---------- 替换占位符 ----------

    if ($React) {
        $tpl["tsconfig.json"] = $tpl["tsconfig.json"].Replace('"target": "ES2021",', '"target": "ES2021",' + "`n    " + '"jsx": "react-jsx",')
        $viteImport = "import react from `"@vitejs/plugin-react`";`n"
        $vitePlugins = "  plugins: [react()]," + "`n" + "  "
    } else {
        $viteImport = ""
        $vitePlugins = ""
    }

    if ($hasPng) {
        $icons = '"icons/icon.ico", "icons/icon.png", "icons/32x32.png", "icons/128x128.png", "icons/128x128@2x.png"'
    } else {
        $icons = '"icons/icon.ico"'
    }

    $values = @{
        CRATE        = $crateName
        LIB          = $libName
        IDENTIFIER   = $identifier
        PORT         = "$Port"
        WINDOW_TITLE = $windowTitle
        ID           = $display
        TITLE        = $Title
        KIND         = $kind
        ICONS        = $icons
    }

    foreach ($rel in $tpl.Keys) {
        $content = $tpl[$rel]
        foreach ($k in $values.Keys) {
            $content = $content.Replace("__${k}__", $values[$k])
        }
        if ($rel -eq "vite.config.ts") {
            $content = $content.Replace("__REACT_IMPORT__", $viteImport).Replace("__REACT_PLUGINS__", $vitePlugins)
        }
        if ($rel -eq "package.json") {
            if ($React) {
                $deps = '"@tauri-apps/api": "^2.11.1",' + "`n    " + '"react": "^19.0.0",' + "`n    " + '"react-dom": "^19.0.0"'
                $devDeps = '"@tauri-apps/cli": "^2.0.0",' + "`n    " + '"@types/react": "^19.0.0",' + "`n    " + '"@types/react-dom": "^19.0.0",' + "`n    " + '"@vitejs/plugin-react": "^4.3.0",' + "`n    " + '"typescript": "^5.7.0",' + "`n    " + '"vite": "^6.0.0"'
            } else {
                $deps = '"@tauri-apps/api": "^2.11.1"'
                $devDeps = '"@tauri-apps/cli": "^2.0.0",' + "`n    " + '"typescript": "^5.7.0",' + "`n    " + '"vite": "^6.0.0"'
            }
            $content = $content.Replace("__FRONTDEPS__", $deps).Replace("__FRONTDEVDEPS__", $devDeps)
        }
        if ($rel -eq "index.html") {
            if ($React) {
                $content = $content.Replace("__APP_ID__", "root").Replace("__ENTRY__", "main.tsx")
            } else {
                $content = $content.Replace("__APP_ID__", "app").Replace("__ENTRY__", "main.ts")
            }
        }
        $outPath = Join-Path $projectDir $rel
        if (($rel -eq "src/main.tsx" -or $rel -eq "src/App.tsx") -and -not $React) { continue }
        if (($rel -eq "src/main.ts") -and $React) { continue }
        Write-Utf8File -Path $outPath -Content $content
    }

    Write-Host ("✓ {0,-42} port={1}" -f ("{0}\{1}_{2}{3}" -f $Chapter, $Id, $Name, $suffix), $Port)
}

# ---------- 独立运行入口 ----------

if ($MyInvocation.InvocationName -ne ".") {
    if (-not $Id -or -not $Name) {
        throw "缺少参数: 请提供 -Chapter -Id -Name -Title -Port"
    }
    New-Exercise -Chapter $Chapter -Id $Id -Name $Name -Title $Title -Port $Port -Answer:$Answer -React:$React
}