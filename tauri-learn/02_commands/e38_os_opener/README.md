# 练习 E38: OS 与 Opener

**知识点：** 插件注册（Rust + capabilities）/ `platform` / `version` / `arch` / `type` / `family` / `openUrl` / `revealItemInDir`

## TODO（练习版）

在 `src/main.ts` 中按注释提示补全：

1. 读取系统信息字段（可补 arch / type / family 等 2-3 个）
2. 渲染系统信息表格
3. `openUrl("https://tauri.app")` 用默认浏览器打开
4. `revealItemInDir` 在资源管理器中显示 appDataDir 下的文件

## 运行

```bash
pnpm install
cargo tauri dev
```

## 说明

- `platform()/type()/arch()/version()/family()` 均为同步 API（v2 中 family 也是同步）
- `type` 与 TS 关键字重名，导入时需重命名：`type as osType`
- 对照答案: `e38_os_opener_answer/`

## 信息

- devUrl: http://localhost:1494
- identifier: com.taurilearn.e38