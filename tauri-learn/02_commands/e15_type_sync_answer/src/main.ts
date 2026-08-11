// ============================================================
// 练习 E15: 前后端类型同步
// 目标: 用 TS 接口描述后端返回值并渲染
// 知识点: invoke<T> 泛型 / camelCase 字段 / JSON 展示
// ============================================================

import { invoke } from "@tauri-apps/api/core";

// 与 Rust 端 UserProfile 对应：
// Rust snake_case 字段（user_id / display_name）会自动转为 JS camelCase
interface UserProfile {
  userId: number;
  displayName: string;
  tags: string[];
}

const fetchBtn = document.querySelector<HTMLButtonElement>("#fetch-btn");
const profilePre = document.querySelector<HTMLPreElement>("#profile-json");
const resultEl = document.querySelector<HTMLParagraphElement>("#result");

fetchBtn!.addEventListener("click", async () => {
  try {
    // Rust 参数 user_id 在 JS 侧写作 userId（camelCase）
    const profile = await invoke<UserProfile>("get_profile", { userId: 1 });
    profilePre!.textContent = JSON.stringify(profile, null, 2);
    resultEl!.textContent = "获取成功 ✓";
    resultEl!.className = "status ok";
  } catch (e) {
    resultEl!.textContent = `调用失败: ${e}`;
    resultEl!.className = "status err";
  }
});