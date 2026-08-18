// ============================================================
// 练习 A08: 剪贴板历史 —— 练习版
// 目标: 剪贴板插件、全局快捷键、应用菜单
// TODO: 按注释提示补全（共 4 处）
// ============================================================

import { useEffect, useState } from "react";
import { listen } from "@tauri-apps/api/event";
// === 步骤 1 ————————————————————————————————————————————————
// TODO: 导入 invoke（拉取历史 / 复制 / 清空）
// 提示: import { invoke } from "@tauri-apps/api/core";
// import { invoke } from "@tauri-apps/api/core";

export default function App() {
  const [history, setHistory] = useState<string[]>([]);
  const [status, setStatus] = useState("复制任意文字试试，或按 Ctrl+Shift+V 呼出/隐藏窗口");

  // 启动时拉一次 + 订阅后端推送
  useEffect(() => {
    // === 步骤 2 ————————————————————————————————————————————
    // TODO: 启动时拉一次历史
    // 提示: invoke<string[]>("get_history").then(setHistory).catch(() => {});
    setHistory([]); // ← 替换成你的代码
    const un = listen<string[]>("clipboard-history", (e) => setHistory(e.payload));
    return () => {
      un.then((f) => f());
    };
  }, []);

  async function copy(text: string) {
    try {
      // === 步骤 3 ————————————————————————————————————————————
      // TODO: 把 text 写回剪贴板（调 copy_text）并提示
      // 提示: await invoke("copy_text", { text });
      //       setStatus("已复制回剪贴板");
      setStatus(`已复制: ${text}`); // ← 替换成你的代码
    } catch (e) {
      setStatus(`复制失败: ${e}`);
    }
  }

  async function clear() {
    // === 步骤 4 ————————————————————————————————————————————
    // TODO: 清空历史（调 clear_history）并提示
    // 提示: await invoke("clear_history");
    //       setStatus("已清空");
    setStatus("已清空"); // ← 替换成你的代码
  }

  return (
    <main className="card">
      <h1>练习 A08: 剪贴板历史</h1>
      <p className="sub">
        后台监控剪贴板（clipboard 插件）· Ctrl+Shift+V 呼出窗口 · 历史存 Store
      </p>

      <div className="row">
        <button onClick={clear}>清空历史</button>
        <span className="hint">窗口菜单也能"清空历史 / 退出"</span>
      </div>

      <ul className="list">
        {history.length === 0 ? (
          <li className="empty">还没有历史——复制点什么吧</li>
        ) : (
          history.map((text, i) => (
            <li key={i} onClick={() => copy(text)} title="点击复制回剪贴板">
              {text}
            </li>
          ))
        )}
      </ul>

      <p className="status">{status}</p>
    </main>
  );
}
