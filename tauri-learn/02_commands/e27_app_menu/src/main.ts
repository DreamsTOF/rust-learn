// ============================================================
// 练习 E27: 应用菜单
// 目标: 前端监听菜单动作事件，展示最近操作
// TODO: 按照注释提示补全代码
// ============================================================

// TODO: 完成填空后取消注释（listen 监听后端事件）
// import { listen } from "@tauri-apps/api/event";

const actionList = document.querySelector<HTMLUListElement>("#action-list");

// === 步骤 1: 监听菜单动作 ————————————————————————————————————
// TODO: 用 listen 监听 "menu-action" 事件，把 e.payload 追加到 #action-list 顶部
// 提示: listen<string>("menu-action", (e) => {
//         const li = document.createElement("li");
//         li.className = "ok";
//         li.innerHTML = `<span class="badge">菜</span>${e.payload}`;
//         actionList!.prepend(li);
//       });

// 占位：先渲染一条提示，完成监听填空后删除
actionList!.innerHTML = `<li class="warn"><span class="badge">?</span>等待监听代码完成</li>`;