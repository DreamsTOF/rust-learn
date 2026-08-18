// ============================================================
// 练习 A03: 番茄钟 —— 答案版
// 目标: async 命令 + tokio、通知插件、系统托盘 + 隐藏窗口
// ============================================================

import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

function fmt(sec: number): string {
  const m = Math.floor(sec / 60);
  const s = sec % 60;
  return `${String(m).padStart(2, "0")}:${String(s).padStart(2, "0")}`;
}

export default function App() {
  const [remaining, setRemaining] = useState<number>(25 * 60);
  const [running, setRunning] = useState(false);
  const [status, setStatus] = useState("");

  // 订阅后端倒计时事件：每秒 tick 更新显示，结束/停止切换状态
  useEffect(() => {
    const unTick = listen<number>("pomodoro-tick", (e) => setRemaining(e.payload));
    const unDone = listen("pomodoro-done", () => {
      setRunning(false);
      setStatus("时间到！");
    });
    const unStop = listen("pomodoro-stopped", () => {
      setRunning(false);
      setStatus("已停止");
    });
    return () => {
      unTick.then((f) => f());
      unDone.then((f) => f());
      unStop.then((f) => f());
    };
  }, []);

  async function start(minutes: number) {
    setRunning(true);
    setStatus("计时中…");
    setRemaining(minutes * 60);
    try {
      await invoke("start_pomodoro", { minutes });
    } catch (e) {
      setRunning(false);
      setStatus(`启动失败: ${e}`);
    }
  }

  async function stop() {
    await invoke("stop_pomodoro");
  }

  return (
    <main className="card">
      <h1>练习 A03: 番茄钟（答案）</h1>
      <p className="sub">倒计时跑在 Rust 后台（async + tokio）· 到点发系统通知 · 关窗进托盘</p>

      <p className="time">{fmt(remaining)}</p>

      <div className="row">
        <button onClick={() => start(25)} disabled={running}>
          工作 25 分
        </button>
        <button onClick={() => start(5)} disabled={running}>
          休息 5 分
        </button>
        <button onClick={stop} disabled={!running}>
          停止
        </button>
      </div>

      <p className="status">{status}</p>
    </main>
  );
}
