// ============================================================
// 练习 A06: 汇率查询 —— 练习版
// 目标: HTTP 插件（reqwest）、异步 + 超时、Store 缓存
// TODO: 按注释提示补全（共 2 处）
// ============================================================

import { useState } from "react";
// === 步骤 1 ————————————————————————————————————————————————
// TODO: 导入 invoke（调用后端 get_rate）
// 提示: import { invoke } from "@tauri-apps/api/core";
// import { invoke } from "@tauri-apps/api/core";

const CURRENCIES = ["USD", "EUR", "GBP", "JPY", "CNY", "HKD", "AUD", "CAD"];

interface RateInfo {
  from: string;
  to: string;
  rate: number;
  date: string;
  from_cache: boolean;
}

export default function App() {
  const [from, setFrom] = useState("USD");
  const [to, setTo] = useState("CNY");
  const [amount, setAmount] = useState("100");
  const [info, setInfo] = useState<RateInfo | null>(null);
  const [status, setStatus] = useState("");

  async function query() {
    setStatus("查询中…");
    try {
      // === 步骤 2 ————————————————————————————————————————————
      // TODO: 调用 get_rate（参数 { from, to }），把结果放进 setInfo，
      //       并提示"来自缓存 / 来自网络"
      // 提示: const info = await invoke<RateInfo>("get_rate", { from, to });
      //       setInfo(info);
      //       setStatus(info.from_cache ? "来自缓存（1 小时内）" : "来自网络");
      setInfo(null); // ← 替换成你的代码
    } catch (e) {
      setStatus(`查询失败: ${e}`);
    }
  }

  const result = info ? Number(amount) * info.rate : null;

  return (
    <main className="card">
      <h1>练习 A06: 汇率查询</h1>
      <p className="sub">HTTP 请求（reqwest + 超时）· Store 缓存 1 小时</p>

      <div className="row">
        <select value={from} onChange={(e) => setFrom(e.target.value)}>
          {CURRENCIES.map((c) => (
            <option key={c} value={c}>
              {c}
            </option>
          ))}
        </select>
        <span className="arrow">→</span>
        <select value={to} onChange={(e) => setTo(e.target.value)}>
          {CURRENCIES.map((c) => (
            <option key={c} value={c}>
              {c}
            </option>
          ))}
        </select>
        <input
          type="number"
          value={amount}
          onChange={(e) => setAmount(e.target.value)}
        />
        <button onClick={query}>查询</button>
      </div>

      <div className="result">
        {result !== null && info ? (
          <>
            <p className="big">
              1 {info.from} = {info.rate.toFixed(4)} {info.to}
            </p>
            <p className="big">
              {amount} {info.from} = {result.toFixed(2)} {info.to}
            </p>
            <p className="sub2">
              汇率日期 {info.date} · {status}
            </p>
          </>
        ) : (
          <p className="placeholder">选择货币，输入金额，点查询</p>
        )}
      </div>

      <p className="status">{status}</p>
    </main>
  );
}
