// ============================================================
// 练习 E14: 可变状态
// 目标: 操作由 manage() 注入的多个可变状态
// TODO: 按照注释提示补全代码
// ============================================================

// TODO: 完成填空后取消注释（invoke 用于调用后端命令）
// import { invoke } from "@tauri-apps/api/core";

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
    // === 步骤 7: 获取标签列表 ————————————————————————————————————
    // TODO: const tags = await invoke<string[]>("get_tags");
    // 提示: 返回的数组直接用于渲染
    // 占位：完成填空后替换为真实调用结果
    const tags: string[] = [];
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
    // === 步骤 5: 计数 +1 ————————————————————————————————————
    // TODO: const n = await invoke<number>("increment");
    // 占位：完成填空后替换为真实调用结果
    const n: number = 0;
    counterEl!.textContent = String(n);
  } catch (e) {
    resultEl!.textContent = `调用失败: ${e}`;
    resultEl!.className = "status err";
  }
});

addTagBtn!.addEventListener("click", async () => {
  try {
    // === 步骤 6: 添加标签 ————————————————————————————————————
    // TODO: const tag = tagInput!.value.trim(); if (!tag) return;
    //       const count = await invoke<number>("add_tag", { tag });
    // 提示: 返回值为当前标签数量；添加后刷新列表
    // 占位：完成填空后替换为真实调用结果
    const count: number = 0;
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
    // === 步骤 8: 清空标签 ————————————————————————————————————
    // TODO: await invoke("clear_tags"); 然后刷新列表
    // 占位：完成填空后删除
    await new Promise<void>((resolve) => setTimeout(resolve, 0));
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