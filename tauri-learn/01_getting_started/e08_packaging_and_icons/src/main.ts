// ============================================================
// 练习 E08: 打包与图标
// 目标: 理解 bundle 产物、图标与 identifier 规范
// TODO: 按照注释提示补全代码
// ============================================================

// TODO: 完成填空后取消注释（invoke 用于调用后端命令）
// import { invoke } from "@tauri-apps/api/core";

// 与后端 BundleInfo 对应的 TS 接口（camelCase ↔ snake_case 自动转换）
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
  // === 步骤 1: 调用后端命令 ————————————————————————————————
  // TODO: 改为调用真实命令：
  //   const info = await invoke<BundleInfo>("bundle_info");
  // 当前为占位数据（保持可编译），完成填空后将显示真实打包信息
  const info: BundleInfo = {
    identifier: "",
    productName: "",
    version: "",
    iconFiles: [],
  };

  // === 步骤 2: 渲染打包信息 ————————————————————————————————
  // TODO: 把 info 的字段写入对应元素
  identEl!.textContent = info.identifier;
  nameEl!.textContent = info.productName;
  versionEl!.textContent = info.version;
  // TODO: 把 iconFiles 渲染为 <li><code>...</code></li> 列表
  // 提示: iconListEl!.innerHTML = info.iconFiles.map((f) => `<li><code>${f}</code></li>`).join("");
  iconListEl!.innerHTML = "";
}

render().catch((e) => {
  identEl!.textContent = `调用失败: ${e}`;
});