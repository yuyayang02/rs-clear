<div align="center">

# rs-clear

[English](./README.md) | 中文

</div>

---

## ✨ 功能特性

* 递归扫描指定目录中的 `Cargo.toml`
* 找到同级目录下的 `target/` 目标文件夹
* 使用命令行互动选择要删除的 target/
* 可选的 `--all` 参数，不通过互动直接删除所有 target/

---

## 📂 安装

### 源码编译

```sh
cargo install --path .
```

---

## 🚀 使用方法

```sh
rs-clear [路径] [选项]
```

### 参数

* `PATH` (可选)：扫描的根目录，默认为当前目录 `.`

### 选项

* `-a`, `--all` - 直接删除所有 target/ 文件夹，不再进行选择
* `-h`, `--help` - 查看帮助
* `-V`, `--version` - 查看版本

### 例子

```sh
# 互动选择当前目录下要删除的 target/
rs-clear

# 不互动：直接删除 ./workspace 下的所有 target/
rs-clear ./workspace --all
```

---

## 🧰 输出示例

```
scanning directory: ./workspace
select target directories to delete:
[deleted] ./workspace/crate-a/target
[deleted] ./workspace/crate-b/target
[failed]  ./workspace/crate-c/target: permission denied
deleting: ./workspace/crate-c/target
done
```

---

## ⚒️ 依赖

* [`clap`](https://crates.io/crates/clap) - 命令行参数解析
* [`inquire`](https://crates.io/crates/inquire) - 互动选择 UI
* [`walkdir`](https://crates.io/crates/walkdir) - 递归目录扫描
* [`clap-cargo`](https://crates.io/crates/clap-cargo) - 风格类似 cargo 的 CLI 设计

---

## 📍 反馈 & 帮助

本项目由 AI 协助完成。如有意见或建议，欢迎提交 [Issue](https://github.com/yuyayang02/rs-clear/issues) 或 PR 关注优化。
