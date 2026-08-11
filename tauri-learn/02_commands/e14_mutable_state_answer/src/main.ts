// ============================================================
// 练习 E14: 可变状态
// 目标: 操作由 manage() 注入的多个可变状态
// 知识点: invoke 调用 / 列表渲染
// ============================================================

import { invoke } from "@tauri-apps/api/core";

const counterBtn = document.querySelector<HTMLButtonElement>("#counter-btn");
const counterEl = document.querySelector<HTMLSpanElement>("#counter");
const tagInput = document.querySelector<HTMLInputElement>("#tag-input");
const addTagBtn = document.querySelector<HTMLButtonElement>("#add-tag-btn");
const clearBtn = document.querySelector<HTMLButtonElement>("#clear-btn");
const refreshBtn = document.querySelector<HTMLButtonElement>("#refresh-btn");
const tagList = document.querySelector<HTMLUListElement>("#tag-list");
const resultEl = document.querySelector<HTMLParagraphElement>("#result");

// 刷新标签列表
async function refreshTags() {
  try {
    const tags = await invoke<string[]>("get_tags");
    tagList!.innerHTML = tags.length
      ? tags
          .map((t, i) => `<li class="ok"><span class="badge">${i + 1}</span>${t}</li>`)
          .join("")
      : `<li>（暂无标签）</li>`;
  } catch (e) {
    resultEl!.textContent = `err: ${e}`;
    resultEl!.className = "status err";
  }
}

counterBtn!.addEventListener("click", async () => {
  try {
    const n = await invoke<number>("increment");
    counterEl!.textContent = String(n);
  } catch (e) {
    resultEl!.textContent = `调用失败: ${e}`;
    resultEl!.className = "status err";
  }
});

addTagBtn!.addEventListener("click", async () => {
  const tag = tagInput!.value.trim();
  if (!tag) return;
  try {
    const count = await invoke<number>("add_tag", { tag });
    tagInput!.value = "";
    resultEl!.textContent = `当前标签数: ${count}`;
    resultEl!.className = "status ok";
    refreshTags();
  } catch (e) {
    resultEl!.textContent = `调用失败: ${e}`;
    resultEl!.className = "status err";
  }
});

clearBtn!.addEventListener("click", async () => {
  try {
    await invoke("clear_tags");
    resultEl!.textContent = "标签已清空";
    resultEl!.className = "status ok";
    refreshTags();
  } catch (e) {
    resultEl!.textContent = `调用失败: ${e}`;
    resultEl!.className = "status err";
  }
});

refreshBtn!.addEventListener("click", refreshTags);

refreshTags();