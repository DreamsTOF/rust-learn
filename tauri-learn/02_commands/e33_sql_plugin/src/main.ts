// ============================================================
// 练习 E33: SQL（sql 插件）
// 目标: 用 @tauri-apps/plugin-sql 建表、增删查、绑定参数与事务
// 知识点: Database.load / execute / select / 绑定参数 / 事务提交回滚
// TODO: 按照注释提示补全代码
// ============================================================

import Database from "@tauri-apps/plugin-sql";

interface Note {
  id: number;
  title: string;
  body: string | null;
}

// === 步骤 1: 加载数据库 ————————————————————————————————————
// TODO: const db = await Database.load("sqlite:notes.db");
// 提示: 连接串格式 "sqlite:文件名"，数据库文件位于 app_config_dir
const db = null as unknown as InstanceType<typeof Database>; // 占位：完成填空后删除
void db; // 占位引用：确保 db 变量被使用（全部 TODO 完成后删除本行）

// === 步骤 2: 建表 ——————————————————————————————————————————
// TODO: await db.execute(
//         "CREATE TABLE IF NOT EXISTS notes (id INTEGER PRIMARY KEY AUTOINCREMENT, title TEXT NOT NULL, body TEXT)"
//       );
// 提示: IF NOT EXISTS 保证重复启动不会报错
// （当前为占位，无操作）

const titleInput = document.querySelector<HTMLInputElement>("#note-title");
const bodyInput = document.querySelector<HTMLInputElement>("#note-body");
// 占位引用：确保输入框变量被使用（全部 TODO 完成后删除本行）
void [titleInput, bodyInput];
const addBtn = document.querySelector<HTMLButtonElement>("#add-btn");
const refreshBtn = document.querySelector<HTMLButtonElement>("#refresh-btn");
const commitBtn = document.querySelector<HTMLButtonElement>("#commit-btn");
const rollbackBtn = document.querySelector<HTMLButtonElement>("#rollback-btn");
const listEl = document.querySelector<HTMLUListElement>("#note-list");
const txnResultEl = document.querySelector<HTMLParagraphElement>("#txn-result");

async function refreshList(): Promise<void> {
  // === 步骤 3: 查询并渲染 ——————————————————————————————————
  // TODO: const rows = await db.select<Note[]>("SELECT * FROM notes ORDER BY id DESC");
  //       然后用 rows.map(...) 渲染（id / title / body），空列表显示 "(暂无笔记)"
  // 提示: select 返回对象数组，字段 id/title/body
  const rows: Note[] = []; // 占位：完成填空后删除
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
    // === 步骤 4: 插入笔记（绑定参数）————————————————————————
    // TODO: 读取输入并执行插入：
    //   const title = titleInput!.value.trim() || "未命名";
    //   const body = bodyInput!.value.trim();
    //   await db.execute("INSERT INTO notes (title, body) VALUES ($1, $2)", [title, body]);
    // 提示: $1/$2 为绑定参数，对应数组元素，避免 SQL 注入
    txnResultEl!.textContent = "已添加（占位：完成填空后生效）";
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
    // === 步骤 5: 事务提交 ——————————————————————————————————
    // TODO: 依次执行三条语句：
    //   await db.execute("BEGIN");
    //   await db.execute("INSERT INTO notes (title, body) VALUES ($1, $2)", ["事务-已提交", "commit 演示"]);
    //   await db.execute("COMMIT");
    // 提示: 顺序 await 时连接池只有单个连接，三条语句落在同一连接上
    txnResultEl!.textContent = "事务已提交（占位：完成填空后生效）";
    txnResultEl!.className = "status ok";
    await refreshList();
  } catch (e) {
    txnResultEl!.textContent = `事务提交失败: ${e}`;
    txnResultEl!.className = "status err";
  }
});

rollbackBtn!.addEventListener("click", async () => {
  try {
    // === 步骤 6: 事务回滚 ——————————————————————————————————
    // TODO: 依次执行：
    //   await db.execute("BEGIN");
    //   await db.execute("INSERT INTO notes (title, body) VALUES ($1, $2)", ["事务-已回滚", "rollback 演示"]);
    //   await db.execute("ROLLBACK");
    // 提示: 回滚后列表不应出现「事务-已回滚」，以此验证事务
    txnResultEl!.textContent = "已回滚（占位：完成填空后生效）";
    txnResultEl!.className = "status ok";
    await refreshList();
  } catch (e) {
    txnResultEl!.textContent = `事务回滚失败: ${e}`;
    txnResultEl!.className = "status err";
  }
});

// 启动时先展示已有数据
refreshList().catch((e) => {
  listEl!.innerHTML = `<li>加载失败: ${e}</li>`;
});