// ============================================================
// 练习 A01: 待办清单 —— 练习版
// 目标: manage / State<T> / Mutex、结构体 + serde、listen/emit
// TODO: 按注释提示补全（共 4 处）
// ============================================================

import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

// 与后端 TodoItem 结构体对应的 TS 接口（字段名一一对应）
interface TodoItem {
  id: number;
  text: string;
  done: boolean;
}

const listEl = document.querySelector<HTMLUListElement>("#todo-list");
const inputEl = document.querySelector<HTMLInputElement>("#new-todo");
const addBtn = document.querySelector<HTMLButtonElement>("#add-btn");
const logEl = document.querySelector<HTMLUListElement>("#log");
const statusEl = document.querySelector<HTMLParagraphElement>("#status");

// 把待办数组渲染成列表（done 决定样式类）
function render(items: TodoItem[]) {
  listEl!.innerHTML = items
    .map(
      (t) => `<li class="${t.done ? "done" : ""}">
        <span class="text">${t.text}</span>
        <button data-action="toggle" data-id="${t.id}">${t.done ? "重开" : "完成"}</button>
        <button data-action="delete" data-id="${t.id}">删除</button>
      </li>`
    )
    .join("");
}

// 启动时拉一次最新列表
async function refresh() {
  try {
    render(await invoke<TodoItem[]>("list_todos"));
  } catch (e) {
    statusEl!.textContent = `调用失败: ${e}`;
  }
}

addBtn!.addEventListener("click", async () => {
  const text = inputEl!.value.trim();
  if (!text) return;
  try {
    // === 步骤 2 ————————————————————————————————————————————
    // TODO: 调用 add_todo 命令（参数 { text }），用返回的最新列表渲染
    // 提示: render(await invoke<TodoItem[]>("add_todo", { text }));
    render([]); // ← 替换成你的代码
    inputEl!.value = "";
  } catch (e) {
    statusEl!.textContent = `调用失败: ${e}`;
  }
});

// 事件委托：点"完成/重开/删除"都走这一个监听
listEl!.addEventListener("click", async (ev) => {
  const btn = (ev.target as HTMLElement).closest<HTMLButtonElement>("[data-action]");
  if (!btn) return;
  try {
    // === 步骤 3 ————————————————————————————————————————————
    // TODO: 先取 id（Number(btn.dataset.id)），再根据 data-action
    //       调用 toggle_todo / delete_todo（参数 { id }）并重新渲染
    // 提示: const id = Number(btn.dataset.id);
    //       if (btn.dataset.action === "toggle")
    //         render(await invoke<TodoItem[]>("toggle_todo", { id }));
    //       else if (btn.dataset.action === "delete")
    //         render(await invoke<TodoItem[]>("delete_todo", { id }));
    render([]); // ← 替换成你的代码
  } catch (e) {
    statusEl!.textContent = `调用失败: ${e}`;
  }
});

// === 步骤 4 ————————————————————————————————————————————————
// TODO: 把下面的占位改成"追加日志"——每收到一条 todo-log，
//       就往 logEl 末尾插一个 <li>
// 提示: logEl!.insertAdjacentHTML("beforeend", `<li>${event.payload}</li>`);
listen<string>("todo-log", (event) => {
  logEl!.textContent = `（TODO：把日志追加成 <li>，事件内容：${event.payload}）`;
});

refresh();
