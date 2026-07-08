# TermRaster: A Terminal Graphics Engine

> **Goal:** Build a zero-dependency, interactive terminal graphics rasterizer from scratch to study memory layouts, CPU bottlenecks, and discrete geometry.

This project is built in two phases. **Phase 1** is a deliberately naive implementation designed to prioritize getting shapes on the screen over performance. **Phase 2** uses `cargo-flamegraph` to profile the rendering loop, identify bottlenecks, and rewrite the engine using strict systems programming principles.

### The Roadmap

* [ ] Implement an interactive input loop (WASD to move, parameters to draw).
* [ ] Draw hollow and filled Circles, Squares, and Rectangles.
* [ ] Support shape overlapping and custom border/fill characters.
* [ ] Implement dynamic canvas resizing when a shape exceeds bounds.
* [ ] **Profile Phase 1:** Generate flamegraphs to document CPU and memory bottlenecks.
* [ ] **Optimize (Phase 2):** Flatten 2D vectors into 1D arrays to fix CPU cache misses.
* [ ] **Optimize (Phase 2):** Replace floating-point trigonometry with integer-math algorithms (e.g., Bresenham's).

### Why this project?

Instead of relying on heavy libraries or AI-generated boilerplate, I wanted to understand exactly what happens when you mix continuous math (`f64`) with a discrete grid (`usize`). The goal isn't just to draw shapes, but to mathematically prove *why* certain algorithms and memory structures are required for performant systems code.
