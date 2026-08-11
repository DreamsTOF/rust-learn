// ============================================================
// 练习 E05: 参数与返回值
// 目标: 掌握命令参数（字符串/数字/布尔/Vec/结构体）与 serde 序列化
// TODO: 按照注释提示补全代码
// ============================================================

// TODO: 完成填空后取消注释（invoke 用于调用后端命令）
// import { invoke } from "@tauri-apps/api/core";

// 与后端 Summary 对应的 TS 接口（camelCase ↔ snake_case 自动转换）
interface Summary {
  textLength: number;
  doubled: number;
  reversedFlag: boolean;
  itemCount: number;
  total: number;
}

const form = document.querySelector<HTMLFormElement>("#form");
const outputEl = document.querySelector<HTMLPreElement>("#output");

form!.addEventListener("submit", async (e) => {
  e.preventDefault();

  try {
    // === 步骤 1: 调用后端命令 ————————————————————————————————
    // TODO: 提取表单数据并调用 analyze 命令，一次传入 5 种类型参数：
    //   const fd = new FormData(form!);
    //   const summary = await invoke<Summary>("analyze", {
    //     text  : (fd.get("text") as string) || ""        → String
    //     number: Number(fd.get("number")) || 0           → i32
    //     flag  : fd.get("flag") === "on"                 → bool
    //     items : (fd.get("items") as string || "").split(",").map(s => s.trim()).filter(Boolean) → Vec
    //     calc  : { a: Number(fd.get("a")) || 0, b: Number(fd.get("b")) || 0 } → 结构体
    //   });
    // 提示: 参数名 camelCase，与 Rust 参数名对应
    // 当前为占位数据（保持可编译），完成填空后将显示真实结果
    const summary: Summary = {
      textLength: 0,
      doubled: 0,
      reversedFlag: false,
      itemCount: 0,
      total: 0,
    };

    // === 步骤 2: 展示结果 ——————————————————————————————————
    // TODO: 把 summary 以 JSON 格式显示到 #output
    // 提示: outputEl!.textContent = JSON.stringify(summary, null, 2);
    outputEl!.textContent = JSON.stringify(summary, null, 2);
  } catch (err) {
    outputEl!.textContent = `调用失败: ${err}`;
  }
});