// ============================================================
// 练习 A02: 记事本 —— 练习版
// 目标: 路径 API、fs 插件读/写、Result 错误处理、React
// TODO: 按注释提示补全（共 4 处）
// ============================================================

import { useEffect, useState } from "react";
// === 步骤 1 ————————————————————————————————————————————————
// TODO: 导入 invoke（调用后端命令）
// 提示: import { invoke } from "@tauri-apps/api/core";
// import { invoke } from "@tauri-apps/api/core";

export default function App() {
  const [content, setContent] = useState("");
  const [filePath, setFilePath] = useState("加载中…");
  const [status, setStatus] = useState("");

  // 挂载时：显示文件位置 + 读取已有内容
  useEffect(() => {
    // === 步骤 2 ————————————————————————————————————————————
    // TODO: 调用后端 note_file_path，把返回的路径放进 setFilePath
    // 提示: invoke<string>("note_file_path")
    //         .then(setFilePath)
    //         .catch((e) => setStatus(`获取路径失败: ${e}`));
    // === 步骤 3 ————————————————————————————————————————————
    // TODO: 调用后端 load_note，把内容放进 setContent 并提示"已加载"
    // 提示: invoke<string>("load_note")
    //         .then((c) => { setContent(c); setStatus("已加载"); })
    //         .catch((e) => setStatus(`读取失败: ${e}`));
    // 占位（保持编译通过，完成后删除）：让两个 setter 先被引用
    setFilePath("加载中…");
    setStatus("");
  }, []);

  async function save() {
    try {
      // === 步骤 4 ————————————————————————————————————————————
      // TODO: 调用后端 save_note（参数 { content }），成功后提示"已保存 时间"
      // 提示: await invoke("save_note", { content });
      //       setStatus(`已保存 ${new Date().toLocaleTimeString()}`);
      // setStatus("…"); // ← 完成后由上面的 invoke 结果替换
    } catch (e) {
      setStatus(`保存失败: ${e}`);
    }
  }

  return (
    <main className="card">
      <h1>练习 A02: 记事本</h1>
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
