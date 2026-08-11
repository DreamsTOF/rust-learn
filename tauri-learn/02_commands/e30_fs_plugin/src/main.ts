// ============================================================
// 练习 E30: 文件系统（fs 插件）
// 目标: 用 @tauri-apps/plugin-fs 读写文件、列目录、查信息、操作文件
// 知识点: readTextFile / writeTextFile / readDir / stat / exists /
//         copyFile / rename / remove / scope 限制
// TODO: 按照注释提示补全代码
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

// 占位引用：确保插件 API 导入被使用（全部 TODO 完成后删除本行）
void [writeTextFile, readTextFile, stat, exists, readDir, copyFile, rename, remove];

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
  // 占位引用：确保路径变量被使用（全部 TODO 完成后删除本行）
  void [demoPath, copyPath, renamedPath];
}

writeBtn!.addEventListener("click", async () => {
  try {
    // === 步骤 1: 写入文本 ————————————————————————————————
    // TODO: await writeTextFile(demoPath, writeInput!.value);
    // 提示: writeTextFile(路径, 内容) 来自 @tauri-apps/plugin-fs
    void writeInput!.value; // 占位引用，完成填空后删除
    fileResultEl!.textContent = "已写入";
  } catch (e) {
    fileResultEl!.textContent = `写入失败: ${e}`;
  }
});

readBtn!.addEventListener("click", async () => {
  try {
    // === 步骤 2: 读取文本 ————————————————————————————————
    // TODO: const content = await readTextFile(demoPath);
    // 提示: readTextFile(路径) 返回 Promise<string>
    const content = ""; // 占位：完成填空后删除
    fileResultEl!.textContent = `内容: ${content}`;
  } catch (e) {
    fileResultEl!.textContent = `读取失败: ${e}`;
  }
});

infoBtn!.addEventListener("click", async () => {
  try {
    // === 步骤 3a: 文件信息 ————————————————————————————————
    // TODO: const info = await stat(demoPath);
    // 提示: info.isFile / info.isDirectory / info.size / info.mtime
    const info = {
      isFile: false,
      isDirectory: false,
      size: 0,
      mtime: null as Date | null,
    }; // 占位：完成填空后删除
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
    // === 步骤 3b: 判断存在 ————————————————————————————————
    // TODO: const ok = await exists(demoPath);
    // 提示: exists(路径) 返回 Promise<boolean>
    const ok = false; // 占位：完成填空后删除
    fileResultEl!.textContent = `demo.txt 是否存在: ${ok}`;
  } catch (e) {
    fileResultEl!.textContent = `exists 失败: ${e}`;
  }
});

listBtn!.addEventListener("click", async () => {
  try {
    // === 步骤 4: 目录列表 ————————————————————————————————
    // TODO: const entries = await readDir(await appDataDir());
    //       再用 entries.map(...) 渲染文件名与 isDirectory 标记
    // 提示: entry.name / entry.isDirectory；可参考答案版渲染写法
    const entries: { name: string; isDirectory: boolean }[] = []; // 占位
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
    // === 步骤 5: 文件操作链 ————————————————————————————————
    // TODO: 依次执行 copyFile(demoPath, copyPath)、
    //       rename(copyPath, renamedPath)、remove(renamedPath)，
    //       成功后显示 "操作链完成: 复制 → 重命名 → 删除"
    // 提示: 三个函数均来自 @tauri-apps/plugin-fs，操作都在 $APPDATA 内
    let chainStep = `待执行: ${copyPath} / ${renamedPath}`; // 占位：完成填空后删除
    chainResultEl!.textContent = chainStep;
    chainResultEl!.className = "status ok";
  } catch (e) {
    chainResultEl!.textContent = `操作链失败: ${e}`;
    chainResultEl!.className = "status err";
  }
});

initPaths().catch((e) => {
  dirEl!.textContent = `获取路径失败: ${e}`;
});