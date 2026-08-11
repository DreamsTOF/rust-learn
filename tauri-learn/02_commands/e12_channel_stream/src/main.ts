// ============================================================
// 练习 E12: Channel 流式传输
// 目标: 创建 Channel 接收后端流式推送并实时渲染
// TODO: 按照注释提示补全代码
// ============================================================

// TODO: 完成填空后取消注释（Channel 用于创建双向通道，invoke 调用后端命令）
// import { Channel, invoke } from "@tauri-apps/api/core";

// 与 Rust 端 StreamItem 对应
interface StreamItem {
  step: number;
  label: string;
}

const progressBtn = document.querySelector<HTMLButtonElement>("#progress-btn");
const streamBtn = document.querySelector<HTMLButtonElement>("#stream-btn");
const progressFill = document.querySelector<HTMLDivElement>("#progress-fill");
const progressText = document.querySelector<HTMLSpanElement>("#progress-text");
const streamList = document.querySelector<HTMLUListElement>("#stream-list");
const resultEl = document.querySelector<HTMLParagraphElement>("#result");

// 进度流：Channel<number> 接收 0-100 的进度值
progressBtn!.addEventListener("click", async () => {
  try {
    // === 步骤 4: 创建进度通道并绑定 onmessage ————————————————————————————
    // TODO: 创建 Channel<number> 并设置 onmessage（更新进度条）：
    //   const ch = new Channel<number>();
    //   ch.onmessage = (msg) => {
    //     progressFill!.style.width = `${msg}%`;
    //     progressText!.textContent = `${msg}%`;
    //   };
    // === 步骤 5: 调用后端并传入通道 ————————————————————————————————————
    // TODO: await invoke("start_progress", { channel: ch });
    // 提示: invoke 参数名 channel 与 Rust 参数一致
    // 占位（完成填空后删除）：
    const ch = {} as { onmessage: (msg: number) => void };
    ch.onmessage = (msg) => {
      progressFill!.style.width = `${msg}%`;
      progressText!.textContent = `${msg}%`;
    };
    await new Promise<void>((resolve) => setTimeout(resolve, 0));
    resultEl!.textContent = "进度推送完成";
    resultEl!.className = "status ok";
  } catch (e) {
    resultEl!.textContent = `调用失败: ${e}`;
    resultEl!.className = "status err";
  }
});

// 消息流：Channel<StreamItem> 接收结构化消息并追加到列表
streamBtn!.addEventListener("click", async () => {
  try {
    // === 步骤 6: 创建消息流通道并绑定 onmessage ————————————————————————————
    // TODO: 创建 Channel<StreamItem>，onmessage 中把每条消息追加到 #stream-list：
    //   const ch = new Channel<StreamItem>();
    //   ch.onmessage = (msg) => {
    //     const li = document.createElement("li");
    //     li.className = "ok";
    //     li.innerHTML = `<span class="badge">${msg.step}</span>${msg.label}`;
    //     streamList!.appendChild(li);
    //   };
    // === 步骤 7: 调用后端并传入通道 ————————————————————————————————————
    // TODO: await invoke("start_stream", { channel: ch });
    // 占位（完成填空后删除）：
    const ch = {} as { onmessage: (msg: StreamItem) => void };
    ch.onmessage = (msg) => {
      const li = document.createElement("li");
      li.className = "ok";
      li.innerHTML = `<span class="badge">${msg.step}</span>${msg.label}`;
      streamList!.appendChild(li);
    };
    await new Promise<void>((resolve) => setTimeout(resolve, 0));
    resultEl!.textContent = "消息流推送完成";
    resultEl!.className = "status ok";
  } catch (e) {
    resultEl!.textContent = `调用失败: ${e}`;
    resultEl!.className = "status err";
  }
});