// ============================================================
// 练习 E48: 自动更新（updater）
// 目标: 集成 updater 插件并实现"检查更新"命令
// 知识点: 更新流程 / endpoints 模板变量 / pubkey 签名验证
// TODO: 按照注释提示补全代码
// ============================================================

// TODO: 完成填空后取消注释（invoke 用于调用后端命令）
// import { invoke } from "@tauri-apps/api/core";

const checkBtn = document.querySelector<HTMLButtonElement>("#check-btn");
const resultEl = document.querySelector<HTMLPreElement>("#result");

checkBtn!.addEventListener("click", async () => {
  resultEl!.textContent = "检查中…";
  try {
    // === 步骤 2: 调用 check_update 命令 ——————————————————————
    // TODO: 调用后端命令并把结果赋给 message 变量
    // 提示: const message = await invoke<string>("check_update");
    // 当前为占位字符串（保持可编译），完成填空后将显示检查结果
    const message = "占位结果";

    // === 步骤 3: 展示结果 ——————————————————————————————————
    // TODO: 成功时显示 ✅ 消息，失败时显示 ❌ 错误
    // 提示: resultEl!.textContent = `✅ ${message}`;
    //       resultEl!.className = "status ok";
    resultEl!.textContent = `✅ ${message}`;
    resultEl!.className = "status ok";
  } catch (e) {
    // 本练习无真实更新源，预期走这里：错误信息本身是教学点
    resultEl!.textContent = `❌ ${e}`;
    resultEl!.className = "status err";
  }
});