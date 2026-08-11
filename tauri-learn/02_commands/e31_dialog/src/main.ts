// ============================================================
// 练习 E31: 对话框（dialog 插件）
// 目标: 用 @tauri-apps/plugin-dialog 打开/保存文件、选择目录、确认与消息
// 知识点: open / save / ask / message / 文件过滤器 / 取消处理
// TODO: 按照注释提示补全代码
// ============================================================

import { open, save, ask, message } from "@tauri-apps/plugin-dialog";

// 占位引用：确保被挖空的插件 API 导入被使用（全部 TODO 完成后删除本行）
void [open, save, ask];

const openBtn = document.querySelector<HTMLButtonElement>("#open-btn");
const multiBtn = document.querySelector<HTMLButtonElement>("#multi-btn");
const dirBtn = document.querySelector<HTMLButtonElement>("#dir-btn");
const saveBtn = document.querySelector<HTMLButtonElement>("#save-btn");
const askBtn = document.querySelector<HTMLButtonElement>("#ask-btn");
const messageBtn = document.querySelector<HTMLButtonElement>("#message-btn");
const resultEl = document.querySelector<HTMLPreElement>("#result");

openBtn!.addEventListener("click", async () => {
  try {
    // === 步骤 1: 打开文件（带过滤器）——————————————————————
    // TODO: const path = await open({
    //         multiple: false,
    //         filters: [{ name: "文本", extensions: ["txt", "md"] }],
    //       });
    // 提示: open 返回 string | string[] | null，取消时为 null
    const path = null as string | null; // 占位：完成填空后删除

    // === 步骤 2: 取消判断 —————————————————————————————————
    // TODO: path 为 null 时显示 "已取消"，否则显示 `选择的文件: ${path}`
    // 提示: if (path === null) { ... } else { ... }
    resultEl!.textContent = path === null ? "已取消" : `选择的文件: ${path}`;
  } catch (e) {
    resultEl!.textContent = `打开失败: ${e}`;
  }
});

multiBtn!.addEventListener("click", async () => {
  try {
    // === 步骤 3: 多选文件 ————————————————————————————————
    // TODO: const paths = await open({ multiple: true });
    // 提示: multiple: true 时返回 string[]，取消仍为 null
    const paths = null as string[] | null; // 占位：完成填空后删除
    resultEl!.textContent =
      paths === null ? "已取消" : `选择了 ${paths.length} 个文件:\n${paths.join("\n")}`;
  } catch (e) {
    resultEl!.textContent = `多选失败: ${e}`;
  }
});

dirBtn!.addEventListener("click", async () => {
  try {
    // === 步骤 3b: 选择目录 ————————————————————————————————
    // TODO: const dir = await open({ directory: true });
    // 提示: directory: true 时打开目录选择器，返回 string | null
    const dir = null as string | null; // 占位：完成填空后删除
    resultEl!.textContent = dir === null ? "已取消" : `选择的目录: ${dir}`;
  } catch (e) {
    resultEl!.textContent = `选择目录失败: ${e}`;
  }
});

saveBtn!.addEventListener("click", async () => {
  try {
    // === 步骤 4: 保存对话框 ————————————————————————————————
    // TODO: const path = await save({
    //         defaultPath: "notes.md",
    //         filters: [{ name: "Markdown", extensions: ["md"] }],
    //       });
    // 提示: save 只返回保存路径，不会真正创建文件；取消返回 null
    const path = null as string | null; // 占位：完成填空后删除
    resultEl!.textContent = path === null ? "已取消" : `保存路径: ${path}`;
  } catch (e) {
    resultEl!.textContent = `保存失败: ${e}`;
  }
});

askBtn!.addEventListener("click", async () => {
  try {
    // === 步骤 5: 确认对话框 ————————————————————————————————
    // TODO: const ok = await ask("确定要继续吗？", { title: "确认操作", kind: "warning" });
    // 提示: ask 返回 true（确定）/ false（取消）
    const ok = false; // 占位：完成填空后删除
    resultEl!.textContent = `你的选择: ${ok}`;
  } catch (e) {
    resultEl!.textContent = `ask 失败: ${e}`;
  }
});

messageBtn!.addEventListener("click", async () => {
  try {
    await message("这是一条消息", { title: "提示", kind: "info" });
    resultEl!.textContent = "消息对话框已关闭";
  } catch (e) {
    resultEl!.textContent = `message 失败: ${e}`;
  }
});