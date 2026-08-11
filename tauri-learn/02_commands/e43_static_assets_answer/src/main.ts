// ============================================================
// 练习 E43: 静态资源
// 目标: 掌握 public/、src/assets、asset 协议三种资源使用方式
// 知识点: public 静态资源 / convertFileSrc / resource_dir / invoke
// ============================================================

import { invoke } from "@tauri-apps/api/core";

const infoBtn = document.querySelector<HTMLButtonElement>("#resource-btn");
const resultEl = document.querySelector<HTMLParagraphElement>("#result");

// 资源使用方式小结（详见 index.html 说明卡片）：
// 1. public/ 下的资源 → <img src="/tauri-logo.svg"> 直接引用（打包原样复制到 dist）
// 2. src/assets 下的资源 → import 引用，打包时自动带内容 hash
// 3. 大文件/动态文件（如用户选择的文件）→ asset 协议：
//    import { convertFileSrc } from "@tauri-apps/api/core";
//    const url = convertFileSrc(filePath);  // 把磁盘路径转成 asset:// 可加载 URL

// 查看 resource 目录（打包后存放额外资源的位置）
infoBtn!.addEventListener("click", async () => {
  try {
    const dir = await invoke<string>("resource_info");
    resultEl!.textContent = `resource 目录: ${dir}`;
    resultEl!.className = "status ok";
  } catch (e) {
    resultEl!.textContent = `查询失败: ${e}`;
    resultEl!.className = "status err";
  }
});