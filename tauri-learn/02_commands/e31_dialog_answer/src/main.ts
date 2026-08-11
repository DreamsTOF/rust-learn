// ============================================================
// 练习 E31: 对话框（dialog 插件）
// 目标: 用 @tauri-apps/plugin-dialog 打开/保存文件、选择目录、确认与消息
// 知识点: open / save / ask / message / 文件过滤器 / 取消处理
// ============================================================

import { open, save, ask, message } from "@tauri-apps/plugin-dialog";

const openBtn = document.querySelector<HTMLButtonElement>("#open-btn");
const multiBtn = document.querySelector<HTMLButtonElement>("#multi-btn");
const dirBtn = document.querySelector<HTMLButtonElement>("#dir-btn");
const saveBtn = document.querySelector<HTMLButtonElement>("#save-btn");
const askBtn = document.querySelector<HTMLButtonElement>("#ask-btn");
const messageBtn = document.querySelector<HTMLButtonElement>("#message-btn");
const resultEl = document.querySelector<HTMLPreElement>("#result");

openBtn!.addEventListener("click", async () => {
  try {
    const path = await open({
      multiple: false,
      filters: [{ name: "文本", extensions: ["txt", "md"] }],
    });
    // 取消时 open 返回 null
    resultEl!.textContent = path === null ? "已取消" : `选择的文件: ${path}`;
  } catch (e) {
    resultEl!.textContent = `打开失败: ${e}`;
  }
});

multiBtn!.addEventListener("click", async () => {
  try {
    const paths = await open({ multiple: true });
    // multiple: true 时返回字符串数组，取消仍为 null
    resultEl!.textContent =
      paths === null ? "已取消" : `选择了 ${paths.length} 个文件:\n${paths.join("\n")}`;
  } catch (e) {
    resultEl!.textContent = `多选失败: ${e}`;
  }
});

dirBtn!.addEventListener("click", async () => {
  try {
    const dir = await open({ directory: true });
    resultEl!.textContent = dir === null ? "已取消" : `选择的目录: ${dir}`;
  } catch (e) {
    resultEl!.textContent = `选择目录失败: ${e}`;
  }
});

saveBtn!.addEventListener("click", async () => {
  try {
    const path = await save({
      defaultPath: "notes.md",
      filters: [{ name: "Markdown", extensions: ["md"] }],
    });
    // save 只返回用户选择的保存路径，不会真正创建文件
    resultEl!.textContent = path === null ? "已取消" : `保存路径: ${path}`;
  } catch (e) {
    resultEl!.textContent = `保存失败: ${e}`;
  }
});

askBtn!.addEventListener("click", async () => {
  try {
    const ok = await ask("确定要继续吗？", { title: "确认操作", kind: "warning" });
    // ask 返回 true（确定）/ false（取消）
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