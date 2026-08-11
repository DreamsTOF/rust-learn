// ============================================================
// 练习 E30: 文件系统（fs 插件）
// 目标: 用 @tauri-apps/plugin-fs 读写文件、列目录、查信息、操作文件
// 知识点: readTextFile / writeTextFile / readDir / stat / exists /
//         copyFile / rename / remove / scope 限制
// ============================================================

import {
  readTextFile,
  writeTextFile,
  readDir,
  stat,
  exists,
  remove,
  copyFile,
  rename,
} from "@tauri-apps/plugin-fs";
import { appDataDir, join } from "@tauri-apps/api/path";

const dirEl = document.querySelector<HTMLPreElement>("#dir-path");
const writeInput = document.querySelector<HTMLInputElement>("#write-content");
const writeBtn = document.querySelector<HTMLButtonElement>("#write-btn");
const readBtn = document.querySelector<HTMLButtonElement>("#read-btn");
const infoBtn = document.querySelector<HTMLButtonElement>("#info-btn");
const existsBtn = document.querySelector<HTMLButtonElement>("#exists-btn");
const listBtn = document.querySelector<HTMLButtonElement>("#list-btn");
const chainBtn = document.querySelector<HTMLButtonElement>("#chain-btn");
const fileResultEl = document.querySelector<HTMLPreElement>("#file-result");
const dirListEl = document.querySelector<HTMLUListElement>("#dir-list");
const chainResultEl = document.querySelector<HTMLParagraphElement>("#chain-result");

let demoPath = ""; // $APPDATA/demo.txt
let copyPath = ""; // $APPDATA/demo-copy.txt
let renamedPath = ""; // $APPDATA/demo-renamed.txt

async function initPaths(): Promise<void> {
  const dir = await appDataDir();
  dirEl!.textContent = dir;
  demoPath = await join(dir, "demo.txt");
  copyPath = await join(dir, "demo-copy.txt");
  renamedPath = await join(dir, "demo-renamed.txt");
}

writeBtn!.addEventListener("click", async () => {
  try {
    await writeTextFile(demoPath, writeInput!.value);
    fileResultEl!.textContent = "已写入";
  } catch (e) {
    fileResultEl!.textContent = `写入失败: ${e}`;
  }
});

readBtn!.addEventListener("click", async () => {
  try {
    const content = await readTextFile(demoPath);
    fileResultEl!.textContent = `内容: ${content}`;
  } catch (e) {
    fileResultEl!.textContent = `读取失败: ${e}`;
  }
});

infoBtn!.addEventListener("click", async () => {
  try {
    const info = await stat(demoPath);
    fileResultEl!.textContent = [
      `isFile: ${info.isFile}`,
      `isDirectory: ${info.isDirectory}`,
      `size: ${info.size} 字节`,
      `mtime: ${info.mtime ? info.mtime.toLocaleString() : "不可用"}`,
    ].join("\n");
  } catch (e) {
    fileResultEl!.textContent = `stat 失败: ${e}`;
  }
});

existsBtn!.addEventListener("click", async () => {
  try {
    const ok = await exists(demoPath);
    fileResultEl!.textContent = `demo.txt 是否存在: ${ok}`;
  } catch (e) {
    fileResultEl!.textContent = `exists 失败: ${e}`;
  }
});

listBtn!.addEventListener("click", async () => {
  try {
    const entries = await readDir(await appDataDir());
    dirListEl!.innerHTML = entries
      .map(
        (entry) =>
          `<li><span class="badge">${entry.isDirectory ? "D" : "F"}</span>` +
          `<span>${entry.name}</span>` +
          `<span class="detail">${entry.isDirectory ? "目录" : "文件"}</span></li>`
      )
      .join("");
  } catch (e) {
    dirListEl!.innerHTML = `<li>readDir 失败: ${e}</li>`;
  }
});

chainBtn!.addEventListener("click", async () => {
  try {
    await copyFile(demoPath, copyPath);
    await rename(copyPath, renamedPath);
    await remove(renamedPath);
    chainResultEl!.textContent = "操作链完成: 复制 → 重命名 → 删除";
    chainResultEl!.className = "status ok";
  } catch (e) {
    chainResultEl!.textContent = `操作链失败: ${e}`;
    chainResultEl!.className = "status err";
  }
});

initPaths().catch((e) => {
  dirEl!.textContent = `获取路径失败: ${e}`;
});