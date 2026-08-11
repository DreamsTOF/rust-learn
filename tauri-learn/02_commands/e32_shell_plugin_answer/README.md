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

## 操作验证（Windows）

1. echo 正常输出 stdout
2. cmd /C "echo err 1>&2" 观察 stderr
3. ping -n 5（约 4 秒）+ 2 秒超时 → 观察超时错误
4. 未授权命令 not_allowed_cmd_xyz → 观察 scope 拒绝

> 说明：Windows 的 timeout 命令在 stdin 被重定向时立即退出，
> 无法演示超时，因此用 ping 模拟耗时命令。

对照练习版: `../e32_shell_plugin/`