// ============================================================
// 练习 E20: 单实例
// 目标: 读取实例 ID，验证重复启动被拦截
// 知识点: invoke 查询 / State 注入 / 单实例行为验证
// ============================================================

import { invoke } from "@tauri-apps/api/core";

const loadBtn = document.querySelector<HTMLButtonElement>("#load-btn");
const idEl = document.querySelector<HTMLSpanElement>("#instance-id");
const resultEl = document.querySelector<HTMLParagraphElement>("#result");

loadBtn!.addEventListener("click", async () => {
  try {
    const id = await invoke<string>("get_instance_id");
    idEl!.textContent = id;
    resultEl!.textContent = "实例 ID 已读取（再次启动应用会拿到同样的 ID）";
    resultEl!.className = "status ok";
  } catch (e) {
    resultEl!.textContent = `调用失败: ${e}`;
    resultEl!.className = "status err";
  }
});