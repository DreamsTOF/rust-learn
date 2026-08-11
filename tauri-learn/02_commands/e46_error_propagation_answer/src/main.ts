// ============================================================
// 练习 E46: 自定义错误传播
// 目标: thiserror 错误枚举 + 错误码，前端按 code 分类处理
// 知识点: invoke 错误对象 / 错误码映射 / 统一错误展示（承接 E11）
// ============================================================

import { invoke } from "@tauri-apps/api/core";

// 后端 AppError 经自定义 Serialize 后传到前端就是一个 { code, message } 对象
interface ErrorBody {
  code: number;
  message: string;
}

// 错误码 → 中文提示
const codeMessages: Record<number, string> = {
  400: "参数错误",
  404: "未找到",
  500: "服务异常",
};

const errorEl = document.querySelector<HTMLDivElement>("#error-area");
const okBtn = document.querySelector<HTMLButtonElement>("#ok-btn");
const badBtn = document.querySelector<HTMLButtonElement>("#bad-btn");
const missingBtn = document.querySelector<HTMLButtonElement>("#missing-btn");
const boomBtn = document.querySelector<HTMLButtonElement>("#boom-btn");

// 统一错误处理：捕获 invoke 错误 → 提取 code → 分类提示 → 展示原始信息
// InvokeArgs 即 Record<string, unknown>，args 保持可选
const invokeWithError = async <T>(cmd: string, args?: Record<string, unknown>): Promise<T> => {
  try {
    return await invoke<T>(cmd, args);
  } catch (e) {
    const err = e as ErrorBody;
    const hint = codeMessages[err.code] ?? "未知错误";
    errorEl!.innerHTML =
      `<span class="badge warn">${err.code}</span>` +
      `<span><strong>${hint}</strong>（原始信息: ${err.message}）</span>`;
    throw e;
  }
};

const runOperation = async (kind: string): Promise<void> => {
  errorEl!.innerHTML = "";
  try {
    const message = await invokeWithError<string>("risky_operation", { kind });
    errorEl!.innerHTML = `<span class="badge">✓</span><span><strong>${message}</strong></span>`;
  } catch {
    // 错误已在 invokeWithError 中展示，这里吞掉即可
  }
};

okBtn!.addEventListener("click", () => runOperation("ok"));
badBtn!.addEventListener("click", () => runOperation("bad"));
missingBtn!.addEventListener("click", () => runOperation("missing"));
boomBtn!.addEventListener("click", () => runOperation("boom"));