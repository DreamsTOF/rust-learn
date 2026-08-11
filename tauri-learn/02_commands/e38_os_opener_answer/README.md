# 练习 E38: OS 与 Opener

**知识点：** 插件注册（Rust + capabilities）/ `platform` / `version` / `arch` / `type` / `family` / `openUrl` / `revealItemInDir`

## 运行

```bash
pnpm install
cargo tauri dev
```

## 操作

1. 页面加载即渲染系统信息表格（platform / type / arch / version / family）
2. 「打开 URL」→ 默认浏览器打开 https://tauri.app
3. 「在资源管理器中显示文件」→ 定位 appDataDir 下的 `reveal-demo.txt`（不存在时打开所在目录）

## 说明

- `platform()/type()/arch()/version()/family()` 均为同步 API（v2 中 family 也是同步）
- `type` 与 TS 关键字重名，导入时需重命名：`type as osType`
- `appDataDir()` 来自 `@tauri-apps/api/path`，返回带结尾分隔符的目录路径
- 对比答案: `e38_os_opener_answer/`

## 信息

- devUrl: http://localhost:1494
- identifier: com.taurilearn.e38