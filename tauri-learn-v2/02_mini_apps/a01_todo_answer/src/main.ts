// ============================================================
// 练习 A01: 待办清单 —— 答案版
// 目标: manage / State<T> / Mutex、结构体 + serde、listen/emit
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
    render(await invoke<TodoItem[]>("add_todo", { text }));
    inputEl!.value = "";
  } catch (e) {
    statusEl!.textContent = `调用失败: ${e}`;
  }
});

// 事件委托：点"完成/重开/删除"都走这一个监听
listEl!.addEventListener("click", async (ev) => {
  const btn = (ev.target as HTMLElement).closest<HTMLButtonElement>("[data-action]");
  if (!btn) return;
  const id = Number(btn.dataset.id);
  try {
    if (btn.dataset.action === "toggle") {
      render(await invoke<TodoItem[]>("toggle_todo", { id }));
    } else if (btn.dataset.action === "delete") {
      render(await invoke<TodoItem[]>("delete_todo", { id }));
    }
  } catch (e) {
    statusEl!.textContent = `调用失败: ${e}`;
  }
});

// 后端每次操作都会 emit "todo-log"，这里收到后追加到日志面板
listen<string>("todo-log", (event) => {
  logEl!.insertAdjacentHTML("beforeend", `<li>${event.payload}</li>`);
});

refresh();
