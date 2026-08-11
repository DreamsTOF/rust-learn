# 练习 E30: 文件系统（fs_plugin）

## 知识点

- @tauri-apps/plugin-fs：readTextFile / writeTextFile / readDir / stat / exists / copyFile / rename / remove
- scope 限制：fs:default 只读应用数据目录；写入需 fs:allow-appdata-write-recursive，元信息需 fs:allow-appdata-meta-recursive
- @tauri-apps/api/path 的 appDataDir / join 组合路径

## 运行

```bash
pnpm install
cargo tauri dev
```

## 操作验证

1. 写入 demo.txt → 读取显示内容 → 查看 stat / exists
2. 列出 appDataDir 目录内容
3. 一键操作链：copy → rename → remove

对照练习版: `../e30_fs_plugin/`