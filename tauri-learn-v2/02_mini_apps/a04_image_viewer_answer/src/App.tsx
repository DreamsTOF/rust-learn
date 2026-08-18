// ============================================================
// 练习 A04: 图片查看器 —— 答案版
// 目标: 拖放、静态资源（asset 协议）、窗口操作
// ============================================================

import { useEffect, useState } from "react";
import { convertFileSrc, invoke } from "@tauri-apps/api/core";
import { getCurrentWebview } from "@tauri-apps/api/webview";
import { getCurrentWindow, LogicalSize } from "@tauri-apps/api/window";

const IMAGE_EXTS = ["png", "jpg", "jpeg", "gif", "bmp", "webp"];

function isImagePath(p: string): boolean {
  const ext = p.split(".").pop()?.toLowerCase();
  return ext ? IMAGE_EXTS.includes(ext) : false;
}

export default function App() {
  const [images, setImages] = useState<string[]>([]);
  const [index, setIndex] = useState(0);
  const [status, setStatus] = useState("拖图片或文件夹进来");
  const [alwaysOnTop, setAlwaysOnTop] = useState(false);
  const [fullscreen, setFullscreen] = useState(false);

  // 拖放：图片文件直接收；文件夹交给后端列目录
  useEffect(() => {
    const un = getCurrentWebview().onDragDropEvent(async (event) => {
      if (event.payload.type !== "drop") return;
      const paths = event.payload.paths;
      const imagePaths: string[] = [];
      for (const p of paths) {
        if (isImagePath(p)) {
          imagePaths.push(p);
        } else {
          // 不是图片 → 当作文件夹，交给后端列出其中的图片
          try {
            const listed = await invoke<string[]>("list_images", { dir: p });
            imagePaths.push(...listed);
          } catch (e) {
            setStatus(`不是图片也不是目录: ${p}`);
          }
        }
      }
      if (imagePaths.length > 0) {
        setImages(imagePaths);
        setIndex(0);
        setStatus(`${imagePaths.length} 张图片`);
      }
    });
    return () => {
      un.then((f) => f());
    };
  }, []);

  const current = images[index];

  async function resizeBy(ratio: number) {
    const win = getCurrentWindow();
    const size = await win.innerSize();
    await win.setSize(
      new LogicalSize(Math.round(size.width * ratio), Math.round(size.height * ratio))
    );
  }

  async function toggleAlwaysOnTop() {
    const next = !alwaysOnTop;
    setAlwaysOnTop(next);
    await getCurrentWindow().setAlwaysOnTop(next);
  }

  async function toggleFullscreen() {
    const next = !fullscreen;
    setFullscreen(next);
    await getCurrentWindow().setFullscreen(next);
  }

  return (
    <main className="app">
      <header>
        <h1>练习 A04: 图片查看器（答案）</h1>
        <div className="tools">
          <button
            onClick={() => setIndex((i) => Math.max(0, i - 1))}
            disabled={images.length === 0}
          >
            上一张
          </button>
          <span className="counter">
            {images.length ? `${index + 1} / ${images.length}` : "-"}
          </span>
          <button
            onClick={() => setIndex((i) => Math.min(images.length - 1, i + 1))}
            disabled={images.length === 0}
          >
            下一张
          </button>
          <button onClick={() => resizeBy(1.25)}>放大</button>
          <button onClick={() => resizeBy(0.8)}>缩小</button>
          <button onClick={async () => getCurrentWindow().center()}>居中</button>
          <button onClick={toggleAlwaysOnTop}>
            {alwaysOnTop ? "取消置顶" : "置顶"}
          </button>
          <button onClick={toggleFullscreen}>
            {fullscreen ? "退出全屏" : "全屏"}
          </button>
        </div>
      </header>

      <div className="viewer">
        {current ? (
          <img src={convertFileSrc(current)} alt="" draggable={false} />
        ) : (
          <p className="placeholder">拖入图片或图片文件夹</p>
        )}
      </div>

      <div className="filmstrip">
        {images.map((p, i) => (
          <img
            key={p}
            src={convertFileSrc(p)}
            className={i === index ? "active" : ""}
            onClick={() => setIndex(i)}
            alt=""
          />
        ))}
      </div>

      <p className="status">{status}</p>
    </main>
  );
}
