// ============================================================
// 练习 E42: 主题切换
// 目标: 用 CSS 变量实现浅色/深色两套主题，支持跟随系统与手动切换
// 知识点: CSS 变量 / prefers-color-scheme / matchMedia / localStorage
// TODO: 按照注释提示补全代码
// ============================================================

// 本练习无 Rust 命令：纯前端主题切换。
// 主题变量定义见 src/styles.css，切换逻辑见下方 main.ts。

type ThemeMode = "light" | "dark" | "auto";

const applyTheme = (_mode: ThemeMode) => {
  // === 步骤 1: 计算目标主题 ————————————————————————————————————
  // TODO: 恢复参数名 mode 并补全 dark 的计算：
  //   const dark = mode === "dark" ||
  //     (mode === "auto" && window.matchMedia("(prefers-color-scheme: dark)").matches);
  // 提示: matchMedia 检测系统主题偏好；auto 模式下跟随系统
  // 当前为占位（完成填空后替换）
  const dark = false;

  // 在 <html> 上设置 data-theme 属性，CSS 用 [data-theme='dark'] 覆盖变量
  document.documentElement.dataset.theme = dark ? "dark" : "light";

  // === 步骤 2: 记忆当前选择 ————————————————————————————————————
  // TODO: 把 mode 写入 localStorage，重启应用后仍是上次选择：
  //   localStorage.setItem("theme-mode", mode);
  // 提示: localStorage 是浏览器持久化存储（Tauri WebView 同样可用）
  // 当前为占位（完成填空后替换）
  localStorage.setItem("theme-mode", "auto");
};

// === 步骤 3: 初始化主题 ————————————————————————————————————
// TODO: 读取 localStorage 中保存的模式（无则 "auto"）并应用：
//   const saved = (localStorage.getItem("theme-mode") as ThemeMode) || "auto";
//   applyTheme(saved);
// 提示: localStorage.getItem 返回 string | null，需要回退默认值
// 当前为占位（完成填空后替换）
applyTheme("auto");

// === 步骤 4: 跟随系统主题变化 ——————————————————————————————————
// TODO: 监听系统主题切换，仅 auto 模式下实时跟随：
//   window.matchMedia("(prefers-color-scheme: dark)")
//     .addEventListener("change", () => {
//       if ((localStorage.getItem("theme-mode") as ThemeMode) === "auto") {
//         applyTheme("auto");
//       }
//     });
// 提示: 手动选择 light/dark 后，系统主题变化不应影响当前选择
// 当前为占位（完成填空后替换）

// === 步骤 5: 绑定主题按钮 ————————————————————————————————————
// TODO: 给每个 [data-theme-btn] 按钮绑定点击事件：
//   document.querySelectorAll<HTMLButtonElement>("[data-theme-btn]").forEach((btn) => {
//     btn.addEventListener("click", () => {
//       applyTheme(btn.dataset.themeBtn as ThemeMode);
//     });
//   });
// 提示: btn.dataset.themeBtn 读取 data-theme-btn 属性值（"auto" | "light" | "dark"）
// 当前为占位（完成填空后替换）