// ============================================================
// 练习 E45: 权限系统（permissions）
// 目标: 用自定义 permission 文件收紧 fs 插件的读写 scope
// 知识点: @tauri-apps/plugin-fs / @tauri-apps/api/path / scope 拒绝
// TODO: 按照注释提示补全代码
// ============================================================

// 插件 API 已安装（package.json），练习版保留 import：
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
  // 占位引用：确保 notePath 被使用（全部 TODO 完成后删除本行）
  void notePath;
}

writeBtn!.addEventListener("click", async () => {
  try {
    // === 步骤 2: 写笔记 ————————————————————————————————————
    // TODO: 调用 writeTextFile 把输入框内容写入 notePath
    // 提示: await writeTextFile(notePath, noteInput!.value);
    await writeTextFile("", noteInput!.value);
    resultEl!.textContent = "✅ 写入成功（路径在允许的 scope 内）";
    resultEl!.className = "status ok";
  } catch (e) {
    resultEl!.textContent = `❌ 写入失败: ${e}`;
    resultEl!.className = "status err";
  }
});

readBtn!.addEventListener("click", async () => {
  try {
    // === 步骤 3: 读笔记 ————————————————————————————————————
    // TODO: 调用 readTextFile 读取 notePath，把内容放进 content 变量
    // 提示: const content = await readTextFile(notePath);
    const content = await readTextFile("");
    resultEl!.textContent = `✅ 读取成功，内容: ${content}`;
    resultEl!.className = "status ok";
  } catch (e) {
    resultEl!.textContent = `❌ 读取失败: ${e}`;
    resultEl!.className = "status err";
  }
});

rogueBtn!.addEventListener("click", async () => {
  try {
    // === 步骤 4: 越权演示 ——————————————————————————————————
    // TODO: 尝试读取系统文件 C:/Windows/win.ini（不在允许的 scope 内）
    // 提示: const content = await readTextFile("C:/Windows/win.ini");
    const content = await readTextFile("");
    resultEl!.textContent = `⚠️ 意外读到了系统文件: ${content}`;
    resultEl!.className = "status ok";
  } catch (e) {
    // 教学点：路径不在 $APPDATA/notes/** 范围内，插件会拒绝（报错含 "path not allowed"）
    resultEl!.textContent = `✅ 越权读取被拒绝（scope 生效）: ${e}`;
    resultEl!.className = "status err";
  }
});

initPath().catch((e) => {
  dirEl!.textContent = `获取路径失败: ${e}`;
});