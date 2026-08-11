// ============================================================
// 练习 E01: 环境准备与项目创建
// 目标: 了解 Tauri 开发前置条件，用后端命令返回环境检查结果
// 知识点: invoke() / 泛型返回类型 / Promise 错误处理
// ============================================================

import { invoke } from "@tauri-apps/api/core";

// 与后端 EnvCheck 结构体对应的 TS 接口
interface EnvCheck {
  name: string;
  ok: boolean;
  detail: string;
}

const listEl = document.querySelector<HTMLUListElement>("#checklist");
const statusEl = document.querySelector<HTMLParagraphElement>("#status");

async function render() {
  // 调用后端命令，泛型指定返回类型
  const checks = await invoke<EnvCheck[]>("check_environment");

  listEl!.innerHTML = checks
    .map(
      (c) =>
        `<li class="${c.ok ? "ok" : "warn"}">
          <span class="badge">${c.ok ? "✓" : "!"}</span>
          <strong>${c.name}</strong>
          <span class="detail">${c.detail}</span>
        </li>`
    )
    .join("");

  const ready = checks.every((c) => c.ok);
  statusEl!.textContent = ready ? "环境就绪，可以开始练习 🎉" : "存在未满足项，请先处理";
  statusEl!.className = ready ? "status ok" : "status err";
}

// 命令失败时兜底展示错误
render().catch((e) => {
  statusEl!.textContent = `调用失败: ${e}`;
  statusEl!.className = "status err";
});