// ============================================================
// 练习 E01: 计数器 —— 练习版
// 目标: invoke / #[tauri::command] / generate_handler! / serde
// TODO: 按注释提示补全两处
// ============================================================

// === 步骤 1 ————————————————————————————————————————————————
// TODO: 导入 invoke（前端调用后端的唯一入口）
// 提示: 取消注释下面这行
// import { invoke } from "@tauri-apps/api/core";

const valueEl = document.querySelector<HTMLSpanElement>("#value");
const btnEl = document.querySelector<HTMLButtonElement>("#increment");
const statusEl = document.querySelector<HTMLParagraphElement>("#status");

let count = 0;

async function increment() {
  try {
    // === 步骤 2 ————————————————————————————————————————————
    // TODO: 调用后端 count_up 命令，把当前值加一后拿回来
    // 提示: count = await invoke<number>("count_up", { current: count });
    count = 0; // ← 替换成你的代码

    valueEl!.textContent = String(count);
    statusEl!.textContent = `后端返回: ${count}`;
  } catch (e) {
    statusEl!.textContent = `调用失败: ${e}`;
  }
}

btnEl!.addEventListener("click", increment);
