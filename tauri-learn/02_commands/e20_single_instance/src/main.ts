// ============================================================
// 练习 E20: 单实例
// 目标: 读取实例 ID，验证重复启动被拦截
// TODO: 按照注释提示补全代码
// ============================================================

// TODO: 完成填空后取消注释（invoke 用于调用后端命令）
// import { invoke } from "@tauri-apps/api/core";

const loadBtn = document.querySelector<HTMLButtonElement>("#load-btn");
const idEl = document.querySelector<HTMLSpanElement>("#instance-id");
const resultEl = document.querySelector<HTMLParagraphElement>("#result");

loadBtn!.addEventListener("click", async () => {
  try {
    // === 步骤 6: 调用 get_instance_id ————————————————————————————————————
    // TODO: const id = await invoke<string>("get_instance_id");
    // 提示: invoke 来自 "@tauri-apps/api/core"；把返回值显示到 #instance-id
    // 占位：完成填空后替换为真实调用结果
    const id: string = "实例-????";
    idEl!.textContent = id;
    resultEl!.textContent = "实例 ID 已读取（再次启动应用会拿到同样的 ID）";
    resultEl!.className = "status ok";
  } catch (e) {
    resultEl!.textContent = `调用失败: ${e}`;
    resultEl!.className = "status err";
  }
});