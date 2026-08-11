# 练习 E45: 权限系统（permissions）

## 知识点
- `capabilities/default.json`：窗口 + 权限声明
- 自定义 permission 文件：`src-tauri/permissions/read-notes.toml`
- scope 通配：`$APPDATA/notes/**`
- `fs:default` 与最小权限的取舍

## 运行

```bash
pnpm install
cargo tauri dev
```

## 对照

- devUrl: http://localhost:1509
- identifier: com.taurilearn.e45a
- 练习版: `../e45_permissions/`