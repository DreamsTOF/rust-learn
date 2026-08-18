// ============================================================
// 练习 A03: 番茄钟 —— 练习版
// 目标: async 命令 + tokio、通知插件、系统托盘 + 隐藏窗口
// TODO: 按注释提示补全（共 4 处）
// ============================================================

import { useEffect, useState } from "react";
// === 步骤 1 ————————————————————————————————————————————————
// TODO: 导入 invoke（启动/停止计时）和 listen（收 tick/done 事件）
// 提示: import { invoke } from "@tauri-apps/api/core";
//       import { listen } from "@tauri-apps/api/event";
// import { invoke } from "@tauri-apps/api/core";
// import { listen } from "@tauri-apps/api/event";

function fmt(sec: number): string {
  const m = Math.floor(sec / 60);
  const s = sec % 60;
  return `${String(m).padStart(2, "0")}:${String(s).padStart(2, "0")}`;
}

export default function App() {
  const [remaining, setRemaining] = useState<number>(25 * 60);
  const [running, setRunning] = useState(false);
  const [status, setStatus] = useState("");

  useEffect(() => {
    // === 步骤 2 ————————————————————————————————————————————
    // TODO: 监听三个事件，更新 state
    // 提示: listen<number>("pomodoro-tick", (e) => setRemaining(e.payload))
    //       listen("pomodoro-done", () => { setRunning(false); setStatus("时间到！"); })
    //       listen("pomodoro-stopped", () => { setRunning(false); setStatus("已停止"); })
    // 占位（保持编译通过，完成后删除）：让三个 setter 先被引用
    setRemaining(remaining);
    setRunning(running);
    setStatus(status);
  }, []);

  async function start(minutes: number) {
    setRunning(true);
    setStatus("计时中…");
    setRemaining(minutes * 60);
    try {
      // === 步骤 3 ————————————————————————————————————————————
      // TODO: await invoke("start_pomodoro", { minutes })
      // 提示: await invoke("start_pomodoro", { minutes });
    } catch (e) {
      setRunning(false);
      setStatus(`启动失败: ${e}`);
    }
  }

  async function stop() {
    // === 步骤 4 ————————————————————————————————————————————
    // TODO: await invoke("stop_pomodoro")
    // 提示: await invoke("stop_pomodoro");
  }

  return (
    <main className="card">
      <h1>练习 A03: 番茄钟</h1>
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
