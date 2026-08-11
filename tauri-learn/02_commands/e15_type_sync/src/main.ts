// ============================================================
// 练习 E15: 前后端类型同步
// 目标: 定义前后端一致的接口类型并渲染返回值
// TODO: 按照注释提示补全代码
// ============================================================

// TODO: 完成填空后取消注释（invoke 用于调用后端命令）
// import { invoke } from "@tauri-apps/api/core";

// === 步骤 4: 定义前端接口 ————————————————————————————————————
// TODO: 定义 UserProfile 接口（JS 侧用 camelCase，与 Rust snake_case 自动对应）
// 提示: interface UserProfile {
//         userId: number;
//         displayName: string;
//         tags: string[];
//       }
interface UserProfile {}

const fetchBtn = document.querySelector<HTMLButtonElement>("#fetch-btn");
const profilePre = document.querySelector<HTMLPreElement>("#profile-json");
const resultEl = document.querySelector<HTMLParagraphElement>("#result");

fetchBtn!.addEventListener("click", async () => {
  try {
    // === 步骤 5: 调用 get_profile ————————————————————————————————————
    // TODO: const profile = await invoke<UserProfile>("get_profile", { userId: 1 });
    // 提示: Rust 参数 user_id 在 JS 侧写作 userId（camelCase）
    // 占位：完成填空后替换为真实调用结果
    const profile: UserProfile = {} as UserProfile;
    profilePre!.textContent = JSON.stringify(profile, null, 2);
    resultEl!.textContent = "获取成功 ✓";
    resultEl!.className = "status ok";
  } catch (e) {
    resultEl!.textContent = `调用失败: ${e}`;
    resultEl!.className = "status err";
  }
});