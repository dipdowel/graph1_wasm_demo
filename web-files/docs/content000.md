# Graph1
## 2D Graphics and Animation in Rust

[Graph1](https://github.com/dipdowel/graph1) is a zero-dependency library for building 2D graphics and animations in pure Rust. It’s designed to be easy to use, yet low-level enough to offer excellent performance — even on constrained systems.

This site includes tutorials and examples to help you get started. <br />
💬 Have questions or feedback? [Start a discussion](https://github.com/dipdowel/graph1/discussions) or [open an issue](https://github.com/dipdowel/graph1/issues).

---

### ✨ Features

- Drawing primitives: lines, rectangles, circles, polygons, etc.
- Bézier curves
- Pixel-based font rendering
- Color math, gradients, and blending
- Basic animation support
- CPU-only rendering — no GPU required
- Works with native rendering libraries (e.g., with [minifb](https://github.com/emoon/rust_minifb)) and in the browser (via WebAssembly)

---

### 🚫 What Graph1 Is Not

- ❌ Not a game engine — though it’s great for making 2D games
- ❌ Not a GUI framework — but simple GUIs are possible
- ❌ Not a rendering backend
- ❌ Not a 3D engine — though 3D is possible with custom math and software rendering

---

### 🕸️ WebAssembly Support

Graph1 runs well in the browser using [WebAssembly](https://webassembly.org/). All animations on this site are WASM-powered.

Prefer a TypeScript-like workflow? [AssemblyScript](https://www.assemblyscript.org/) might be worth exploring.
