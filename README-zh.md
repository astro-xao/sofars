# Sofars

[![Build Status](https://github.com/astro-xao/sofars/actions/workflows/release-plz.yml/badge.svg)](https://github.com/astro-xao/sofars/actions)
[![Crates.io](https://img.shields.io/crates/v/sofars.svg)](https://crates.io/crates/sofars)
[![Downloads](https://img.shields.io/crates/d/sofars.svg)](https://crates.io/crates/sofars)
[![Documentation](https://docs.rs/sofars/badge.svg)](https://docs.rs/sofars)
[![Rust](https://img.shields.io/badge/rust-1.85.0%2B-blue.svg?maxAge=3600)](https://github.com/astro-xao/sofars)

[English](README.md)

`sofars` 是国际天文联合会（IAU）官方 [基础天文标准库 (SOFA)](http://iausofa.org) 的纯 Rust 实现。它提供了一套全面、高效且符合行业标准的天文计算工具，适用于高精度的时间尺度转换、坐标系统转换及基础天文模型计算。

## 核心特性

- **标准符合性**: 严格遵循 IAU 2000/2006 标准模型，确保计算结果与 SOFA C 库高度一致。
- **纯 Rust 实现**: 无需 C 编译器环境，享受 Rust 的内存安全、并发特性与卓越性能。
- **功能全面**:
  - **时间尺度 (`ts`)**: 支持 UTC, TAI, TT, TDB, TCG, TCB 等多种时间尺度的精确转换。
  - **岁差、章动与极移 (`pnp`)**: 提供完整的 IAU 岁差与章动模型。
  - **基础天体测量 (`astro`)**: 包括恒星视位置、折射修正等核心算法。
  - **坐标系统 (`coords`)**: 支持地心、日心、球面等多种坐标系转换。
  - **向量与矩阵运算 (`vm`)**: 针对天文计算优化的 3D 向量与旋转矩阵工具。
  - **历法工具 (`cal`)**: 儒略日 (JD) 与公历日期的相互转换。

## 安装

在你的 `Cargo.toml` 中添加以下依赖：

```toml
[dependencies]
sofars = "0.6.0"
```

## 快速上手

以下示例展示了如何计算给定儒略日的地球旋转角 (ERA)：

```rust
use sofars::consts::DJ00;
use sofars::ts::era00;

fn main() {
    // 2000年1月1日 12:00 (TT)
    let tt1 = DJ00;
    let tt2 = 0.5;

    let era = era00(tt1, tt2);
    println!("地球旋转角: {} 弧度", era);
}
```

## 文档

详细的 API 说明与函数列表请访问 [docs.rs/sofars](https://docs.rs/sofars)。

## 许可协议与 SOFA 使用条款

本项目遵循 **MIT** 许可协议。

**重要说明**：由于本库的核心算法源自 IAU SOFA 源代码，任何使用本项目的行为还应遵守 SOFA 的许可条款。
- 任何包含使用 `sofars` 获得的结果的发表作品或商业产品，应当承认使用了由 **IAU SOFA Board** 提供的算法。
- 详情请参阅项目根目录下的 `LICENSE` 文件。
