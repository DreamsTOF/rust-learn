// ============================================================
// 练习 E47: 打包发布（packaging）
// 目标: 读取打包元信息，理解多平台产物与发布流程
// 知识点: invoke / bundle_info / 多平台产物 / 体积优化
// ============================================================

import { invoke } from "@tauri-apps/api/core";

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
    const info = await invoke<BundleInfo>("bundle_info");
    infoEl!.innerHTML = [
      ["identifier", info.identifier],
      ["productName（product_name）", info.product_name],
      ["version", info.version],
      ["platform（std::env::consts::OS）", info.platform],
    ]
      .map(
        ([k, v]) =>
          `<tr><th>${k}</th><td><code>${v}</code></td></tr>`
      )
      .join("");
  } catch (e) {
    infoEl!.innerHTML = `<tr><th>读取失败</th><td>${e}</td></tr>`;
  }
}

loadInfo().catch((e) => {
  infoEl!.innerHTML = `<tr><th>加载失败</th><td>${e}</td></tr>`;
});