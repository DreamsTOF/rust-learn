// ============================================================
// 练习 E41: React 集成
// 目标: React 状态驱动 UI，用 invoke 调后端命令，事件监听封装进 hook
// 知识点: React 挂载 / createRoot / StrictMode
// ============================================================

import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import "./styles.css";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>
);