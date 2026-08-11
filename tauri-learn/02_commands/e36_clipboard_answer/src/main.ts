// ============================================================
// 练习 E36: 剪贴板
// 目标: 使用 @tauri-apps/plugin-clipboard-manager 读写与清空系统剪贴板
// 知识点: writeText / readText / clear / 结果展示
// ============================================================

import { writeText, readText, clear } from "@tauri-apps/plugin-clipboard-manager";

const textInput = document.querySelector<HTMLInputElement>("#text");
const writeBtn = document.querySelector<HTMLButtonElement>("#write-btn");
const readBtn = document.querySelector<HTMLButtonElement>("#read-btn");
const clearBtn = document.querySelector<HTMLButtonElement>("#clear-btn");
const resultEl = document.querySelector<HTMLParagraphElement>("#result");
const clipEl = document.querySelector<HTMLPreElement>("#clipboard");

// 写入剪贴板（可在其他应用如记事本中 Ctrl+V 粘贴验证）
writeBtn!.addEventListener("click", async () => {
  const text = textInput!.value.trim();
  if (!text) {
    resultEl!.textContent = "请输入要写入的内容";
    resultEl!.className = "status err";
    return;
  }
  try {
    await writeText(text);
    resultEl!.textContent = `已写入剪贴板: ${text}`;
    resultEl!.className = "status ok";
    clipEl!.textContent = text;
  } catch (e) {
    resultEl!.textContent = `写入失败: ${e}`;
    resultEl!.className = "status err";
  }
});

// 读取剪贴板文本
readBtn!.addEventListener("click", async () => {
  try {
    const text = await readText();
    clipEl!.textContent = text || "（空）";
    resultEl!.textContent = "已读取剪贴板内容";
    resultEl!.className = "status ok";
  } catch (e) {
    resultEl!.textContent = `读取失败: ${e}`;
    resultEl!.className = "status err";
  }
});

// 清空剪贴板
clearBtn!.addEventListener("click", async () => {
  try {
    await clear();
    clipEl!.textContent = "（已清空）";
    resultEl!.textContent = "剪贴板已清空";
    resultEl!.className = "status ok";
  } catch (e) {
    resultEl!.textContent = `清空失败: ${e}`;
    resultEl!.className = "status err";
  }
});