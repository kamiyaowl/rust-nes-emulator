const js = import("./pkg/rust_nes_emulator_wasm.js");
js.then(js => {
  js.greet("Hello Rust wasm!");
});