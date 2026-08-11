// ============================================================
// 练习 E03: 运行与构建
// 目标: 理解 tauri dev / tauri build 与 devUrl / frontendDist
// TODO: 按照注释提示补全代码
// ============================================================

// TODO: 完成填空后取消注释（invoke 用于调用后端命令）
// import { invoke } from "@tauri-apps/api/core";

// 与后端 BuildInfo 对应的 TS 接口（camelCase 对应 Rust snake_case）
interface BuildInfo {
  devUrl: string | null;
  frontendDist: string;
  identifier: string;
  productName: string;
}

const devUrlEl = document.querySelector<HTMLSpanElement>("#dev-url");
const distEl = document.querySelector<HTMLSpanElement>("#frontend-dist");
const identEl = document.querySelector<HTMLSpanElement>("#identifier");
const nameEl = document.querySelector<HTMLSpanElement>("#product-name");

async function render() {
  // === 步骤 1: 调用后端命令 ————————————————————————————————
  // TODO: 改为调用真实命令：
  //   const info = await invoke<BuildInfo>("build_info");
  // 当前为占位数据（保持可编译），完成填空后将显示真实配置
  const info: BuildInfo = {
    devUrl: null,
    frontendDist: "",
    identifier: "",
    productName: "",
  };

  // === 步骤 2: 渲染配置项 ——————————————————————————————————
  // TODO: 把 info 的字段写入对应元素（devUrl 可能为 null）
  // 提示: devUrlEl!.textContent = info.devUrl ?? "（未配置，当前为构建模式）";
  devUrlEl!.textContent = info.devUrl ?? "（未配置，当前为构建模式）";
  // TODO: distEl!.textContent = info.frontendDist;
  distEl!.textContent = "";
  // TODO: identEl!.textContent = info.identifier;
  identEl!.textContent = "";
  // TODO: nameEl!.textContent = info.productName;
  nameEl!.textContent = "";
}

render().catch((e) => {
  devUrlEl!.textContent = `调用失败: ${e}`;
});