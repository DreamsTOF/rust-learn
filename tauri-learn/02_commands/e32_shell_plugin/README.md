# 练习 E32: Shell（shell_plugin）

## 知识点

- @tauri-apps/plugin-shell：Command.create(程序, 参数).execute()
- Output 对象：code / stdout / stderr
- execute({ timeout }) 超时终止；scope 白名单外的命令被拒绝
- capabilities 中仅 echo / cmd 被允许（shell:allow-execute + allow 列表）

## 运行

```bash
pnpm install
cargo tauri dev
```

## 填空任务（src/main.ts）

1. echo 的 Command.create + execute
2. 结果字段读取（code / stdout / stderr）
3. stderr 重定向命令（cmd /C "echo err 1>&2"）
4. execute({ timeout: 2000 }) 超时演示
5. 未授权命令的 scope 拒绝

对照答案: `../e32_shell_plugin_answer/`