// ============================================================
// 练习 E38: OS 与 Opener
// 目标: 查询系统信息，并用 opener 打开 URL / 在资源管理器中显示文件
// 知识点: platform / version / arch / type / family / openUrl / revealItemInDir
// ============================================================

import {
  platform,
  version,
  arch,
  family,
  type as osType,
} from "@tauri-apps/plugin-os";
import { openUrl, revealItemInDir } from "@tauri-apps/plugin-opener";
import { appDataDir } from "@tauri-apps/api/path";

// === 读取系统信息（全部为同步 API；family 返回 'unix' | 'windows'）——
const info: Array<[string, string]> = [
  ["platform（平台）", platform()],
  ["type（系统类型）", osType()],
  ["arch（架构）", arch()],
  ["version（系统版本）", version()],
  ["family（系统家族）", family()],
];

const tableEl = document.querySelector<HTMLTableElement>("#os-table tbody");
const openBtn = document.querySelector<HTMLButtonElement>("#open-btn");
const revealBtn = document.querySelector<HTMLButtonElement>("#reveal-btn");
const resultEl = document.querySelector<HTMLParagraphElement>("#result");

// === 渲染系统信息表格 ——————————————————————————————————————
tableEl!.innerHTML = info
  .map(([k, v]) => `<tr><th>${k}</th><td>${v}</td></tr>`)
  .join("");

// 打开 URL：用系统默认浏览器打开
openBtn!.addEventListener("click", async () => {
  try {
    await openUrl("https://tauri.app");
    resultEl!.textContent = "已在默认浏览器打开 https://tauri.app";
    resultEl!.className = "status ok";
  } catch (e) {
    resultEl!.textContent = `打开失败: ${e}`;
    resultEl!.className = "status err";
  }
});

// 在资源管理器中显示文件：用 appDataDir 拼一个文件名
// 文件不存在时 Windows 也会打开其所在目录（也可先写入真实文件再显示）
revealBtn!.addEventListener("click", async () => {
  try {
    const dir = await appDataDir();
    const path = `${dir}reveal-demo.txt`;
    await revealItemInDir(path);
    resultEl!.textContent = `已在资源管理器中显示: ${path}`;
    resultEl!.className = "status ok";
  } catch (e) {
    resultEl!.textContent = `显示失败: ${e}`;
    resultEl!.className = "status err";
  }
});