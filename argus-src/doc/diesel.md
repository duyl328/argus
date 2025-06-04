
# 🚀 Diesel 常用命令与操作速查文档（适用于 SQLite）

## 🧱 一、准备阶段

### 1. 安装 Diesel CLI

```bash
cargo install diesel_cli --no-default-features --features sqlite
```

📌 **作用**：安装 Diesel 的命令行工具，后续的 `diesel` 命令依赖它。
💡 使用 `--features sqlite` 指定支持的数据库类型（也支持 postgres、mysql）。

---

### 2. 创建 `.env` 文件

```env
DATABASE_URL=sqlite://db.sqlite
```

📌 **作用**：告诉 Diesel 数据库连接位置。Diesel 会自动读取 `.env` 里的 `DATABASE_URL`。
💡 `db.sqlite` 是数据库文件路径（你也可以放到其他位置）。

---

## 🛠 二、数据库初始化相关命令

### 3. 初始化数据库 + 创建迁移目录

```bash
diesel setup
```

📌 **作用**：完成以下操作：

* 创建数据库文件（如果是 SQLite）
* 创建 `migrations/` 目录（用于放置数据库变更脚本）

---

## 🧩 三、数据库迁移操作

### 4. 创建一个新的迁移脚本

```bash
diesel migration generate create_users
```

📌 **作用**：生成一组迁移文件，用于定义新的数据库结构变更。

✅ 会生成：

```
migrations/
  └── 2025-06-04-xxxxxx_create_users/
        ├── up.sql    # 写数据库创建语句
        └── down.sql  # 写数据库回退语句
```

---

### 5. 执行所有未运行的迁移（更新数据库结构）

```bash
diesel migration run
```

📌 **作用**：执行所有 `up.sql`，把数据库结构升级到最新版本。

---

### 6. 回滚上一次的迁移

```bash
diesel migration revert
```

📌 **作用**：撤销上一次执行的迁移（执行 `down.sql`）。

---

### 7. 回滚到初始状态（慎用）

```bash
diesel migration redo
```

📌 **作用**：撤销并重新执行上一个迁移，相当于：`revert` ➜ `run`。

---

## 🧾 四、模型生成与数据交互（Rust 代码部分）

### 8. 根据数据库表结构生成 Rust 模型（schema.rs）

```bash
diesel print-schema > src/schema.rs

# 目前项目在用
diesel print-schema > src/storage/schema.rs
```

📌 **作用**：读取数据库结构，生成对应的 Rust 表定义代码。
💡 你可以在项目里使用这些定义进行查询。

---

## 🧪 五、调试 & 检查状态

### 9. 查看当前数据库迁移状态

```bash
diesel migration list
```

📌 **作用**：显示所有迁移的执行状态（哪些已运行、哪些没运行）。

---

# 🗂 示例工作流（SQLite）

```bash
# 安装 CLI（仅第一次）
cargo install diesel_cli --no-default-features --features sqlite

# 设置数据库地址
echo DATABASE_URL=sqlite://db.sqlite > .env

# 初始化项目
diesel setup

# 创建一条迁移记录
diesel migration generate create_users

# 编辑 up.sql 和 down.sql，定义数据库变更

# 应用迁移
diesel migration run

# 如果写错了，回滚
diesel migration revert

# 查看迁移状态
diesel migration list

# 自动生成 Rust 表结构定义
diesel print-schema > src/schema.rs
```

---

# 📎 常见文件结构

```
project/
├── .env                    # 数据库连接字符串
├── migrations/             # 所有迁移记录
│   └── <timestamp>_<name>/
│       ├── up.sql
│       └── down.sql
├── src/
│   ├── main.rs
│   └── schema.rs           # 自动生成的表结构
```

---

