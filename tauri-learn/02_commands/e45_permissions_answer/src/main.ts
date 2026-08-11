// ============================================================
// 练习 E45: 权限系统（permissions）
// 目标: 用自定义 permission 文件收紧 fs 插件的读写 scope
// 知识点: @tauri-apps/plugin-fs / @tauri-apps/api/path / scope 拒绝
// ============================================================

import { readTextFile, writeTextFile } from "@tauri-apps/plugin-fs";
import { appDataDir, join } from "@tauri-apps/api/path";

const dirEl = document.querySelector<HTMLPreElement>("#notes-dir");
const noteInput = document.querySelector<HTMLInputElement>("#note-content");
const writeBtn = document.querySelector<HTMLButtonElement>("#write-btn");
const readBtn = document.querySelector<HTMLButtonElement>("#read-btn");
const rogueBtn = document.querySelector<HTMLButtonElement>("#rogue-btn");
const resultEl = document.querySelector<HTMLPreElement>("#result");

// 笔记文件路径：appDataDir/notes/demo.txt（对应 scope 中的 $APPDATA/notes/**）
let notePath = "";

async function initPath(): Promise<void> {
  const dir = await appDataDir();
  dirEl!.textContent = dir;
  notePath = await join(dir, "notes", "demo.txt");
}

writeBtn!.addEventListener("click", async () => {
  try {
    // 写笔记：路径在 scope（$APPDATA/notes/**）内，被 fs:allow-write-text-file 放行
    await writeTextFile(notePath, noteInput!.value);
    resultEl!.textContent = "✅ 写入成功（路径在允许的 scope 内）";
    resultEl!.className = "status ok";
  } catch (e) {
    resultEl!.textContent = `❌ 写入失败: ${e}`;
    resultEl!.className = "status err";
  }
});

readBtn!.addEventListener("click", async () => {
  try {
    // 读笔记：同一路径，被 fs:allow-read-text-file 放行
    const content = await readTextFile(notePath);
    resultEl!.textContent = `✅ 读取成功，内容: ${content}`;
    resultEl!.className = "status ok";
  } catch (e) {
    resultEl!.textContent = `❌ 读取失败: ${e}`;
    resultEl!.className = "status err";
  }
});

rogueBtn!.addEventListener("click", async () => {
  try {
    // 越权演示：该路径不在 $APPDATA/notes/** 范围内，会被 scope 拒绝
    const content = await readTextFile("C:/Windows/win.ini");
    resultEl!.textContent = `⚠️ 意外读到了系统文件: ${content}`;
    resultEl!.className = "status ok";
  } catch (e) {
    resultEl!.textContent = `✅ 越权读取被拒绝（scope 生效）: ${e}`;
    resultEl!.className = "status err";
  }
});

initPath().catch((e) => {
  dirEl!.textContent = `获取路径失败: ${e}`;
});