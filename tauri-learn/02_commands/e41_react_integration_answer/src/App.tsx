// ============================================================
// 练习 E41: React 集成
// 目标: React 状态驱动 UI，用 invoke 调后端命令，事件监听封装进 hook
// 知识点: useState + invoke / useEffect 事件监听 / cleanup 返回 unlisten
// ============================================================

import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

export default function App() {
  const [count, setCount] = useState<number>(0);

  // 事件监听 hook 化：useEffect 中注册，cleanup 返回 unlisten
  // （组件卸载时取消监听，防止监听器泄漏；StrictMode 下开发模式会执行两次挂载）
  useEffect(() => {
    let disposed = false;
    let unlisten: (() => void) | undefined;
    // listen 返回 Promise<UnlistenFn>，resolve 后拿到取消函数
    listen<number>("counter-changed", (event) => {
      setCount(event.payload);
    }).then((fn) => {
      if (disposed) {
        fn(); // 组件已卸载：立即取消，避免泄漏
      } else {
        unlisten = fn;
      }
    });
    return () => {
      disposed = true;
      unlisten?.(); // 卸载时取消监听
    };
  }, []);

  const handleIncrement = async () => {
    try {
      // 后端命令返回最新计数，写入 React 状态触发重渲染
      const next = await invoke<number>("increment");
      setCount(next);
    } catch (e) {
      console.error(e);
    }
  };

  const handleReset = async () => {
    try {
      const next = await invoke<number>("reset_counter");
      setCount(next);
    } catch (e) {
      console.error(e);
    }
  };

  return (
    <main>
      <h1>练习 E41: React 集成</h1>
      <p className="sub">useState + invoke / 事件监听 hook 化 / unlisten 防泄漏</p>

      <div className="card">
        <h2>计数器（后端 Mutex 状态）</h2>
        <p className="status">当前计数: {count}</p>
        <div className="row">
          <button onClick={handleIncrement}>+1（increment）</button>
          <button className="secondary" onClick={handleReset}>
            重置（reset_counter）
          </button>
        </div>
      </div>

      <div className="card">
        <h2>说明</h2>
        <ul className="checklist">
          <li><code>useState</code> 声明状态，<code>setCount</code> 更新并触发重渲染（状态驱动 UI）</li>
          <li>事件监听封装进 <code>useEffect</code>，cleanup 返回 <code>unlisten</code> 防止监听泄漏</li>
          <li>后端 <code>Counter(Mutex)</code> 由 <code>.manage()</code> 注入，命令用 <code>State</code> 访问</li>
          <li>后端每次变更后 <code>app.emit("counter-changed", ...)</code> 广播，前端监听同步</li>
        </ul>
      </div>
    </main>
  );
}