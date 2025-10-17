# puniyu-plugin

## 介绍

`puniyu-plugin` 是一个用于开发 Puniyu 平台插件的 Rust crate。它提供了必要的工具和宏来简化插件开发过程，让开发者能够专注于插件功能的实现。

## 功能特性

- 🚀 **简单易用** - 提供直观的 API 和宏来快速开发插件
- ⚡ **高性能** - 基于 Rust 语言构建，确保插件运行效率
- 🎯 **类型安全** - 利用 Rust 的类型系统避免运行时错误
- 🔌 **可扩展** - 支持任务调度、命令处理等多种插件类型

## 快速开始

### 安装

在你的 `Cargo.toml` 中添加依赖：

```toml
[dependencies]
puniyu-plugin = "0.1"
```

### 基本用法

```rust
use puniyu_plugin::prelude::*;

#[plugin]
async fn my_plugin() {
	// 插件初始化逻辑
	println!("Hello, Puniyu!");
}
```

## 文档

更多详细信息请参考 [API 文档](https://docs.rs/puniyu-plugin)。

## 许可证

本项目采用 [LGPL](../LICENSE) 许可证。