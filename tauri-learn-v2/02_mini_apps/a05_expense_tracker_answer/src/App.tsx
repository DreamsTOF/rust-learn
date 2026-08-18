// ============================================================
// 练习 A05: 记账本 —— 答案版
// 目标: SQL 插件（建表迁移 + 前端增删改查 + SUM 统计）
// ============================================================

import { useEffect, useState } from "react";
import Database from "@tauri-apps/plugin-sql";

const CATEGORIES = ["餐饮", "交通", "购物", "娱乐", "其他"];
const DB_URL = "sqlite:expenses.db";

interface Expense {
  id: number;
  title: string;
  amount: number;
  category: string;
  createdAt: string;
}

interface MonthTotal {
  month: string;
  total: number;
}

export default function App() {
  const [db, setDb] = useState<Database | null>(null);
  const [expenses, setExpenses] = useState<Expense[]>([]);
  const [totals, setTotals] = useState<MonthTotal[]>([]);
  const [title, setTitle] = useState("");
  const [amount, setAmount] = useState("");
  const [category, setCategory] = useState(CATEGORIES[0]);
  const [status, setStatus] = useState("连接数据库…");

  // 打开数据库（首次连接时自动执行 Rust 端迁移建表）
  useEffect(() => {
    Database.load(DB_URL)
      .then(async (db) => {
        setDb(db);
        await refresh(db);
        setStatus("已连接");
      })
      .catch((e) => setStatus(`连接失败: ${e}`));
  }, []);

  async function refresh(db: Database) {
    const rows = await db.select<Expense[]>(
      "SELECT id, title, amount, category, created_at AS createdAt FROM expenses ORDER BY id DESC"
    );
    setExpenses(rows);

    const months = await db.select<MonthTotal[]>(
      "SELECT strftime('%Y-%m', created_at) AS month, SUM(amount) AS total " +
        "FROM expenses GROUP BY month ORDER BY month DESC"
    );
    setTotals(months);
  }

  async function add() {
    if (!db) return;
    const value = Number(amount);
    if (!title.trim() || Number.isNaN(value)) {
      setStatus("请填写标题和金额");
      return;
    }
    try {
      // datetime('now','localtime') 由 SQLite 生成，前端不用管时间
      await db.execute(
        "INSERT INTO expenses (title, amount, category, created_at) VALUES ($1, $2, $3, datetime('now','localtime'))",
        [title.trim(), value, category]
      );
      setTitle("");
      setAmount("");
      await refresh(db);
      setStatus("已添加");
    } catch (e) {
      setStatus(`添加失败: ${e}`);
    }
  }

  async function remove(id: number) {
    if (!db) return;
    try {
      await db.execute("DELETE FROM expenses WHERE id = $1", [id]);
      await refresh(db);
      setStatus("已删除");
    } catch (e) {
      setStatus(`删除失败: ${e}`);
    }
  }

  return (
    <main className="card">
      <h1>练习 A05: 记账本</h1>
      <p className="sub">SQLite 数据库（sql 插件）· 增删改查 + 月度合计</p>

      <div className="row">
        <input
          placeholder="买了什么"
          value={title}
          onChange={(e) => setTitle(e.target.value)}
        />
        <input
          placeholder="金额"
          type="number"
          value={amount}
          onChange={(e) => setAmount(e.target.value)}
        />
        <select value={category} onChange={(e) => setCategory(e.target.value)}>
          {CATEGORIES.map((c) => (
            <option key={c} value={c}>
              {c}
            </option>
          ))}
        </select>
        <button onClick={add}>添加</button>
      </div>

      <h2 className="summary-title">月度合计</h2>
      <ul className="summary">
        {totals.map((t) => (
          <li key={t.month}>
            <span>{t.month}</span>
            <strong>¥ {t.total.toFixed(2)}</strong>
          </li>
        ))}
      </ul>

      <ul className="list">
        {expenses.map((e) => (
          <li key={e.id}>
            <span className="cat">{e.category}</span>
            <span className="text">{e.title}</span>
            <span className="amount">-¥ {e.amount.toFixed(2)}</span>
            <span className="time">{e.createdAt}</span>
            <button onClick={() => remove(e.id)}>删除</button>
          </li>
        ))}
      </ul>

      <p className="status">{status}</p>
    </main>
  );
}
