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

## 填空任务（src/main.ts）

1. Database.load 加载数据库
2. CREATE TABLE 建表
3. INSERT 绑定参数（$1/$2）
4. select 查询并渲染列表
5. 事务提交（BEGIN/INSERT/COMMIT）
6. 事务回滚（BEGIN/INSERT/ROLLBACK）

对照答案: `../e33_sql_plugin_answer/`