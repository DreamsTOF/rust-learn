// ============================================================
// 练习 E34: Store 插件
// 目标: 使用 @tauri-apps/plugin-store 实现键值持久化与变化监听
// 知识点: load / set / get / has / delete / save / onChange 监听
// TODO: 按照注释提示补全代码
// ============================================================

// TODO: 完成填空后取消注释（load 用于加载 store 文件）
// import { load } from "@tauri-apps/plugin-store";

// 占位 store 接口：与 @tauri-apps/plugin-store 的 Store 关键方法对齐
// （完成全部填空后，本接口与下方占位对象可一并删除）
interface StoreLike {
  set(key: string, value: unknown): Promise<void>;
  get<T>(key: string): Promise<T | undefined>;
  has(key: string): Promise<boolean>;
  delete(key: string): Promise<boolean>;
  save(): Promise<void>;
  onChange<T>(cb: (key: string, value: T | undefined) => void): Promise<() => void>;
}

// === 步骤 1: 加载 store ————————————————————————————————————
// TODO: 加载（或创建）store 文件并关闭自动保存：
//   const store = await load("settings.json", { autoSave: false });
// 提示: 文件保存在 app_data_dir 下；autoSave: false 表示修改后需手动 save()
// 当前为类型化占位对象（完成填空后删除），保证后续代码可编译
const store: StoreLike = {
  set: async () => {},
  get: async () => undefined,
  has: async () => false,
  delete: async () => false,
  save: async () => {},
  onChange: async () => () => {},
};

const keyInput = document.querySelector<HTMLInputElement>("#key");
const valueInput = document.querySelector<HTMLInputElement>("#value");
const setBtn = document.querySelector<HTMLButtonElement>("#set-btn");
const getBtn = document.querySelector<HTMLButtonElement>("#get-btn");
const hasBtn = document.querySelector<HTMLButtonElement>("#has-btn");
const deleteBtn = document.querySelector<HTMLButtonElement>("#delete-btn");
const resultEl = document.querySelector<HTMLParagraphElement>("#result");
const logEl = document.querySelector<HTMLPreElement>("#log");

logEl!.textContent = "";

// 操作日志：把消息追加到日志区（练习版已实现，填空时直接使用）
function log(msg: string): void {
  const time = new Date().toLocaleTimeString();
  logEl!.textContent += `[${time}] ${msg}\n`;
}

// === 步骤 6: 监听 store 变化（v2.2+ 的 watch 更名为 onChange）——
// TODO: 把回调参数 (key, value) 的变化写入日志：
//   await store.onChange((key, value) => {
//     const v = value === undefined ? "null" : JSON.stringify(value);
//     log(`onChange: 键 "${key}" 变化 → ${v}`);
//   });
// 提示: set 时 value 为新值；delete 时 value 为 undefined（显示 null）
// 当前回调为占位（完成填空后替换），仅演示 log() 用法
async function setupWatch(): Promise<void> {
  await store.onChange((key, value) => {
    log(`占位监听: 键 "${key}" 变化 → ${JSON.stringify(value)}`);
  });
}
setupWatch().catch((e) => log(`监听注册失败: ${e}`));

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
    // === 步骤 2: set + save ————————————————————————————————————
    // TODO: 写入并持久化：
    //   await store.set(key, value);
    //   await store.save(); // autoSave: false 时必须手动保存
    // 提示: set 只修改内存，save() 才会写盘；成功后更新 #result
    // 当前为空操作占位（完成填空后删除，并移入真实调用）
    resultEl!.textContent = `已写入并保存: ${key} = ${value}`;
    resultEl!.className = "status ok";
    keyInput!.value = "";
    valueInput!.value = "";
  } catch (e) {
    resultEl!.textContent = `写入失败: ${e}`;
    resultEl!.className = "status err";
  }
});

// 读取：key 不存在时 get 返回 undefined（早期版本返回 null）
getBtn!.addEventListener("click", async () => {
  const key = keyInput!.value.trim();
  if (!key) {
    resultEl!.textContent = "请输入 key";
    resultEl!.className = "status err";
    return;
  }
  try {
    // === 步骤 3: get 读取与 null 处理 ——————————————————————————
    // TODO: 读取指定 key 并处理"不存在"的情况：
    //   const value = await store.get<string>(key);
    //   if (value === undefined) {
    //     resultEl!.textContent = `键 "${key}" 不存在（返回 undefined/null）`;
    //     resultEl!.className = "status err";
    //   } else {
    //     resultEl!.textContent = `键 "${key}" 的值: ${value}`;
    //     resultEl!.className = "status ok";
    //   }
    // 提示: 不存在时返回 undefined（早期版本返回 null），二者都表示"无值"
    // 当前为占位提示（完成填空后替换）
    resultEl!.textContent = `键 "${key}" 的读取逻辑待补全`;
    resultEl!.className = "status";
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
    // === 步骤 4: has 判断 ————————————————————————————————————
    // TODO: 判断 key 是否存在并提示：
    //   const exists = await store.has(key);
    //   resultEl!.textContent = `键 "${key}" ${exists ? "存在" : "不存在"}`;
    //   resultEl!.className = exists ? "status ok" : "status err";
    // 当前为占位提示（完成填空后替换）
    resultEl!.textContent = `键 "${key}" 的存在性判断待补全`;
    resultEl!.className = "status";
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
    // === 步骤 5: delete + save ————————————————————————————————
    // TODO: 删除指定 key 并持久化：
    //   const removed = await store.delete(key);
    //   await store.save();
    //   resultEl!.textContent = removed ? `已删除键 "${key}" 并保存`
    //                                   : `键 "${key}" 不存在，无需删除`;
    // 提示: delete 返回 boolean 表示是否删除了已有键；删除同样需要 save()
    // 当前为占位提示（完成填空后替换）
    resultEl!.textContent = `键 "${key}" 的删除逻辑待补全`;
    resultEl!.className = "status";
  } catch (e) {
    resultEl!.textContent = `删除失败: ${e}`;
    resultEl!.className = "status err";
  }
});

// 占位导出：使本文件成为模块（顶层 await 需要；import 取消注释后可删除本行）
export {};