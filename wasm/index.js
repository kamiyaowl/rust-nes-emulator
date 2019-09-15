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

    ELEMENT.locale("ja", ELEMENT.lang.ja);
    const app = new Vue({
        el: '#app',
        data: {
            message: "asdf",
            keyconfigVisible: false,
            keyconfig: [
                { key: "A", description: "Cross-Key Left" },
                { key: "W", description: "Cross-Key Up" },
                { key: "S", description: "Cross-Key Down" },
                { key: "D", description: "Cross-Key Right" },
                { key: "J", description: "Button A" },
                { key: "K", description: "Button B" },
                { key: "U", description: "Button Select" },
                { key: "I", description: "Button Start" },
                { key: "R", description: "[emulator] Reset" },
                { key: "O", description: "[emulator] Select ROM" },
            ],
        },
        methods: {
            load() {},
            reset() {},
        },
    });
}

main();