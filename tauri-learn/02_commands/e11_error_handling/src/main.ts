// ============================================================
// 练习 E11: 错误处理
// 目标: 观察 thiserror 错误消息如何传回前端
// TODO: 按照注释提示补全代码
// ============================================================

// TODO: 完成填空后取消注释（invoke 用于调用后端命令）
// import { invoke } from "@tauri-apps/api/core";

const numberInput = document.querySelector<HTMLInputElement>("#number");
const parseBtn = document.querySelector<HTMLButtonElement>("#parse-btn");
const readBtn = document.querySelector<HTMLButtonElement>("#read-btn");
const resultEl = document.querySelector<HTMLParagraphElement>("#result");

// 解析数字：输入非法时后端返回 InvalidInput 错误
parseBtn!.addEventListener("click", async () => {
  try {
    // === 步骤 5: 调用 parse_number ————————————————————————————————————
    // TODO: const input = numberInput!.value;
    //       const v = await invoke<number>("parse_number", { input });
    // 提示: Rust 参数 input 在 JS 侧同名
    // 占位：完成填空后替换为真实调用结果（当前先展示输入框内容）
    const v: number = Number(numberInput!.value) || 0;
    resultEl!.textContent = `解析结果 ×2 = ${v}`;
    resultEl!.className = "status ok";
  } catch (e) {
    // Tauri 会把 Err 序列化为字符串（thiserror 的 Display 消息）
    resultEl!.textContent = `err: ${e}`;
    resultEl!.className = "status err";
  }
});

// 读文件：文件不存在时后端经 ? 传播 Io 错误
readBtn!.addEventListener("click", async () => {
  try {
    // === 步骤 6: 调用 read_marker ————————————————————————————————————
    // TODO: const text = await invoke<string>("read_marker");
    // 提示: 文件不存在时后端返回 Err，前端收到 "IO 错误: ..." 消息
    // 占位：完成填空后替换为真实调用结果
    const text: string = "";
    resultEl!.textContent = text;
    resultEl!.className = "status ok";
  } catch (e) {
    resultEl!.textContent = `err: ${e}`;
    resultEl!.className = "status err";
  }
});