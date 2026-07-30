/**
 * Leptos 练习沙箱 — 本地代码编辑器
 *
 * 和 trunk serve 一起运行:
 *   1. 终端 1: cd sandbox && trunk serve
 *   2. 终端 2: node sandbox/editor-server.js
 *   3. 打开 http://localhost:3002/?e=01
 *
 * 编辑代码 → 点击 "Save & Run" → trunk 增量编译 → iframe 自动刷新
 */

const http = require('http');
const fs = require('fs');
const path = require('path');

const EXERCISES_DIR = path.join(__dirname, 'src', 'exercises');
const TRUNK_PORT = 3001;
const EDITOR_PORT = 3002;

// 练习编号 → 文件名映射（扩展到 20 个练习）
const EXERCISE_NAMES = {
  '01': 'e01_hello_world',
  '01_answer': 'e01_hello_world_answer',
  '02': 'e02_text_nodes',
  '02_answer': 'e02_text_nodes_answer',
  '03': 'e03_html_elements',
  '03_answer': 'e03_html_elements_answer',
  '04': 'e04_element_nesting',
  '04_answer': 'e04_element_nesting_answer',
  '05': 'e05_component_definition',
  '05_answer': 'e05_component_definition_answer',
  '06': 'e06_component_nesting',
  '06_answer': 'e06_component_nesting_answer',
  '07': 'e07_fragment_syntax',
  '07_answer': 'e07_fragment_syntax_answer',
  '08': 'e08_comments',
  '08_answer': 'e08_comments_answer',
  '09': 'e09_rust_expressions',
  '09_answer': 'e09_rust_expressions_answer',
  '10': 'e10_block_expressions',
  '10_answer': 'e10_block_expressions_answer',
  '11': 'e11_conditional_if',
  '11_answer': 'e11_conditional_if_answer',
  '12': 'e12_match_in_view',
  '12_answer': 'e12_match_in_view_answer',
  '13': 'e13_index_method_call',
  '13_answer': 'e13_index_method_call_answer',
  '14': 'e14_builder_pattern',
  '14_answer': 'e14_builder_pattern_answer',
  '15': 'e15_browser_devtools',
  '15_answer': 'e15_browser_devtools_answer',
  '16': 'e16_svg_elements',
  '16_answer': 'e16_svg_elements_answer',
  '17': 'e17_raw_html',
  '17_answer': 'e17_raw_html_answer',
  '18': 'e18_fragment_multi_root',
  '18_answer': 'e18_fragment_multi_root_answer',
  '19': 'e19_dynamic_tag_name',
  '19_answer': 'e19_dynamic_tag_name_answer',
  '20': 'e20_builder_advanced',
  '20_answer': 'e20_builder_advanced_answer',
};

// 简单的 HTML 模板：左编辑 + 右预览
function renderEditorPage(exerciseKey, fileName, fileContent) {
  const isAnswer = exerciseKey.endsWith('_answer');
  const exNum = exerciseKey.replace('_answer', '');
  const previewUrl = `http://localhost:${TRUNK_PORT}/?e=${exerciseKey}`;

  return `<!DOCTYPE html>
<html lang="zh-CN">
<head>
<meta charset="utf-8"/>
<meta name="viewport" content="width=device-width, initial-scale=1.0"/>
<title>练习 ${exNum} ${isAnswer ? '(答案)' : ''} — Leptos 沙箱编辑器</title>
<style>
  *, *::before, *::after { box-sizing: border-box; margin: 0; padding: 0; }
  body { font-family: system-ui, -apple-system, sans-serif; background: #0d1117; color: #c9d1d9; height: 100vh; display: flex; flex-direction: column; }
  .toolbar { display: flex; align-items: center; gap: 0.8rem; padding: 0.6rem 1rem; background: #161b22; border-bottom: 1px solid #30363d; flex-shrink: 0; }
  .toolbar h1 { font-size: 1rem; font-weight: 600; }
  .toolbar .nav { display: flex; gap: 0.4rem; }
  .toolbar .nav a { color: #58a6ff; text-decoration: none; font-size: 0.85rem; padding: 0.2rem 0.5rem; border-radius: 4px; }
  .toolbar .nav a:hover { background: #1f2937; }
  .toolbar .spacer { flex: 1; }
  .btn { padding: 0.4rem 1rem; border: none; border-radius: 6px; cursor: pointer; font-size: 0.85rem; font-weight: 500; }
  .btn-save { background: #238636; color: #fff; }
  .btn-save:hover { background: #2ea043; }
  .btn-save:active { background: #1a7f37; }
  .status { font-size: 0.8rem; color: #8b949e; }
  .status.saving { color: #d29922; }
  .status.saved { color: #3fb950; }
  .main { display: flex; flex: 1; overflow: hidden; }
  .editor-pane { flex: 1; display: flex; flex-direction: column; border-right: 1px solid #30363d; min-width: 0; }
  .editor-pane textarea { flex: 1; width: 100%; border: none; outline: none; resize: none; padding: 1rem; font-family: 'JetBrains Mono', 'Fira Code', 'Cascadia Code', monospace; font-size: 13px; line-height: 1.6; background: #0d1117; color: #c9d1d9; tab-size: 4; }
  .editor-pane textarea:focus { background: #0d1117; }
  .preview-pane { flex: 1; display: flex; flex-direction: column; min-width: 0; }
  .preview-pane iframe { flex: 1; width: 100%; border: none; background: #fff; }
  .preview-label { padding: 0.3rem 0.8rem; font-size: 0.75rem; color: #8b949e; background: #161b22; border-bottom: 1px solid #30363d; flex-shrink: 0; }
  @media (max-width: 768px) { .main { flex-direction: column; } .editor-pane { border-right: none; border-bottom: 1px solid #30363d; min-height: 50vh; } }
</style>
</head>
<body>
<div class="toolbar">
  <h1>练习 ${exNum}</h1>
  <span style="font-size:0.8rem;color:#8b949e">${isAnswer ? '答案' : '练习'}</span>
  <div class="nav">
    <a href="/?e=${exerciseKey}">↻ 刷新</a>
    <a href="/?e=${exNum}_answer">${isAnswer ? '→ 练习' : '→ 答案'}</a>
  </div>
  <div class="spacer"></div>
  <span class="status" id="status">就绪</span>
  <button class="btn btn-save" onclick="saveCode()">Save & Run</button>
</div>
<div class="main">
  <div class="editor-pane">
    <textarea id="editor" spellcheck="false">${escapeHtml(fileContent)}</textarea>
  </div>
  <div class="preview-pane">
    <div class="preview-label">▶ 预览 (localhost:${TRUNK_PORT})</div>
    <iframe id="preview" src="${previewUrl}"></iframe>
  </div>
</div>
<script>
  const STATUS = document.getElementById('status');
  const EDITOR = document.getElementById('editor');
  const PREVIEW = document.getElementById('preview');

  // Ctrl+S 快捷键
  EDITOR.addEventListener('keydown', (e) => { if ((e.ctrlKey || e.metaKey) && e.key === 's') { e.preventDefault(); saveCode(); } });

  function saveCode() {
    STATUS.textContent = '保存中...';
    STATUS.className = 'status saving';
    const code = EDITOR.value;
    fetch('/save/${exerciseKey}', { method: 'POST', body: code })
      .then(r => r.text())
      .then(msg => {
        STATUS.textContent = msg === 'ok' ? '✓ 已保存，编译中...' : '✗ ' + msg;
        STATUS.className = 'status saved';
        // 刷新预览
        PREVIEW.src = '${previewUrl}' + '&_t=' + Date.now();
        setTimeout(() => { STATUS.textContent = '就绪'; STATUS.className = 'status'; }, 3000);
      })
      .catch(e => { STATUS.textContent = '✗ 保存失败: ' + e.message; });
  }

  function escapeHtml(s) { return s.replace(/&/g,'&amp;').replace(/</g,'&lt;').replace(/>/g,'&gt;'); }
</script>
</body>
</html>`;
}

