# 练习 E37: HTTP 插件

**知识点：** 插件注册（Rust + capabilities + URL scope）/ `fetch` GET/POST / Headers / 非 2xx 处理 / `connectTimeout` 超时 / 错误捕获

## TODO（练习版）

在 `src/main.ts` 中按注释提示补全：

1. GET 请求与自定义 Headers（User-Agent / X-Custom）
2. `res.ok` 判断与非 2xx 状态码展示
3. POST 请求（Content-Type: application/json + JSON body）
4. 超时选项 `connectTimeout: 3000`
5. catch 中的错误展示（网络不可达 / 超时 / scope 拒绝）

## 运行

```bash
pnpm install
cargo tauri dev
```

## 说明

- v2 中超时字段为 `connectTimeout`（毫秒，仅连接阶段）；`timeout` / `readTimeout` 是 v1 字段
- URL scope 在 `src-tauri/capabilities/default.json` 中配置（当前允许全部 http/https，可收紧）
- 对照答案: `e37_http_plugin_answer/`

## 信息

- devUrl: http://localhost:1492
- identifier: com.taurilearn.e37