# TermRaster

[![Build Status](https://img.shields.io/github/actions/workflow/status/JeronimoCapelle/TermRaster/rust.yml?branch=main)](https://github.com/JeronimoCapelle/TermRaster/actions)
[![Crates.io](https://img.shields.io/crates/v/termraster.svg)](https://crates.io/crates/termraster)
[![License: CC0-1.0](https://img.shields.io/badge/License-CC0_1.0-lightgrey.svg)](LICENSE)

**TermRaster** is a zero-dependency software rasterizer and data visualization engine built entirely from scratch in pure Rust. 

Designed for backend engineers and sysadmins working in headless environments (like SSH sessions), TermRaster translates raw data arrays and geometric primitives into high-performance, buffered ASCII graphics directly in the terminal window.

## ⚡ Core Architecture & Systems Design

This project was built to explore the absolute limits of low-level terminal rendering, prioritizing CPU cache locality, mathematical efficiency, and strict memory safety. 

* **100% Integer-Only Pipeline:** Floating-point operations (FPUs) are slow and unnecessary for pixel grids. All geometric rendering is handled via integer-only mathematics, including a custom implementation of **Bresenham’s Line Algorithm** and the **Midpoint Circle Algorithm**, completely eliminating `f64` and `.sqrt()` overhead.
* **Data-Oriented Memory Layout:** Shapes are stored using strict Enums rather than `Box<dyn Trait>`, completely eliminating dynamic dispatch and vtable lookups. The `Canvas` utilizes a flat, 1D contiguous array (`Vec<char>`) mapping 2D coordinates to 1D indices to maximize L1/L2 CPU cache hits.
* **I/O Syscall Optimization:** Standard terminal printing causes severe bottlenecks due to repeated `write()` system calls. TermRaster employs a custom double-buffering architecture, allocating a pre-sized `String` buffer and executing exactly one I/O flush per frame to ensure zero tearing and minimal OS overhead.
* **Zero Dependencies:** The core rendering engine is built using only the Rust Standard Library (`std`). 

## 🛠️ Dual-Use Design

TermRaster operates as both a standalone Command Line Interface (CLI) for immediate data visualization, and a highly modular Rust library for custom terminal UI backends.

### 1. The CLI Plotter (In Development)
Instantly pipe data into TermRaster to generate scaled graphs without leaving your SSH session.
```bash
# Example usage:
cat server_latency.csv | termraster plot --type line --scale auto