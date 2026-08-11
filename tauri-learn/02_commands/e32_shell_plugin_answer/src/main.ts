// ============================================================
// 练习 E32: Shell（shell 插件）
// 目标: 用 @tauri-apps/plugin-shell 执行外部命令，读取 stdout/stderr
// 知识点: Command.create / execute / 超时 / scope 白名单
// ============================================================

import { Command } from "@tauri-apps/plugin-shell";

const echoBtn = document.querySelector<HTMLButtonElement>("#echo-btn");
const stderrBtn = document.querySelector<HTMLButtonElement>("#stderr-btn");
const timeoutBtn = document.querySelector<HTMLButtonElement>("#timeout-btn");
const deniedBtn = document.querySelector<HTMLButtonElement>("#denied-btn");
const resultEl = document.querySelector<HTMLPreElement>("#result");

// Output 对象字段: code / stdout / stderr（code 可能为 null，如被信号终止）
function formatOutput(label: string, out: { code: number | null; stdout: string; stderr: string }): string {
  return [
    `${label}`,
    `code:   ${out.code}`,
    `stdout: ${out.stdout.trim() || "(空)"}`,
    `stderr: ${out.stderr.trim() || "(空)"}`,
  ].join("\n");
}

echoBtn!.addEventListener("click", async () => {
  try {
    const out = await Command.create("echo", ["hello", "from", "shell"]).execute();
    resultEl!.textContent = formatOutput("echo hello from shell", out);
  } catch (e) {
    resultEl!.textContent = `执行失败: ${e}`;
  }
});

stderrBtn!.addEventListener("click", async () => {
  try {
    // cmd /C "echo err 1>&2"：把输出重定向到 stderr
    const out = await Command.create("cmd", ["/C", "echo err 1>&2"]).execute();
    resultEl!.textContent = formatOutput("stderr 重定向", out);
  } catch (e) {
    resultEl!.textContent = `执行失败: ${e}`;
  }
});

timeoutBtn!.addEventListener("click", async () => {
  try {
    // 前端 v2 API 无内置 timeout 选项：用 spawn() 拿到子进程句柄，
    // 2 秒后手动 kill 模拟超时终止（Rust 侧 CommandBuilder 才有 timeout()）
    const child = await Command.create("cmd", ["/C", "ping", "-n", "5", "127.0.0.1"]).spawn();
    await new Promise((r) => setTimeout(r, 2000));
    await child.kill();
    resultEl!.textContent = "已手动终止（2 秒超时）：ping 命令被 kill";
  } catch (e) {
    resultEl!.textContent = `命令超时或被终止:\n${e}`;
  }
});

deniedBtn!.addEventListener("click", async () => {
  try {
    const out = await Command.create("not_allowed_cmd_xyz").execute();
    resultEl!.textContent = formatOutput("未授权命令意外成功", out);
  } catch (e) {
    resultEl!.textContent = `scope 拒绝（命令不在白名单）:\n${e}`;
  }
});