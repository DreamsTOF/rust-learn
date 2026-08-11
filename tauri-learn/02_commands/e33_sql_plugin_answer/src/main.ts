// ============================================================
// 练习 E33: SQL（sql 插件）
// 目标: 用 @tauri-apps/plugin-sql 建表、增删查、绑定参数与事务
// 知识点: Database.load / execute / select / 绑定参数 / 事务提交回滚
// ============================================================

import Database from "@tauri-apps/plugin-sql";

interface Note {
  id: number;
  title: string;
  body: string | null;
}

// 数据库句柄（init 中完成异步初始化）
let db: InstanceType<typeof Database>;

async function init(): Promise<void> {
  // 连接串格式 "sqlite:文件名"，数据库文件位于 app_config_dir
  db = await Database.load("sqlite:notes.db");
  // IF NOT EXISTS 保证重复启动不会报错
  await db.execute(
    "CREATE TABLE IF NOT EXISTS notes (id INTEGER PRIMARY KEY AUTOINCREMENT, title TEXT NOT NULL, body TEXT)"
  );
}

const titleInput = document.querySelector<HTMLInputElement>("#note-title");
const bodyInput = document.querySelector<HTMLInputElement>("#note-body");
const addBtn = document.querySelector<HTMLButtonElement>("#add-btn");
const refreshBtn = document.querySelector<HTMLButtonElement>("#refresh-btn");
const commitBtn = document.querySelector<HTMLButtonElement>("#commit-btn");
const rollbackBtn = document.querySelector<HTMLButtonElement>("#rollback-btn");
const listEl = document.querySelector<HTMLUListElement>("#note-list");
const txnResultEl = document.querySelector<HTMLParagraphElement>("#txn-result");

async function refreshList(): Promise<void> {
  const rows = await db.select<Note[]>("SELECT * FROM notes ORDER BY id DESC");
  listEl!.innerHTML =
    rows
      .map(
        (row) =>
          `<li><span class="badge">#${row.id}</span><span><strong>${row.title}</strong> — ${row.body ?? ""}</span></li>`
      )
      .join("") || "<li>(暂无笔记)</li>";
}

addBtn!.addEventListener("click", async () => {
  try {
    const title = titleInput!.value.trim() || "未命名";
    const body = bodyInput!.value.trim();
    // $1/$2 为绑定参数，对应数组元素，避免 SQL 注入
    await db.execute("INSERT INTO notes (title, body) VALUES ($1, $2)", [title, body]);
    txnResultEl!.textContent = "已添加";
    txnResultEl!.className = "status ok";
    await refreshList();
  } catch (e) {
    txnResultEl!.textContent = `添加失败: ${e}`;
    txnResultEl!.className = "status err";
  }
});

refreshBtn!.addEventListener("click", async () => {
  try {
    await refreshList();
    txnResultEl!.textContent = "列表已刷新";
    txnResultEl!.className = "status ok";
  } catch (e) {
    txnResultEl!.textContent = `刷新失败: ${e}`;
    txnResultEl!.className = "status err";
  }
});

commitBtn!.addEventListener("click", async () => {
  try {
    // 顺序 await 时连接池只有单个连接，BEGIN/INSERT/COMMIT 落在同一连接上
    await db.execute("BEGIN");
    await db.execute("INSERT INTO notes (title, body) VALUES ($1, $2)", [
      "事务-已提交",
      "commit 演示",
    ]);
    await db.execute("COMMIT");
    txnResultEl!.textContent = "事务已提交（列表应出现「事务-已提交」）";
    txnResultEl!.className = "status ok";
    await refreshList();
  } catch (e) {
    txnResultEl!.textContent = `事务提交失败: ${e}`;
    txnResultEl!.className = "status err";
  }
});

rollbackBtn!.addEventListener("click", async () => {
  try {
    await db.execute("BEGIN");
    await db.execute("INSERT INTO notes (title, body) VALUES ($1, $2)", [
      "事务-已回滚",
      "rollback 演示",
    ]);
    await db.execute("ROLLBACK");
    txnResultEl!.textContent = "已回滚（列表不应出现「事务-已回滚」）";
    txnResultEl!.className = "status ok";
    await refreshList();
  } catch (e) {
    txnResultEl!.textContent = `事务回滚失败: ${e}`;
    txnResultEl!.className = "status err";
  }
});

// 启动时先展示已有数据
init()
  .then(() => refreshList())
  .catch((e) => {
    listEl!.innerHTML = `<li>初始化失败: ${e}</li>`;
  });