 // ============================================================
 // 模板: Minimal (Vanilla JS)
 //
 // 这是一个最小化 Tauri 前端模板
 // ============================================================
 import { invoke } from "@tauri-apps/api/core";
 
 document.getElementById("app").innerHTML = `
   <div style="text-align:center;padding:2rem;font-family:sans-serif;">
     <h1>${await invoke("greet", { name: "Tauri" })}</h1>
     <p>Tauri v2 练习项目模板</p>
   </div>
 `;
