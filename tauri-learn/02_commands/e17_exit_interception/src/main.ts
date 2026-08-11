// ============================================================
// 练习 E17: 退出拦截
// 目标: 监听关闭拦截事件，确认后调用命令真正退出
// TODO: 按照注释提示补全代码
// ============================================================

// TODO: 完成填空后取消注释（invoke 调用命令；listen 监听后端事件）
// import { invoke } from "@tauri-apps/api/core";
// import { listen } from "@tauri-apps/api/event";

// === 步骤 4: 监听关闭拦截事件 ————————————————————————————————————
// TODO: listen("close-requested", () => {
//         if (confirm("确定要退出吗？")) {
//           invoke("confirm_close").catch((e) => console.error("退出失败", e));
//         }
//       }).catch((e) => console.error("监听失败", e));
// 提示: 点窗口 X 时后端会先拦截并广播 close-requested 事件，
//       确认后调用 confirm_close 命令销毁窗口