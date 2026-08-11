// ============================================================
// 练习 E34: Store 插件
// 目标: 使用 @tauri-apps/plugin-store 实现键值持久化与变化监听
// 知识点: load / set / get / has / delete / save / onChange 监听
// ============================================================

import { load, Store } from "@tauri-apps/plugin-store";

// 全局 store 句柄（init 中完成异步加载）
let store: Store;

// 加载（或创建）store：文件保存在 app_data_dir/settings.json
// autoSave: false → 关闭自动保存，修改后必须手动调用 save() 才落盘
async function init(): Promise<void> {
  store = await load("settings.json", { autoSave: false });
  // === 监听 store 变化（v2.2+ 的 watch 更名为 onChange）——============
  // 回调 (key, value)：set 时 value 为新值；delete 时 value 为 undefined（视为 null）
  await store.onChange((key, value) => {
    const v = value === undefined ? "null" : JSON.stringify(value);
    log(`onChange: 键 "${key}" 变化 → ${v}`);
  });
}

const keyInput = document.querySelector<HTMLInputElement>("#key");
const valueInput = document.querySelector<HTMLInputElement>("#value");
const setBtn = document.querySelector<HTMLButtonElement>("#set-btn");
const getBtn = document.querySelector<HTMLButtonElement>("#get-btn");
const hasBtn = document.querySelector<HTMLButtonElement>("#has-btn");
const deleteBtn = document.querySelector<HTMLButtonElement>("#delete-btn");
const resultEl = document.querySelector<HTMLParagraphElement>("#result");
const logEl = document.querySelector<HTMLPreElement>("#log");

logEl!.textContent = "";

// 操作日志：把消息追加到日志区
function log(msg: string): void {
  const time = new Date().toLocaleTimeString();
  logEl!.textContent += `[${time}] ${msg}\n`;
}

// 写入：set 修改内存 → save 写盘
setBtn!.addEventListener("click", async () => {
  const key = keyInput!.value.trim();
  const value = valueInput!.value.trim();
  if (!key) {
    resultEl!.textContent = "请输入 key";
    resultEl!.className = "status err";
    return;
  }
  try {
    await store.set(key, value);
    await store.save(); // autoSave: false，必须手动 save 才会持久化
    resultEl!.textContent = `已写入并保存: ${key} = ${value}`;
    resultEl!.className = "status ok";
    keyInput!.value = "";
    valueInput!.value = "";
  } catch (e) {
    resultEl!.textContent = `写入失败: ${e}`;
    resultEl!.className = "status err";
  }
});

// 读取：key 不存在时 get 返回 undefined（早期版本返回 null），统一按"无值"处理
getBtn!.addEventListener("click", async () => {
  const key = keyInput!.value.trim();
  if (!key) {
    resultEl!.textContent = "请输入 key";
    resultEl!.className = "status err";
    return;
  }
  try {
    const value = await store.get<string>(key);
    if (value === undefined) {
      resultEl!.textContent = `键 "${key}" 不存在（返回 undefined/null）`;
      resultEl!.className = "status err";
    } else {
      resultEl!.textContent = `键 "${key}" 的值: ${value}`;
      resultEl!.className = "status ok";
    }
  } catch (e) {
    resultEl!.textContent = `读取失败: ${e}`;
    resultEl!.className = "status err";
  }
});

// 判断 key 是否存在
hasBtn!.addEventListener("click", async () => {
  const key = keyInput!.value.trim();
  if (!key) {
    resultEl!.textContent = "请输入 key";
    resultEl!.className = "status err";
    return;
  }
  try {
    const exists = await store.has(key);
    resultEl!.textContent = `键 "${key}" ${exists ? "存在" : "不存在"}`;
    resultEl!.className = exists ? "status ok" : "status err";
  } catch (e) {
    resultEl!.textContent = `检查失败: ${e}`;
    resultEl!.className = "status err";
  }
});

// 删除：delete 修改内存 → save 写盘（返回 boolean 表示是否删除成功）
deleteBtn!.addEventListener("click", async () => {
  const key = keyInput!.value.trim();
  if (!key) {
    resultEl!.textContent = "请输入 key";
    resultEl!.className = "status err";
    return;
  }
  try {
    const removed = await store.delete(key);
    await store.save();
    resultEl!.textContent = removed
      ? `已删除键 "${key}" 并保存`
      : `键 "${key}" 不存在，无需删除`;
    resultEl!.className = removed ? "status ok" : "status err";
  } catch (e) {
    resultEl!.textContent = `删除失败: ${e}`;
    resultEl!.className = "status err";
  }
});

// 启动：加载 store 并注册监听
init().catch((e) => {
  log(`初始化失败: ${e}`);
});