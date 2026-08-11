// ============================================================
// 练习 E47: 打包发布（packaging）
// 目标: 读取打包元信息，理解多平台产物与发布流程
// 知识点: invoke / bundle_info / 多平台产物 / 体积优化
// TODO: 按照注释提示补全代码
// ============================================================

// TODO: 完成填空后取消注释（invoke 用于调用后端命令）
// import { invoke } from "@tauri-apps/api/core";

// 与 Rust 侧 BundleInfo 结构体对应的类型
interface BundleInfo {
  identifier: string;
  product_name: string;
  version: string;
  platform: string;
}

const infoEl = document.querySelector<HTMLTableSectionElement>("#info-body");

async function loadInfo(): Promise<void> {
  try {
    // === 步骤 3: 调用 bundle_info 命令 ————————————————————————
    // TODO: 调用后端命令并把结果赋给 info 变量
    // 提示: const info = await invoke<BundleInfo>("bundle_info");
    // 当前为占位对象（保持可编译），完成填空后将显示真实元信息
    const info: BundleInfo = {
      identifier: "com.example.placeholder",
      product_name: "placeholder",
      version: "0.0.0",
      platform: "unknown",
    };

    // === 步骤 4: 展示到表格 ——————————————————————————————————
    // TODO: 把 info 的四个字段渲染到 #info-body 表格
    // 提示: [["identifier", info.identifier], ["productName", info.product_name], ...]
    //       .map(([k, v]) => `<tr><th>${k}</th><td><code>${v}</code></td></tr>`).join("")
    infoEl!.innerHTML = [
      ["identifier", info.identifier],
      ["productName（product_name）", info.product_name],
      ["version", info.version],
      ["platform（std::env::consts::OS）", info.platform],
    ]
      .map(([k, v]) => `<tr><th>${k}</th><td><code>${v}</code></td></tr>`)
      .join("");
  } catch (e) {
    infoEl!.innerHTML = `<tr><th>读取失败</th><td>${e}</td></tr>`;
  }
}

loadInfo().catch((e) => {
  infoEl!.innerHTML = `<tr><th>加载失败</th><td>${e}</td></tr>`;
});