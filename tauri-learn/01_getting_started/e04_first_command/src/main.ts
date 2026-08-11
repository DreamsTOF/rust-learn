// ============================================================
// 练习 E04: 第一个命令
// 目标: 走通 #[tauri::command] → generate_handler! → invoke() 全链路
// TODO: 按照注释提示补全代码
// ============================================================

// TODO: 完成填空后取消注释（invoke 用于调用后端命令）
// import { invoke } from "@tauri-apps/api/core";

const nameInput = document.querySelector<HTMLInputElement>("#name");
const greetBtn = document.querySelector<HTMLButtonElement>("#greet-btn");
const resultEl = document.querySelector<HTMLParagraphElement>("#result");

greetBtn!.addEventListener("click", async () => {
  try {
    // === 步骤 1: 调用后端命令 ————————————————————————————————
    // TODO: 读取输入框值并调用 greet 命令：
    //   const name = nameInput!.value.trim() || "Tauri";
    //   const message = await invoke<string>("greet", { name });
    // 提示: invoke 的参数名与 Rust 参数名一致（camelCase）
    // 当前为空字符串占位（保持可编译），完成填空后将显示问候语
    let message = "";

    // === 步骤 2: 展示结果 ——————————————————————————————————
    // TODO: 把 message 显示到 #result，并设置 className = "status ok"
    // 提示: resultEl!.textContent = message;
    //       resultEl!.className = "status ok";
    resultEl!.textContent = message;
  } catch (e) {
    resultEl!.textContent = `调用失败: ${e}`;
    resultEl!.className = "status err";
  }
});