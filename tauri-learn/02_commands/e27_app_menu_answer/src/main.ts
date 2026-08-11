// ============================================================
// 练习 E27: 应用菜单
// 目标: 前端监听菜单动作事件，展示最近操作
// 知识点: listen / menu-action 事件
// ============================================================

import { listen } from "@tauri-apps/api/event";

const actionList = document.querySelector<HTMLUListElement>("#action-list");

// 后端 emit 的 menu-action 事件：菜单点击 / 快捷键触发后到达这里
listen<string>("menu-action", (e) => {
  const li = document.createElement("li");
  li.className = "ok";
  li.innerHTML = `<span class="badge">菜</span>${e.payload}`;
  actionList!.prepend(li);
});