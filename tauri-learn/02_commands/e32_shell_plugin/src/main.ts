// ============================================================
// 练习 E32: Shell（shell 插件）
// 目标: 用 @tauri-apps/plugin-shell 执行外部命令，读取 stdout/stderr
// 知识点: Command.create / execute / 超时 / scope 白名单
// TODO: 按照注释提示补全代码
// ============================================================

import { Command } from "@tauri-apps/plugin-shell";

// 占位引用：确保被挖空的插件 API 导入被使用（全部 TODO 完成后删除本行）
void [Command];

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
    // === 步骤 1: 创建并执行命令 ————————————————————————————
    // TODO: const out = await Command.create("echo", ["hello", "from", "shell"]).execute();
    // 提示: Command.create(程序名, 参数数组) 的程序名必须出现在 capabilities 白名单
    const out = null as { code: number; stdout: string; stderr: string } | null; // 占位

    // === 步骤 2: 读取结果字段 ——————————————————————————————
    // TODO: 用 formatOutput("echo hello from shell", out) 展示 code / stdout / stderr
    resultEl!.textContent = out === null ? "(占位)" : formatOutput("echo hello from shell", out);
  } catch (e) {
    resultEl!.textContent = `执行失败: ${e}`;
  }
});

stderrBtn!.addEventListener("click", async () => {
  try {
    // === 步骤 3: stderr 演示 ————————————————————————————————
    // TODO: const out = await Command.create("cmd", ["/C", "echo err 1>&2"]).execute();
    // 提示: cmd /C "echo err 1>&2" 把输出重定向到 stderr
    const out = null as { code: number; stdout: string; stderr: string } | null; // 占位
    resultEl!.textContent = out === null ? "(占位)" : formatOutput("stderr 重定向", out);
  } catch (e) {
    resultEl!.textContent = `执行失败: ${e}`;
  }
});

timeoutBtn!.addEventListener("click", async () => {
  try {
    // === 步骤 4: 超时演示 ——————————————————————————————————
    // TODO: 用 spawn() 启动 ping（约 4 秒），2 秒后手动 kill 模拟超时：
    //   const child = await Command.create("cmd", ["/C", "ping", "-n", "5", "127.0.0.1"]).spawn();
    //   await new Promise((r) => setTimeout(r, 2000));
    //   await child.kill();
    // 提示: 前端 v2 API 无内置 timeout 选项，用 spawn + kill 模拟；
    //       Rust 侧 CommandBuilder 才有 timeout()
    resultEl!.textContent = "（占位）已手动终止（2 秒超时）：ping 命令被 kill";
  } catch (e) {
    resultEl!.textContent = `命令超时或被终止:\n${e}`;
  }
});

deniedBtn!.addEventListener("click", async () => {
  try {
    // === 步骤 5: 未授权命令 ————————————————————————————————
    // TODO: const out = await Command.create("not_allowed_cmd_xyz").execute();
    // 提示: 该命令不在 capabilities 白名单，execute 会抛出 scope 拒绝错误
    const out = null as { code: number; stdout: string; stderr: string } | null; // 占位
    resultEl!.textContent = out === null ? "(占位)" : formatOutput("未授权命令意外成功", out);
  } catch (e) {
    resultEl!.textContent = `scope 拒绝（命令不在白名单）:\n${e}`;
  }
});