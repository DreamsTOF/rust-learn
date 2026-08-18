// ============================================================
// 练习 A02: 记事本 —— 答案版
// 目标: 路径 API、fs 插件读/写、Result 错误处理、React
// ============================================================

import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";

export default function App() {
  const [content, setContent] = useState("");
  const [filePath, setFilePath] = useState("加载中…");
  const [status, setStatus] = useState("");

  // 挂载时：显示文件位置 + 读取已有内容
  useEffect(() => {
    invoke<string>("note_file_path")
      .then(setFilePath)
      .catch((e) => setStatus(`获取路径失败: ${e}`));

    invoke<string>("load_note")
      .then((c) => {
        setContent(c);
        setStatus("已加载");
      })
      .catch((e) => setStatus(`读取失败: ${e}`));
  }, []);

  async function save() {
    try {
      await invoke("save_note", { content });
      setStatus(`已保存 ${new Date().toLocaleTimeString()}`);
    } catch (e) {
      setStatus(`保存失败: ${e}`);
    }
  }

  return (
    <main className="card">
      <h1>练习 A02: 记事本（答案）</h1>
      <p className="sub">关掉软件字还在 —— 内容存在 {filePath}</p>

      <textarea
        value={content}
        onChange={(e) => setContent(e.target.value)}
        placeholder="写点什么，然后点保存……"
      />

      <div className="row">
        <button onClick={save}>保存</button>
        <span className="status">{status}</span>
      </div>
    </main>
  );
}