function escapeHtml(text) {
  return text.replace(/&/g,'&amp;').replace(/</g,'&lt;').replace(/>/g,'&gt;');
}

const server = http.createServer((req, res) => {
  const url = new URL(req.url, `http://localhost:${EDITOR_PORT}`);
  const pathname = url.pathname;

  // POST /save/<exerciseKey> — 保存代码到文件
  if (req.method === 'POST' && pathname.startsWith('/save/')) {
    const exerciseKey = pathname.slice(6); // '/save/' 去掉
    const fileName = EXERCISE_NAMES[exerciseKey];
    if (!fileName) {
      res.writeHead(400); res.end('unknown exercise: ' + exerciseKey);
      return;
    }
    let body = '';
    req.on('data', c => body += c);
    req.on('end', () => {
      try {
        fs.writeFileSync(path.join(EXERCISES_DIR, fileName + '.rs'), body, 'utf-8');
        console.log('Saved:', fileName + '.rs');
        res.end('ok');
      } catch (e) {
        res.writeHead(500); res.end(e.message);
      }
    });
    return;
  }

  // GET /?e=01 — 编辑器页面
  const exerciseKey = url.searchParams.get('e') || '01';
  const fileName = EXERCISE_NAMES[exerciseKey];
  if (!fileName) {
    res.writeHead(404);
    res.end('Unknown exercise: ' + exerciseKey);
    return;
  }

  const filePath = path.join(EXERCISES_DIR, fileName + '.rs');
  let fileContent;
  try {
    fileContent = fs.readFileSync(filePath, 'utf-8');
  } catch (e) {
    fileContent = '// 文件未找到: ' + filePath;
  }

  res.writeHead(200, { 'Content-Type': 'text/html; charset=utf-8' });
  res.end(renderEditorPage(exerciseKey, fileName, fileContent));
});

server.listen(EDITOR_PORT, () => {
  console.log('');
  console.log('╔══════════════════════════════════════════════╗');
  console.log('║   Leptos 练习沙箱编辑器                      ║');
  console.log('╠══════════════════════════════════════════════╣');
  console.log(`║  编辑: http://localhost:${EDITOR_PORT}/?e=01    ║`);
  console.log(`║  预览: http://localhost:${TRUNK_PORT}/?e=01     ║`);
  console.log('╠══════════════════════════════════════════════╣');
  console.log('║  用法:                                      ║');
  console.log('║  1. 终端 1: cd sandbox && trunk serve       ║');
  console.log('║  2. 终端 2: node sandbox/editor-server.js   ║');
  console.log('║  3. 浏览器打开编辑器地址                     ║');
  console.log('║  4. 编辑代码 → Save & Run                    ║');
  console.log('╚══════════════════════════════════════════════╝');
  console.log('');
});
