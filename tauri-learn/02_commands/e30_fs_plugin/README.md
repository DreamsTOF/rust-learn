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

## 填空任务（src/main.ts）

1. writeTextFile 写入 demo.txt
2. readTextFile 读取内容
3. stat / exists 查看文件信息
4. readDir 列出目录并渲染
5. copyFile → rename → remove 操作链

对照答案: `../e30_fs_plugin_answer/`