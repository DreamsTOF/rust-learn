# 前端速成：Vanilla TS（给 Vue 开发者）

> 阶段 1 与八道小菜的第一道菜（a01）用 **Vanilla TS**（原生 TypeScript 操作 DOM，不带框架），先把"前后端通电话"这件事搞懂，再用 React 不迟。如果你熟悉 Vue，下面用 Vue 语法一一对照，几分钟就能上手。

## 找元素：querySelector

Vue 用 `ref` 拿模板里的元素。Vanilla TS 用 `querySelector` 按选择器找：

```typescript
const btnEl = document.querySelector<HTMLButtonElement>("#increment");
//                       ^^^^^^^^^^^^^^^^^^^^^^^^^^ ^^^^^^^^^^^
//                       告诉 TS"找到的是按钮"     CSS 选择器（按 id 找）
```

- `#increment`：按 id 找元素（`#` 前缀）。找不到时返回 `null`，所以下面用 `!` 非空断言："我保证它存在"
- `<HTMLButtonElement>`：泛型，给 TS 类型提示，`btnEl.addEventListener` 才不报错

```typescript
// Vue 对照：const btnEl = ref<HTMLButtonElement | null>(null)
```

## 更新内容：textContent vs innerHTML

| | `textContent` | `innerHTML` |
|---|---|---|
| 把字符串当什么 | 纯文字 | HTML 代码解析 |
| 适合 | 显示数字、状态文字 | 渲染带标签的列表 |

```typescript
valueEl!.textContent = String(count);        // 纯文字：显示数字
listEl!.innerHTML = items.map(...).join(""); // HTML：渲染列表
```

> 坑：展示用户输入或后端返回的字符串时，用 `textContent`；用 `innerHTML` 会把内容当 HTML 解析，`<` `>` 等字符会出问题。

## 样式类：className

Vue 用 `:class`。Vanilla 用 `className` 整体替换 class 属性：

```typescript
statusEl!.className = "status ok";   // 等价于 :class="{ ok: true }"
```

## 事件：addEventListener

Vue 用 `@click`。Vanilla 用 `addEventListener`：

```typescript
btnEl!.addEventListener("click", increment);   // 点击按钮 → 执行 increment 函数
```

## 异步：async/await 与错误兜底

Vue 里 `await invoke(...)` 怎么写，这里就怎么写：

```typescript
async function increment() {
  try {
    const next = await invoke<number>("count_up", { current: count });
    //                            ↑ 泛型：告诉 TS 返回值类型
  } catch (e) {
    // 后端报错、命令未注册、参数对不上……都会走到这里
  }
}
```

- `await`：等结果回来。不 await 拿到的是 Promise 对象而不是数据
- `try/catch`：调用失败时兜底展示错误，而不是白屏

## 模板字符串与数组渲染

Vue 用 `v-for` + `{{ }}`。Vanilla 用 `map().join()` + 反引号模板字符串：

```typescript
const lines = ["src/", "src-tauri/"];
lines.map((line) => `<li>${line}</li>`).join("");
//    ↑ 把每个元素变成一段字符串        ↑ 拼成一个大字符串（不 join 会带逗号！）
```

## 小结

| 你要做的事 | Vue | Vanilla TS |
|----------|-----|-----------|
| 找元素 | `ref()` | `document.querySelector` |
| 显示文本 | `{{ }}` | `el.textContent` |
| 渲染列表 | `v-for` | `el.innerHTML = items.map(...).join("")` |
| 样式切换 | `:class` | `el.className` |
| 事件 | `@click` | `el.addEventListener` |
| 调后端 | `await invoke()` | `await invoke()`（一样） |
