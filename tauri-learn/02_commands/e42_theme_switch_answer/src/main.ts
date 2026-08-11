// ============================================================
// 练习 E42: 主题切换
// 目标: 用 CSS 变量实现浅色/深色两套主题，支持跟随系统与手动切换
// 知识点: CSS 变量 / prefers-color-scheme / matchMedia / localStorage
// ============================================================

// 本练习无 Rust 命令：纯前端主题切换。
// 主题变量定义见 src/styles.css，切换逻辑见下方 main.ts。

type ThemeMode = "light" | "dark" | "auto";

const applyTheme = (mode: ThemeMode) => {
  // dark = 手动深色，或 auto 且系统偏好深色
  const dark =
    mode === "dark" ||
    (mode === "auto" && window.matchMedia("(prefers-color-scheme: dark)").matches);
  // 在 <html> 上设置 data-theme 属性，CSS 用 [data-theme='dark'] 覆盖变量
  document.documentElement.dataset.theme = dark ? "dark" : "light";
  // 记忆选择：重启应用后仍是上次的选择
  localStorage.setItem("theme-mode", mode);
};

// 初始化：读取 localStorage（无则 auto），并应用主题
const saved = (localStorage.getItem("theme-mode") as ThemeMode) || "auto";
applyTheme(saved);

// auto 模式下，系统主题切换时实时跟随
window
  .matchMedia("(prefers-color-scheme: dark)")
  .addEventListener("change", () => {
    if ((localStorage.getItem("theme-mode") as ThemeMode) === "auto") {
      applyTheme("auto");
    }
  });

// 按钮组：跟随系统 / 浅色 / 深色
document.querySelectorAll<HTMLButtonElement>("[data-theme-btn]").forEach((btn) => {
  btn.addEventListener("click", () => {
    applyTheme(btn.dataset.themeBtn as ThemeMode);
  });
});