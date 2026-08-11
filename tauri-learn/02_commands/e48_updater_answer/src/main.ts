// ============================================================
// 练习 E48: 自动更新（updater）
// 目标: 集成 updater 插件并实现"检查更新"命令
// 知识点: 更新流程 / endpoints 模板变量 / pubkey 签名验证
// ============================================================

import { invoke } from "@tauri-apps/api/core";

const checkBtn = document.querySelector<HTMLButtonElement>("#check-btn");
const resultEl = document.querySelector<HTMLPreElement>("#result");

checkBtn!.addEventListener("click", async () => {
  resultEl!.textContent = "检查中…";
  try {
    const message = await invoke<string>("check_update");
    resultEl!.textContent = `✅ ${message}`;
    resultEl!.className = "status ok";
  } catch (e) {
    // 本练习无真实更新源，预期走这里：错误信息本身是教学点
    resultEl!.textContent = `❌ ${e}`;
    resultEl!.className = "status err";
  }
});