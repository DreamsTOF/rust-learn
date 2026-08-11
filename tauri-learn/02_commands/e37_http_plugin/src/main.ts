// ============================================================
// 练习 E37: HTTP 插件
// 目标: 使用 @tauri-apps/plugin-http 在前端发起 GET/POST 请求并处理错误
// 知识点: fetch / Headers / 非 2xx 处理 / connectTimeout 超时 / 错误捕获
// TODO: 按照注释提示补全代码
// ============================================================

// TODO: 完成填空后取消注释（fetch 用于发起 HTTP 请求）
// import { fetch } from "@tauri-apps/plugin-http";

const urlInput = document.querySelector<HTMLInputElement>("#url");
const getBtn = document.querySelector<HTMLButtonElement>("#get-btn");
const postBtn = document.querySelector<HTMLButtonElement>("#post-btn");
const timeoutBtn = document.querySelector<HTMLButtonElement>("#timeout-btn");
const resultEl = document.querySelector<HTMLParagraphElement>("#result");
const responseEl = document.querySelector<HTMLPreElement>("#response");

// GET：自定义 Headers，检查 res.ok（非 2xx 显示状态码）
getBtn!.addEventListener("click", async () => {
  resultEl!.textContent = "GET 请求中...";
  resultEl!.className = "status";
  try {
    // === 步骤 1: GET 请求与 Headers ————————————————————————————
    // TODO: 读取 URL 并发起 GET 请求（自定义 Headers）：
    //   const url = urlInput!.value.trim() || "https://httpbin.org/get";
    //   const res = await fetch(url, {
    //     method: "GET",
    //     headers: { "User-Agent": "tauri-learn", "X-Custom": "hello" },
    //   });
    // 提示: fetch 来自 '@tauri-apps/plugin-http'，返回标准 Response 对象
    // === 步骤 2: res.ok 判断与状态码展示 ————————————————————————
    // TODO: 检查响应状态并展示：
    //   if (!res.ok) {
    //     resultEl!.textContent = `HTTP ${res.status}`; // 非 2xx 展示状态码
    //     resultEl!.className = "status err";
    //     return;
    //   }
    //   const text = await res.text(); // 也可用 res.json() 解析 JSON
    //   resultEl!.textContent = `GET 成功 (${res.status})`;
    //   resultEl!.className = "status ok";
    //   responseEl!.textContent = text;
    // 提示: res.ok 为 false 表示非 2xx；用 https://httpbin.org/status/404 可测试
    // 当前为占位（完成填空后替换）
    responseEl!.textContent = "（GET/状态码逻辑待补全）";
    resultEl!.textContent = "GET 逻辑待补全";
    resultEl!.className = "status";
  } catch (e) {
    // === 步骤 5: 错误展示 ————————————————————————————————————
    // TODO: 展示错误信息（网络不可达 / 超时 / URL 不在 scope 内都会走到这里）：
    //   resultEl!.textContent = `请求失败: ${e}`;
    //   resultEl!.className = "status err";
    //   responseEl!.textContent = String(e);
    // 当前为占位（完成填空后替换）
    resultEl!.textContent = "（错误处理逻辑待补全）";
    resultEl!.className = "status err";
  }
});

// POST：JSON 请求体（Content-Type 必须为 application/json）
postBtn!.addEventListener("click", async () => {
  resultEl!.textContent = "POST 请求中...";
  resultEl!.className = "status";
  try {
    // === 步骤 3: POST 请求 ————————————————————————————————————
    // TODO: 读取 URL（默认 httpbin.org/post，自动把 /get 换为 /post）
    //       并发起 POST 请求（JSON 请求体）：
    //   const url = (urlInput!.value.trim() || "https://httpbin.org/post").replace(/\/get$/, "/post");
    //   const res = await fetch(url, {
    //     method: "POST",
    //     headers: { "Content-Type": "application/json" },
    //     body: JSON.stringify({ name: "tauri" }),
    //   });
    //   ...res.ok 判断与展示（参考步骤 2 的 GET 写法）
    // 提示: 用 httpbin.org/post 测试；Content-Type 必须为 application/json
    // 当前为占位（完成填空后替换）
    responseEl!.textContent = "（POST 逻辑待补全）";
    resultEl!.textContent = "POST 逻辑待补全";
    resultEl!.className = "status";
  } catch (e) {
    resultEl!.textContent = `请求失败: ${e}`;
    resultEl!.className = "status err";
    responseEl!.textContent = String(e);
  }
});

// 超时演示：v2 用 connectTimeout（毫秒，仅连接阶段）
timeoutBtn!.addEventListener("click", async () => {
  // 192.0.2.1 是 IANA 保留的文档地址（TEST-NET-1），正常网络不可达，
  // connectTimeout 3000ms 内连接不上即抛错 → 被下方 catch 捕获展示
  resultEl!.textContent = "请求中（connectTimeout: 3000ms）...";
  resultEl!.className = "status";
  try {
    // === 步骤 4: 超时选项 ————————————————————————————————————
    // TODO: 发起带连接超时的请求：
    //   const url = "http://192.0.2.1/";
    //   const res = await fetch(url, { method: "GET", connectTimeout: 3000 });
    //   ...res.ok 判断与展示（参考步骤 2 的 GET 写法）
    // 提示: v2 超时字段为 connectTimeout（毫秒）；timeout/readTimeout 是 v1 字段
    // 当前为占位（完成填空后替换）
    responseEl!.textContent = "（超时请求逻辑待补全）";
    resultEl!.textContent = "超时请求逻辑待补全";
    resultEl!.className = "status";
  } catch (e) {
    // 超时/网络错误会在这里被捕获展示
    resultEl!.textContent = `请求失败（超时或不可达）: ${e}`;
    resultEl!.className = "status err";
    responseEl!.textContent = String(e);
  }
});