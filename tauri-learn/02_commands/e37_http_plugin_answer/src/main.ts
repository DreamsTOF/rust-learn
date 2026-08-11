// ============================================================
// 练习 E37: HTTP 插件
// 目标: 使用 @tauri-apps/plugin-http 在前端发起 GET/POST 请求并处理错误
// 知识点: fetch / Headers / 非 2xx 处理 / connectTimeout 超时 / 错误捕获
// ============================================================

import { fetch } from "@tauri-apps/plugin-http";

const urlInput = document.querySelector<HTMLInputElement>("#url");
const getBtn = document.querySelector<HTMLButtonElement>("#get-btn");
const postBtn = document.querySelector<HTMLButtonElement>("#post-btn");
const timeoutBtn = document.querySelector<HTMLButtonElement>("#timeout-btn");
const resultEl = document.querySelector<HTMLParagraphElement>("#result");
const responseEl = document.querySelector<HTMLPreElement>("#response");

// GET：自定义 Headers，检查 res.ok（非 2xx 显示状态码）
getBtn!.addEventListener("click", async () => {
  const url = urlInput!.value.trim() || "https://httpbin.org/get";
  resultEl!.textContent = "GET 请求中...";
  resultEl!.className = "status";
  try {
    const res = await fetch(url, {
      method: "GET",
      headers: { "User-Agent": "tauri-learn", "X-Custom": "hello" },
    });
    if (!res.ok) {
      // 非 2xx 响应：展示状态码（如 httpbin.org/status/404 → HTTP 404）
      resultEl!.textContent = `HTTP ${res.status} ${res.statusText}`;
      resultEl!.className = "status err";
      responseEl!.textContent = await res.text();
      return;
    }
    const text = await res.text(); // 也可用 res.json() 解析 JSON
    resultEl!.textContent = `GET 成功 (${res.status})`;
    resultEl!.className = "status ok";
    responseEl!.textContent = text;
  } catch (e) {
    // 网络不可达、超时、URL 不在 scope 内都会走到这里
    resultEl!.textContent = `请求失败: ${e}`;
    resultEl!.className = "status err";
    responseEl!.textContent = String(e);
  }
});

// POST：JSON 请求体（Content-Type 必须为 application/json）
postBtn!.addEventListener("click", async () => {
  // 默认 URL 为 httpbin.org/get，POST 时自动切换为 /post
  const url = (urlInput!.value.trim() || "https://httpbin.org/post").replace(
    /\/get$/,
    "/post",
  );
  resultEl!.textContent = "POST 请求中...";
  resultEl!.className = "status";
  try {
    const res = await fetch(url, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ name: "tauri" }),
    });
    if (!res.ok) {
      resultEl!.textContent = `HTTP ${res.status} ${res.statusText}`;
      resultEl!.className = "status err";
      responseEl!.textContent = await res.text();
      return;
    }
    const text = await res.text();
    resultEl!.textContent = `POST 成功 (${res.status})`;
    resultEl!.className = "status ok";
    responseEl!.textContent = text;
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
  const url = "http://192.0.2.1/";
  resultEl!.textContent = "请求中（connectTimeout: 3000ms）...";
  resultEl!.className = "status";
  try {
    const res = await fetch(url, { method: "GET", connectTimeout: 3000 });
    resultEl!.textContent = `请求成功 (${res.status})`;
    resultEl!.className = "status ok";
    responseEl!.textContent = await res.text();
  } catch (e) {
    resultEl!.textContent = `请求失败（超时或不可达）: ${e}`;
    resultEl!.className = "status err";
    responseEl!.textContent = String(e);
  }
});