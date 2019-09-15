async function main() {
    const { memory }       = await import('./node_modules/rust-nes-emulator-wasm/rust_nes_emulator_wasm_bg');
    const { WasmEmulator, get_screen_width, get_screen_height, get_num_of_colors } = await import('./node_modules/rust-nes-emulator-wasm/rust_nes_emulator_wasm.js');
    const SCREEN_WIDTH  = get_screen_width();
    const SCREEN_HEIGHT = get_screen_height();
    const NUM_OF_COLORS = get_num_of_colors();

    const emu = new WasmEmulator();
    emu.reset();
    const buf = new Uint8Array(memory.buffer);
    const v = buf[emu.get_fb_ptr()];
    console.log(v);

}

main();