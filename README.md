# TermRaster

A memory-efficient, interactive software rasterizer built from scratch to render graphics directly in the terminal.

TermRaster is an exploration of low-level graphics programming and idiomatic Rust. Instead of relying on existing rendering engines or heavy external graphics libraries, this project implements core rasterization algorithms from the ground up. The focus is on performance, memory safety, and understanding the mathematical foundations of rendering.

## Core Philosophy

* **From Scratch:** Mathematical calculations, framebuffers, and rendering algorithms (like line drawing and polygon filling) are implemented manually to solidify an understanding of computer graphics.
* **Memory Efficiency:** Designed to minimize allocations during the render loop, utilizing flat arrays for framebuffers and aggressively reusing memory blocks.
* **Idiomatic Rust:** The codebase strictly adheres to Rust's safety guarantees and formatting standards, running with aggressive pedantic lints to ensure high-quality, professional code.

## Future Capabilities & Goals

The project is actively evolving as a learning environment for graphics engineering. Key technical features include:

* **Custom Framebuffers:** A double-buffered internal state that maps 2D coordinates to 1D arrays for fast iteration and rendering.
* **Shape Generation:** Algorithmic generation of basic geometry (lines, rectangles, circles).
* **Character Mapping:** Variable character density for rendering, allowing different ASCII characters to represent different elements, borders, or filled areas.
* **Interactive Viewport:** Real-time translation of the camera view, allowing the user to navigate the rendered space interactively.
* **Filled vs. Unfilled Geometry:** Scanline algorithms to distinguish between hollow wireframes and solid filled shapes.

## Technical Approach

TermRaster operates by constructing a virtual grid of characters in memory. During each draw, shapes are mathematically plotted onto this grid using algorithms. Once the frame calculation is complete, the entire buffer is flushed to the standard output using optimized ANSI escape sequences, ensuring smooth frame rates without visual tearing. 

By enforcing maximum compiler strictness and eliminating unnecessary dependencies, the architecture remains lightweight and highly maintainable.