// ============================================================
// 练习 E13: 命令模块化
// 目标: 调用按模块拆分的命令
// TODO: 按照注释提示补全代码
// ============================================================

// TODO: 完成填空后取消注释（invoke 用于调用后端命令）
// import { invoke } from "@tauri-apps/api/core";

const aInput = document.querySelector<HTMLInputElement>("#a");
const bInput = document.querySelector<HTMLInputElement>("#b");
const textInput = document.querySelector<HTMLInputElement>("#text");
const addBtn = document.querySelector<HTMLButtonElement>("#add-btn");
const subBtn = document.querySelector<HTMLButtonElement>("#sub-btn");
const upperBtn = document.querySelector<HTMLButtonElement>("#upper-btn");
const countBtn = document.querySelector<HTMLButtonElement>("#count-btn");
const mathResult = document.querySelector<HTMLParagraphElement>("#math-result");
const textResult = document.querySelector<HTMLParagraphElement>("#text-result");

addBtn!.addEventListener("click", async () => {
  try {
    // === 步骤 6: 调用 add ————————————————————————————————————
    // TODO: const sum = await invoke<number>("add", { a: Number(aInput!.value), b: Number(bInput!.value) });
    // 提示: 前端仍按命令名调用；模块路径只在 Rust 侧注册时体现
    // 占位：完成填空后替换为真实调用结果（当前先用输入框做本地计算）
    const sum: number = (Number(aInput!.value) || 0) + (Number(bInput!.value) || 0);
    mathResult!.textContent = `a + b = ${sum}`;
    mathResult!.className = "status ok";
  } catch (e) {
    mathResult!.textContent = `调用失败: ${e}`;
    mathResult!.className = "status err";
  }
});

subBtn!.addEventListener("click", async () => {
  try {
    // === 步骤 7: 调用 sub ————————————————————————————————————
    // TODO: const diff = await invoke<number>("sub", { a: Number(aInput!.value), b: Number(bInput!.value) });
    // 占位：完成填空后替换为真实调用结果（当前先用输入框做本地计算）
    const diff: number = (Number(aInput!.value) || 0) - (Number(bInput!.value) || 0);
    mathResult!.textContent = `a - b = ${diff}`;
    mathResult!.className = "status ok";
  } catch (e) {
    mathResult!.textContent = `调用失败: ${e}`;
    mathResult!.className = "status err";
  }
});

upperBtn!.addEventListener("click", async () => {
  try {
    // === 步骤 8: 调用 to_upper ————————————————————————————————————
    // TODO: const s = await invoke<string>("to_upper", { s: textInput!.value });
    // 占位：完成填空后替换为真实调用结果（当前先展示输入框内容）
    const s: string = textInput!.value || "";
    textResult!.textContent = `大写: ${s}`;
    textResult!.className = "status ok";
  } catch (e) {
    textResult!.textContent = `调用失败: ${e}`;
    textResult!.className = "status err";
  }
});

countBtn!.addEventListener("click", async () => {
  try {
    // === 步骤 9: 调用 word_count ————————————————————————————————————
    // TODO: const n = await invoke<number>("word_count", { s: textInput!.value });
    // 占位：完成填空后替换为真实调用结果
    const n: number = 0;
    textResult!.textContent = `单词数: ${n}`;
    textResult!.className = "status ok";
  } catch (e) {
    textResult!.textContent = `调用失败: ${e}`;
    textResult!.className = "status err";
  }
});