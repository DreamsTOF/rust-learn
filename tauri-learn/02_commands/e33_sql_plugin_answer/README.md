# 练习 E33: SQL（sql_plugin）

## 知识点

- @tauri-apps/plugin-sql：Database.load("sqlite:notes.db")
- execute（写/DDL，支持 $1/$2 绑定参数）与 select（返回对象数组）
- 事务：BEGIN → INSERT → COMMIT / ROLLBACK
- capabilities：sql:default 不含 execute 权限，需补 sql:allow-execute

## 运行

```bash
pnpm install
cargo tauri dev
```

## 操作验证

1. 添加笔记（绑定参数）→ 列表自动刷新
2. 事务提交：列表出现「事务-已提交」
3. 事务回滚：列表不出现「事务-已回滚」
4. 数据库文件位于 app_config_dir（notes.db）

对照练习版: `../e33_sql_plugin/`