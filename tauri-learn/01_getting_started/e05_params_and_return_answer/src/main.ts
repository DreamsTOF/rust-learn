// ============================================================
// 练习 E05: 参数与返回值
// 目标: 掌握命令参数（字符串/数字/布尔/Vec/结构体）与 serde 序列化
// 知识点: invoke 多类型传参 / 结构体参数 / 结果展示
// ============================================================

import { invoke } from "@tauri-apps/api/core";

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
  const fd = new FormData(form!);

  try {
    // 一次性传入 5 种类型的参数：String / number / boolean / string[] / 嵌套对象
    const summary = await invoke<Summary>("analyze", {
      text: (fd.get("text") as string) || "",
      number: Number(fd.get("number")) || 0,
      flag: fd.get("flag") === "on",
      items: ((fd.get("items") as string) || "")
        .split(",")
        .map((s) => s.trim())
        .filter(Boolean),
      calc: {
        a: Number(fd.get("a")) || 0,
        b: Number(fd.get("b")) || 0,
      },
    });

    outputEl!.textContent = JSON.stringify(summary, null, 2);
  } catch (err) {
    outputEl!.textContent = `调用失败: ${err}`;
  }
});