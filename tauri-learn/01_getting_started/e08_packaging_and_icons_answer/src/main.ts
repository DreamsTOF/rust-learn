// ============================================================
// 练习 E08: 打包与图标
// 目标: 理解 bundle 产物、图标与 identifier 规范
// 知识点: invoke() / 打包元数据展示 / 产物说明
// ============================================================

import { invoke } from "@tauri-apps/api/core";

// 与后端 BundleInfo 对应的 TS 接口
interface BundleInfo {
  identifier: string;
  productName: string;
  version: string;
  iconFiles: string[];
}

const identEl = document.querySelector<HTMLSpanElement>("#identifier");
const nameEl = document.querySelector<HTMLSpanElement>("#product-name");
const versionEl = document.querySelector<HTMLSpanElement>("#version");
const iconListEl = document.querySelector<HTMLUListElement>("#icon-list");

async function render() {
  const info = await invoke<BundleInfo>("bundle_info");

  identEl!.textContent = info.identifier;
  nameEl!.textContent = info.productName;
  versionEl!.textContent = info.version;
  iconListEl!.innerHTML = info.iconFiles
    .map((f) => `<li><code>${f}</code></li>`)
    .join("");
}

render().catch((e) => {
  identEl!.textContent = `调用失败: ${e}`;
});