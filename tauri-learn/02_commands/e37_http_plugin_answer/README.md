# 练习 E37: HTTP 插件

**知识点：** 插件注册（Rust + capabilities + URL scope）/ `fetch` GET/POST / Headers / 非 2xx 处理 / `connectTimeout` 超时 / 错误捕获

## 运行

```bash
pnpm install
cargo tauri dev
```

## 操作

1. GET：默认请求 `https://httpbin.org/get`（自定义 User-Agent / X-Custom Headers）
2. 非 2xx 演示：URL 改为 `https://httpbin.org/status/404` → 显示 `HTTP 404`
3. POST：自动改用 `httpbin.org/post`，发送 JSON body `{ "name": "tauri" }`
4. 超时演示：请求保留地址 `http://192.0.2.1/`（正常网络不可达），3 秒连接超时后 catch 展示错误
5. 错误处理：网络不可达 / 超时 / URL 不在 scope 内都会进入 catch

## 说明

- v2 中超时字段为 `connectTimeout`（毫秒，仅连接阶段）；`timeout` / `readTimeout` 是 v1 字段
- URL scope 在 `src-tauri/capabilities/default.json` 中配置（当前允许全部 http/https，可收紧）
- 对比答案: `e37_http_plugin_answer/`

## 信息

- devUrl: http://localhost:1492
- identifier: com.taurilearn.e37