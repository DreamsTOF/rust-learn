// ============================================================
// 练习 E01: 环境准备与项目创建
// 目标: 了解 Tauri 开发前置条件，用后端命令返回环境检查结果
// TODO: 按照注释提示补全代码
// ============================================================

// TODO: 完成填空后取消注释（invoke 用于调用后端命令）
// import { invoke } from "@tauri-apps/api/core";

// 与后端 EnvCheck 结构体对应的 TS 接口
interface EnvCheck {
  name: string;
  ok: boolean;
  detail: string;
}

const listEl = document.querySelector<HTMLUListElement>("#checklist");
const statusEl = document.querySelector<HTMLParagraphElement>("#status");

async function render() {
  // === 步骤 1: 调用后端命令 ————————————————————————————————
  // TODO: 改为调用真实命令：
  //   const checks = await invoke<EnvCheck[]>("check_environment");
  // 当前为空数组占位（保持可编译），完成填空后页面将显示检查清单
  const checks: EnvCheck[] = [];

  // === 步骤 2: 渲染检查清单 ————————————————————————————————
  // TODO: 完善 li 模板，每项渲染：
  //   <li class="ok|warn"><span class="badge">✓|!</span>
  //   <strong>名称</strong><span class="detail">说明</span></li>
  // 提示: c.ok ? "ok" : "warn"，c.ok ? "✓" : "!"
  listEl!.innerHTML = checks.map((c) => `<li>${c.name}</li>`).join("");

  // === 步骤 3: 汇总状态 ————————————————————————————————————
  // TODO: 完成就绪判断（当前恒为 true）
  // 提示: const ready = checks.every((c) => c.ok);
  const ready = checks.length > 0 && checks.every((c) => c.ok);
  statusEl!.textContent = ready ? "环境就绪，可以开始练习 🎉" : "存在未满足项，请先处理";
}

// 命令失败时兜底展示错误
render().catch((e) => {
  statusEl!.textContent = `调用失败: ${e}`;
});