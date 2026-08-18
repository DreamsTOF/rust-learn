# 练习 A02 答案讲解：记事本

> **用法**：卡住时再看本页。每一处 TODO 按三步走——先看"练习版缺什么"，再看"答案版填了什么"，最后回查原理文档对应小节。
> **作业范围**：本练习只需动 2 个文件：`src-tauri/src/lib.rs`（后端）与 `src/App.tsx`（前端 React），共 10 处 TODO。

## 对照总览

| 文件 | 练习版状态 | 你缺的 |
|---|---|---|
| `src-tauri/src/lib.rs` | `note_path` / `note_file_path` 已给 | `load_note` 读、`save_note` 写、注册插件、登记命令 |
| `src/App.tsx` | React 界面已搭好 | import invoke + 3 处 invoke 调用 |

> **前端基础提示**：`useState` / `useEffect` / 受控组件不熟的话，回看 [《练习 A02》第 4 节：React](a02_notepad.md#sec-a02-react)。

## lib.rs TODO 1：load_note 读文件

### 练习版这里是什么

```rust
#[tauri::command]
fn load_note(app: AppHandle) -> Result<String, String> {
    let path = note_path(&app)?;
    if !path.exists() {
        return Ok(String::new()); // 首次打开：没有文件，就当空笔记
    }
    Ok(String::from("（TODO：读取文件内容）")) // ← 替换成你的代码
}
```

### 答案版填了什么

```rust
    let mut opts = OpenOptions::new();
    opts.read(true);
    let mut file = app
        .fs()
        .open(&path, opts)
        .map_err(|e| format!("打开文件失败：{e}"))?;
    let mut content = String::new();
    file.read_to_string(&mut content).map_err(|e| format!("读取失败：{e}"))?;
    Ok(content)
```

### 为什么

- `app.fs()` 是 fs 插件提供的访问器（`FsExt` trait）
- `OpenOptions::new().read(true)`：**只读**打开。注意 `OpenOptions` 的字段是私有的，**不能用结构体字面量** `{ read: true }`，必须用 builder 方法打开开关
- `read_to_string`：把文件读成一个 `String`；需要 `use std::io::Read`
- 每个可能失败的调用都 `map_err` 成人话 + `?` 提前返回
- **`if !path.exists()` 分支是业务判断**：文件不存在 ≠ 错误，而是"第一次打开"——返回空串

### 回查文档

[第 2 节：fs 插件读文件](a02_notepad.md#sec-a02-fs)、[第 3 节：Result 错误处理](a02_notepad.md#sec-a02-result)。

## lib.rs TODO 2：save_note 写文件

### 练习版这里是什么

```rust
#[tauri::command]
fn save_note(app: AppHandle, content: String) -> Result<(), String> {
    let path = note_path(&app)?;
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir).map_err(|e| format!("创建目录失败：{e}"))?;
    }
    Ok(()) // ← 替换成你的代码
}
```

### 答案版填了什么

```rust
    let mut opts = OpenOptions::new();
    opts.write(true).create(true).truncate(true);
    let mut file = app
        .fs()
        .open(&path, opts)
        .map_err(|e| format!("创建文件失败：{e}"))?;
    file.write_all(content.as_bytes()).map_err(|e| format!("写入失败：{e}"))?;
    Ok(())
```

### 为什么

- 三个开关一起开，含义依次是：**可写 + 不存在则创建 + 打开即清空**——"保存"= 整个覆盖，不留上次的残留
- `content.as_bytes()`：`String` 转字节再写
- `create_dir_all` 在前面先建好父目录，否则 `create: true` 也会因父目录缺失而失败
- 返回 `Ok(())`：保存成功不需要回传数据，前端知道"没报错"就够了

### 回查文档

[第 2 节：OpenOptions 三个开关](a02_notepad.md#sec-a02-fs)。

## lib.rs TODO 3-4：注册插件 + 登记命令

### 练习版这里是什么

```rust
tauri::Builder::default()
    // TODO: .plugin(tauri_plugin_fs::init())
    .invoke_handler(tauri::generate_handler![
        // TODO: note_file_path, load_note, save_note,
    ])
```

### 答案版填了什么

```rust
tauri::Builder::default()
    .plugin(tauri_plugin_fs::init())
    .invoke_handler(tauri::generate_handler![note_file_path, load_note, save_note])
```

### 为什么

- **插件必须先注册**：`app.fs()` 是在插件 `init` 时注册进应用状态的；不注册，运行时直接报错
- 三个命令都要登记，缺一个前端就"命令未找到"

### 回查文档

[第 2 节：为什么用 fs 插件](a02_notepad.md#sec-a02-fs)。

## App.tsx TODO 1：导入 invoke

### 练习版这里是什么

```typescript
// import { invoke } from "@tauri-apps/api/core";
```

### 答案版填了什么

```typescript
import { invoke } from "@tauri-apps/api/core";
```

### 为什么

同练习 01——`invoke` 是前端调用后端的唯一入口，不导入就是未定义变量。

## App.tsx TODO 2-3：useEffect 里加载

### 练习版这里是什么

```typescript
useEffect(() => {
  // TODO: invoke<string>("note_file_path") ...
  // TODO: invoke<string>("load_note") ...
  setFilePath("加载中…"); // 占位
  setStatus("");
}, []);
```

### 答案版填了什么

```typescript
useEffect(() => {
  invoke<string>("note_file_path")
    .then(setFilePath)
    .catch((e) => setStatus(`获取路径失败: ${e}`));

  invoke<string>("load_note")
    .then((c) => {
      setContent(c);
      setStatus("已加载");
    })
    .catch((e) => setStatus(`读取失败: ${e}`));
}, []);
```

### 为什么

- `useEffect(..., [])`：只在组件**挂载后执行一次**——正好用来做"打开应用时加载"
- `.then(setFilePath)`：把返回的路径字符串直接喂给 setter
- 两个 invoke 互不依赖，可同时发起
- 占位版里那两个 `setFilePath("加载中…")` / `setStatus("")` 是为了让练习版**编译通过**（setter 被引用），完成 TODO 后删除

### 回查文档

[第 4 节：React — useEffect](a02_notepad.md#sec-a02-react)。

## App.tsx TODO 4：保存

### 练习版这里是什么

```typescript
async function save() {
  try {
    // TODO: await invoke("save_note", { content }); ...
  } catch (e) {
    setStatus(`保存失败: ${e}`);
  }
}
```

### 答案版填了什么

```typescript
async function save() {
  try {
    await invoke("save_note", { content });
    setStatus(`已保存 ${new Date().toLocaleTimeString()}`);
  } catch (e) {
    setStatus(`保存失败: ${e}`);
  }
}
```

### 为什么

- `{ content }` 的 key 等于 Rust 参数名 `content`——**注意它和 state 变量同名**，正好直接把整个 state 传过去
- 后端返回 `Result<(), String>`：成功是 `Ok(())`（前端无需处理返回值），失败是 `Err` → Promise reject → `.catch` → 显示"保存失败: <原因>"
- 保存成功显示时间，用户有反馈

### 回查文档

[第 3 节：Result 错误处理](a02_notepad.md#sec-a02-result)。

## 验收标准

```bash
cd 02_mini_apps/a02_notepad
cargo tauri dev
```

输入文字 → 保存 → 状态行显示"已保存 HH:MM:SS"。关闭应用，再次启动 → 文字还在。窗口底部显示的路径指向真实存在的 `note.txt`（可用资源管理器打开确认）。

**破坏性验证**（确认你是理解了，而不是碰巧对）：

- 把 `save_note` 里的 `truncate: true` 删掉 → 第二次保存后，内容变成"旧内容+新内容"叠在一起（验证 truncate 的清空作用）
- 把 `create_dir_all` 那两行删掉 → 若目录不存在，保存报"创建文件失败"（验证父目录必须先建）
- 把命令名 `"load_note"` 拼错 → 启动时状态行显示"读取失败"（验证命令名一致性）
- 把 `OpenOptions` 换成 `{ read: true }` 去写 → 报"创建文件失败"（验证读写开关的作用）

## 升级挑战（可选）

- 加一个"字数统计"：保存时同时显示字数（前端 `content.length` 即可，体会"前端能算的别麻烦后端"）
- 加"自动保存"：`useEffect` 里监听 `content` 变化，300ms 防抖后自动调 `save_note`（为超级项目 P10 打基础）
