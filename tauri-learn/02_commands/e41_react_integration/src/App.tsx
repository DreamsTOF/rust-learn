// ============================================================
// 练习 E41: React 集成
// 目标: React 状态驱动 UI，用 invoke 调后端命令，事件监听封装进 hook
// 知识点: useState + invoke / useEffect 事件监听 / cleanup 返回 unlisten
// TODO: 按照注释提示补全代码
// ============================================================

// TODO: 完成填空后取消注释（invoke 用于调用后端命令）
// import { invoke } from "@tauri-apps/api/core";
// TODO: 完成填空后取消注释（listen 用于监听后端广播事件）
// import { listen } from "@tauri-apps/api/event";
// TODO: 完成填空后把 useState 加回导入（用于声明计数器状态）
import { useEffect } from "react";

export default function App() {
  // === 步骤 1: 声明计数器状态 ————————————————————————————————————
  // TODO: 用 useState 声明 count（number，初始 0）与 setCount：
  //   const [count, setCount] = useState<number>(0);
  // 提示: count 用于渲染，setCount 更新状态并触发重渲染
  // 当前为占位（完成填空后删除下面两行）
  let count: number = 0;
  const setCount = (_next: number) => {
    count = _next;
  };

  // === 步骤 2: 事件监听 hook 化 ——————————————————————————————————
  // TODO: 在 useEffect 中监听后端广播 "counter-changed"：
  //   let unlisten: (() => void) | undefined;
  //   listen<number>("counter-changed", (event) => setCount(event.payload))
  //     .then((fn) => { unlisten = fn; });
  //   return () => { unlisten?.(); };  // 组件卸载时取消监听（防泄漏）
  // 提示: import { listen } from "@tauri-apps/api/event"；
  //       useEffect 的返回值就是 cleanup 函数，在卸载/依赖变化时执行
  // 当前为占位（完成填空后替换）
  useEffect(() => {
    return () => {};
  }, []);

  const handleIncrement = async () => {
    try {
      // === 步骤 3: 调用 increment 命令 ————————————————————————————
      // TODO: const next = await invoke<number>("increment");
      //       setCount(next);
      // 提示: invoke 来自 "@tauri-apps/api/core"；命令返回最新计数（u32）
      // 当前为占位：本地自增演示（完成填空后替换为后端 invoke）
      setCount(count + 1);
    } catch (e) {
      console.error(e);
    }
  };

  const handleReset = async () => {
    try {
      // === 步骤 4: 调用 reset_counter 命令 ————————————————————————
      // TODO: const next = await invoke<number>("reset_counter");
      //       setCount(next);
      // 提示: reset_counter 把后端计数置 0 并返回
      // 当前为占位：本地清零演示（完成填空后替换为后端 invoke）
      setCount(0);
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
        <p className="status">
          当前计数:{" "}
          {/* === 步骤 5: 渲染当前计数 —————————————————————————————————— */}
          {/* TODO: 在花括号中嵌入 count 的值，例如：当前计数: {count} */}
          {"—"}
        </p>
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