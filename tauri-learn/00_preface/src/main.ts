 // ============================================================
 // Tauri v2 练习项目 — 导航首页
 //
 // 显示所有章节的练习列表导航
 // ============================================================
 
 import { invoke } from "@tauri-apps/api/core";
 
 const app = document.getElementById("app")!;
 
 const chapters = [
   {
     id: "01_environment",
     title: "第 1 章：环境准备",
     count: 10,
     desc: "搭建 Rust、Node.js、Tauri CLI 开发环境",
   },
   {
     id: "02_basics",
     title: "第 2 章：基础入门",
     count: 20,
     desc: "创建第一个 Tauri 应用，理解项目结构",
   },
   {
     id: "03_commands_ipc",
     title: "第 3 章：命令与 IPC",
     count: 65,
     desc: "命令系统、异步 IPC、依赖注入、错误处理",
   },
   {
     id: "04_state_config",
     title: "第 4 章：状态管理与配置",
     count: 35,
     desc: "Managed State、配置管理、路径 API",
   },
   {
     id: "05_events_lifecycle",
     title: "第 5 章：事件与生命周期",
     count: 30,
     desc: "事件系统、应用生命周期、窗口事件",
   },
   {
     id: "06_frontend",
     title: "第 6 章：前端集成",
     count: 40,
     desc: "Vite/React/Vue/Svelte 集成",
   },
   {
     id: "07_plugins",
     title: "第 7 章：插件系统",
     count: 70,
     desc: "FS/Dialog/Shell/SQL/Store 等官方插件",
   },
   {
     id: "08_window_menu_tray",
     title: "第 8 章：窗口、菜单与托盘",
     count: 35,
     desc: "多窗口管理、菜单栏、系统托盘",
   },
   {
     id: "09_testing",
     title: "第 9 章：验证与测试",
     count: 15,
     desc: "单元测试、集成测试、E2E 测试",
   },
   {
     id: "10_advanced",
     title: "第 10 章：高级主题",
     count: 35,
     desc: "安全模型、打包发布、移动端开发",
   },
 ];
 
 async function main() {
   const greeting = await invoke<string>("greet", { name: "学习者" });
 
   const chapterCards = chapters
     .map(
       (ch) => `
       <div class="chapter-card">
         <h2>${ch.title}</h2>
         <p class="desc">${ch.desc}</p>
         <span class="count">${ch.count} 道题</span>
       </div>
     `
     )
     .join("");
 
   app.innerHTML = `
     <div class="container">
       <header>
         <h1>${greeting}</h1>
         <p class="subtitle">Tauri v2 从入门到实战 · 共 355+ 道练习题 + 80 步终极项目</p>
       </header>
       <div class="grid">
         ${chapterCards}
       </div>
       <footer>
         <p>使用 <code>cd chapter/eNN_name && cargo tauri dev</code> 启动对应练习</p>
       </footer>
     </div>
   `;
 }
 
 main();
