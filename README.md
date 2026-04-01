# Scaffold

> 这是一个个人使用的项目脚手架工具，主要通过 Vibe coding 方式实现。它可以帮助你快速初始化不同编程语言的项目结构和常用配置文件。

## 功能特性

目前支持以下两种语言的快速初始化：

- 🦀 **Rust**
  - 自动生成 `rust-toolchain.toml`
  - 自动生成 `rustfmt.toml`
  - 自动生成包含常用命令的 `justfile`

- 🐍 **Python**
  - 自动调用 `uv init` 初始化项目（如果 `pyproject.toml` 不存在）
  - 自动安装开发依赖：`ruff` 和 `ty`
  - 自动生成包含常用命令的 `justfile`

## 依赖要求

在使用本脚手架生成的环境时，建议（或必须）安装以下工具：

- [Just](https://github.com/casey/just) - 用于运行 `justfile` 中定义的任务。
- [uv](https://github.com/astral-sh/uv) - Python 项目初始化强依赖此工具。

## 安装

作为一个 Rust 编写的命令行工具，你可以通过 `cargo` 直接从源码本地安装：

```bash
git clone <repository-url>
cd scaffold
cargo install --path .
```

## 使用方法

命令行基本语法为：

```bash
scaffold init <LANGUAGE> [PATH]
```

- `<LANGUAGE>`: 必填，支持 `rust` 或 `python`。
- `[PATH]`: 选填，项目目录路径，默认为当前目录 (`.`)。

### 示例

**初始化 Rust 项目：**

```bash
# 在当前目录初始化
scaffold init rust

# 在指定目录初始化
scaffold init rust ./my-rust-app
```

**初始化 Python 项目：**

```bash
# 在当前目录初始化
scaffold init python

# 在指定目录初始化
scaffold init python ./my-python-app
```

## 技术栈

本项目本身使用 Rust 开发，核心依赖包括：
- `clap` - 强大的命令行参数解析器
- `colored` - 终端文本着色
