// ============================================================
// 练习 E13: 命令模块化
// 目标: 调用按模块拆分的命令
// 知识点: invoke 按命令名调用（模块路径只在 Rust 侧体现）
// ============================================================

import { invoke } from "@tauri-apps/api/core";

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
    const sum = await invoke<number>("add", {
      a: Number(aInput!.value),
      b: Number(bInput!.value),
    });
    mathResult!.textContent = `a + b = ${sum}`;
    mathResult!.className = "status ok";
  } catch (e) {
    mathResult!.textContent = `调用失败: ${e}`;
    mathResult!.className = "status err";
  }
});

subBtn!.addEventListener("click", async () => {
  try {
    const diff = await invoke<number>("sub", {
      a: Number(aInput!.value),
      b: Number(bInput!.value),
    });
    mathResult!.textContent = `a - b = ${diff}`;
    mathResult!.className = "status ok";
  } catch (e) {
    mathResult!.textContent = `调用失败: ${e}`;
    mathResult!.className = "status err";
  }
});

upperBtn!.addEventListener("click", async () => {
  try {
    const s = await invoke<string>("to_upper", { s: textInput!.value });
    textResult!.textContent = `大写: ${s}`;
    textResult!.className = "status ok";
  } catch (e) {
    textResult!.textContent = `调用失败: ${e}`;
    textResult!.className = "status err";
  }
});

countBtn!.addEventListener("click", async () => {
  try {
    const n = await invoke<number>("word_count", { s: textInput!.value });
    textResult!.textContent = `单词数: ${n}`;
    textResult!.className = "status ok";
  } catch (e) {
    textResult!.textContent = `调用失败: ${e}`;
    textResult!.className = "status err";
  }
});