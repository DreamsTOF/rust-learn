// ============================================================
// 练习 E03: 运行与构建
// 目标: 理解 tauri dev / tauri build 与 devUrl / frontendDist
// 知识点: invoke() / 结构体返回类型 / AppHandle 注入
// ============================================================

import { invoke } from "@tauri-apps/api/core";

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
  const info = await invoke<BuildInfo>("build_info");

  devUrlEl!.textContent = info.devUrl ?? "（未配置，当前为构建模式）";
  distEl!.textContent = info.frontendDist;
  identEl!.textContent = info.identifier;
  nameEl!.textContent = info.productName;
}

render().catch((e) => {
  devUrlEl!.textContent = `调用失败: ${e}`;
});