 // ============================================================
 // 模板: Vite + TypeScript
 //
 // 标准 Tauri v2 练习模板
 // ============================================================
 import { invoke } from "@tauri-apps/api/core";
 
 const app = document.getElementById("app")!;
 
 async function main() {
   const greeting = await invoke<string>("greet", { name: "Tauri" });
   app.innerHTML = `
     <div style="text-align:center;padding:2rem;font-family:sans-serif;">
       <h1>${greeting}</h1>
       <p>Tauri v2 练习项目模板 (Vite + TypeScript)</p>
     </div>
   `;
 }
 
 main();
