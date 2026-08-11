// ============================================================
// 练习 E38: OS 与 Opener
// 目标: 查询系统信息，并用 opener 打开 URL / 在资源管理器中显示文件
// 知识点: platform / version / arch / type / family / openUrl / revealItemInDir
// TODO: 按照注释提示补全代码
// ============================================================

import { platform, version } from "@tauri-apps/plugin-os";
// TODO: 读取更多系统信息字段时取消注释（补全 arch / type / family）
// import { arch, family, type as osType } from "@tauri-apps/plugin-os";
// TODO: 完成填空后取消注释（opener 与 path API）
// import { openUrl, revealItemInDir } from "@tauri-apps/plugin-opener";
// import { appDataDir } from "@tauri-apps/api/path";

// === 步骤 1: 读取平台信息 ————————————————————————————————————
// TODO: 补全系统信息字段（可挖 2-3 个）：
//   const info: Array<[string, string]> = [
//     ["platform（平台）", platform()],
//     ["type（系统类型）", osType()],
//     ["arch（架构）", arch()],
//     ["version（系统版本）", version()],
//     ["family（系统家族）", family()],
//   ];
// 提示: family() 返回 'unix' | 'windows'；type 与 TS 关键字重名需重命名导入
// 当前为占位（完成填空后替换）
const info: Array<[string, string]> = [
  ["platform（平台）", platform()],
  ["version（系统版本）", version()],
];

const tableEl = document.querySelector<HTMLTableElement>("#os-table tbody");
const openBtn = document.querySelector<HTMLButtonElement>("#open-btn");
const revealBtn = document.querySelector<HTMLButtonElement>("#reveal-btn");
const resultEl = document.querySelector<HTMLParagraphElement>("#result");

// === 步骤 2: 渲染系统信息表格 ——————————————————————————————————
// TODO: 把 info 渲染为表格行：
//   tableEl!.innerHTML = info
//     .map(([k, v]) => `<tr><th>${k}</th><td>${v}</td></tr>`)
//     .join("");
// 提示: 使用 styles.css 提供的 table.kv 样式
// 当前为占位（完成填空后替换）
tableEl!.innerHTML = `<tr><th>待补全</th><td>${info.length} 个字段等待渲染</td></tr>`;

// 打开 URL：用系统默认浏览器打开
openBtn!.addEventListener("click", async () => {
  try {
    // === 步骤 3: 打开 URL ————————————————————————————————————
    // TODO: 用默认浏览器打开 URL：
    //   await openUrl("https://tauri.app");
    //   resultEl!.textContent = "已在默认浏览器打开 https://tauri.app";
    //   resultEl!.className = "status ok";
    // 提示: openUrl 来自 '@tauri-apps/plugin-opener'
    // 当前为占位（完成填空后替换）
    resultEl!.textContent = "（openUrl 逻辑待补全）";
    resultEl!.className = "status";
  } catch (e) {
    resultEl!.textContent = `打开失败: ${e}`;
    resultEl!.className = "status err";
  }
});

// 在资源管理器中显示文件：用 appDataDir 拼一个文件名
// 文件不存在时 Windows 也会打开其所在目录（也可先写入真实文件再显示）
revealBtn!.addEventListener("click", async () => {
  try {
    // === 步骤 4: 在资源管理器中显示文件 ——————————————————————————
    // TODO: 定位 appDataDir 下的文件：
    //   const dir = await appDataDir();
    //   const path = `${dir}reveal-demo.txt`;
    //   await revealItemInDir(path);
    //   resultEl!.textContent = `已在资源管理器中显示: ${path}`;
    //   resultEl!.className = "status ok";
    // 提示: appDataDir 来自 '@tauri-apps/api/path'，返回带结尾分隔符的目录路径
    // 当前为占位（完成填空后替换）
    resultEl!.textContent = "（revealItemInDir 逻辑待补全）";
    resultEl!.className = "status";
  } catch (e) {
    resultEl!.textContent = `显示失败: ${e}`;
    resultEl!.className = "status err";
  }
});