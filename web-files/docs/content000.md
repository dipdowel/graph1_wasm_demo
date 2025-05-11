# Graph1
## 2D Graphics and Animation in Rust

[Graph1](https://github.com/dipdowel/graph1) is a zero-dependency library for building 2D graphics and animations in pure Rust. It’s designed to be easy to use, yet low-level enough to offer good performance even on constrained systems.<br /><br />

This site provides tutorials and examples to help you get started.

---

### ✨ Features

- Drawing primitives: lines, rectangles, circles, polygons, etc.
- Bézier curves
- Render text with [CBF](https://github.com/dipdowel/compact-bitmap-font) pixel fonts   
- Color math, gradients, and blending
- Animation support 
- CPU-only rendering — no GPU required
- Works with any native rendering library (e.g. [minifb](https://github.com/emoon/rust_minifb)) and in the browser (WASM)

---

### 🛠️ Current state
[Graph1](https://github.com/dipdowel/graph1) is already usable, though it is still in **active development**: 
- Some APIs are still evolving 
- Some features are implemented just partially
- Some bugs may still be present
- No official crate published yet on `crates.io`
  
Available via Git dependency in your: `Cargo.toml`, e.g.:
```toml
graph1 = { git = "https://github.com/dipdowel/graph1", tag = "v0.0.4-alpha" }
```

Please refer to the [ROADMAP](https://github.com/dipdowel/graph1/blob/develop/ROADMAP.md) for an overview of planned features and improvements.

---

💬 Have questions or feedback? [Start a discussion](https://github.com/dipdowel/graph1/discussions) or [open an issue](https://github.com/dipdowel/graph1/issues) on GitHub.


---


### 🕸️ WebAssembly Support

Graph1 runs well in the browser using [WebAssembly](https://webassembly.org/). All animations on this site are WASM-powered.

Prefer a TypeScript-like workflow? [AssemblyScript](https://www.assemblyscript.org/) might be worth exploring.
