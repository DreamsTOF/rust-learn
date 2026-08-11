// ============================================================
// 练习 E17: 退出拦截
// 目标: 监听关闭拦截事件，确认后调用命令真正退出
// 知识点: listen 事件监听 / confirm 确认框
// ============================================================

import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

// 点窗口 X 时后端先拦截并广播 close-requested，
// 这里弹确认框，确认后调用 confirm_close 销毁窗口
listen("close-requested", () => {
  if (confirm("确定要退出吗？")) {
    invoke("confirm_close").catch((e) => console.error("退出失败", e));
  }
}).catch((e) => console.error("监听失败", e));