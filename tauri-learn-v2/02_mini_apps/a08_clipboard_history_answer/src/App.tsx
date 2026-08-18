// ============================================================
// 练习 A08: 剪贴板历史 —— 答案版
// 目标: 剪贴板插件、全局快捷键、应用菜单
// ============================================================

import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

export default function App() {
  const [history, setHistory] = useState<string[]>([]);
  const [status, setStatus] = useState("复制任意文字试试，或按 Ctrl+Shift+V 呼出/隐藏窗口");

  // 启动时拉一次 + 订阅后端推送
  useEffect(() => {
    invoke<string[]>("get_history").then(setHistory).catch(() => {});
    const un = listen<string[]>("clipboard-history", (e) => setHistory(e.payload));
    return () => {
      un.then((f) => f());
    };
  }, []);

  async function copy(text: string) {
    try {
      await invoke("copy_text", { text });
      const short = text.length > 20 ? text.slice(0, 20) + "…" : text;
      setStatus(`已复制回剪贴板: ${short}`);
    } catch (e) {
      setStatus(`复制失败: ${e}`);
    }
  }

  async function clear() {
    await invoke("clear_history");
    setStatus("已清空");
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
