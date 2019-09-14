const wasm = import('./node_modules/rust-nes-emulator-wasm/rust_nes_emulator_wasm.js');
wasm.then(wasm => {
  wasm.greet('WebAssembly');
});