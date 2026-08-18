// ============================================================
// 练习 A07: 批量重命名 —— 答案版
// 目标: 后台任务（async_runtime::spawn）、Channel 进度、dialog
// ============================================================

import { useState } from "react";
import { invoke, Channel } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";

interface PreviewItem {
  old: string;
  new: string;
}

interface RenameProgress {
  done: number;
  total: number;
  current: string;
  finished: boolean;
}

export default function App() {
  const [dir, setDir] = useState("");
  const [find, setFind] = useState("IMG_");
  const [replace, setReplace] = useState("photo_");
  const [preview, setPreview] = useState<PreviewItem[]>([]);
  const [progress, setProgress] = useState<RenameProgress | null>(null);
  const [status, setStatus] = useState("先选择文件夹");

  async function pickDir() {
    const selected = await open({ directory: true });
    if (typeof selected === "string") {
      setDir(selected);
      setStatus(`已选择: ${selected}`);
    }
  }

  async function doPreview() {
    try {
      const items = await invoke<PreviewItem[]>("preview_rename", { dir, find, replace });
      setPreview(items);
      setStatus(`预览完成：${items.length} 个文件会改名`);
    } catch (e) {
      setStatus(`预览失败: ${e}`);
    }
  }

  async function doRename() {
    setProgress(null);
    setStatus("重命名中…");
    try {
      // Channel：把"进度通道"作为参数传给后端，后端每处理一个文件往里 send
      const channel = new Channel<RenameProgress>();
      channel.onmessage = (m) => {
        setProgress(m);
        if (m.finished) {
          setStatus(`完成：共处理 ${m.total} 个文件`);
        }
      };
      await invoke("run_rename", { dir, find, replace, onProgress: channel });
    } catch (e) {
      setStatus(`重命名失败: ${e}`);
    }
  }

  const percent =
    progress && progress.total > 0 ? Math.round((progress.done / progress.total) * 100) : 0;

  return (
    <main className="card">
      <h1>练习 A07: 批量重命名</h1>
      <p className="sub">后台任务（spawn）· Channel 进度推送 · 先预览再执行</p>

      <div className="row">
        <input readOnly value={dir} placeholder="文件夹路径" />
        <button onClick={pickDir}>选择文件夹</button>
      </div>

      <div className="row">
        <input value={find} onChange={(e) => setFind(e.target.value)} placeholder="查找（如 IMG_）" />
        <span className="arrow">→</span>
        <input value={replace} onChange={(e) => setReplace(e.target.value)} placeholder="替换为（如 photo_）" />
        <button onClick={doPreview} disabled={!dir}>
          预览
        </button>
        <button onClick={doRename} disabled={!dir || preview.length === 0}>
          开始重命名
        </button>
      </div>

      <div className="progress">
        <div className="bar" style={{ width: `${percent}%` }} />
      </div>
      {progress && (
        <p className="progress-text">
          {progress.done} / {progress.total} · 正在处理 {progress.current}
        </p>
      )}

      <ul className="preview">
        {preview.map((item, i) => (
          <li key={i}>
            <span className="old">{item.old}</span>
            <span className="arrow">→</span>
            <span className="new">{item.new}</span>
          </li>
        ))}
      </ul>

      <p className="status">{status}</p>
    </main>
  );
}
