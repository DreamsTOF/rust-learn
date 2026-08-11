# 练习 E45: 权限系统（permissions）

## 知识点
- `capabilities/default.json`：窗口 + 权限声明（支持 identifier + allow 对象）
- 自定义 permission 文件：`src-tauri/permissions/read-notes.toml`（只接受 identifier 字符串列表）
- scope 收紧：capability 中 `{ "identifier": ..., "allow": [{ "path": "$APPDATA/notes/**" }] }`
- `fs:default` 与最小权限的取舍

## 任务
1. 补全 `src-tauri/permissions/read-notes.toml` 的 permissions 数组（缺读文本文件条目）
2. 在 `capabilities/default.json` 的 permissions 数组加入两个带 `allow` 的权限对象
   （JSON 不支持注释，TODO 说明写在 index.html 的说明卡片里；不加 allow 时 scope 为空，所有 fs 调用都会被拒）
3. 补全 `src/main.ts` 的写笔记 / 读笔记 / 越权演示调用
4. 验证：写/读成功，越权读取被 scope 拒绝（错误含 "path not allowed"）

## 运行

```bash
pnpm install
cargo tauri dev
```

## 对照答案

`../e45_permissions_answer/`（devUrl: http://localhost:1509）