async function main() {
  const { memory } = await import(
    "./node_modules/rust-nes-emulator-wasm/rust_nes_emulator_wasm_bg"
  );
  const {
    WasmEmulator,
    get_screen_width,
    get_screen_height,
    get_num_of_colors
  } = await import(
    "./node_modules/rust-nes-emulator-wasm/rust_nes_emulator_wasm.js"
  );
  const SCREEN_WIDTH = get_screen_width();
  const SCREEN_HEIGHT = get_screen_height();
  const NUM_OF_COLORS = get_num_of_colors();

  const emu = new WasmEmulator();
  emu.reset();
  const buf = new Uint8Array(memory.buffer);
  const v = buf[emu.get_fb_ptr()];
  console.log(v);

  ELEMENT.locale("ja", ELEMENT.lang.ja);
  const app = new Vue({
    el: "#app",
    data: {
      navbarVisible: true,
      loadRomVisible: false,
      keyconfigVisible: false,
      keyconfig: [
        { key: "A", info: "Left" },
        { key: "W", info: "Up" },
        { key: "S", info: "Down" },
        { key: "D", info: "Right" },
        { key: "J", info: "A" },
        { key: "K", info: "B" },
        { key: "U", info: "Select" },
        { key: "I", info: "Start" }
      ]
    },
    methods: {
      romSelect(e) {
        if (e.target.files.length == 0) return;
        const reader = new FileReader();
        reader.onload = file => {
            console.log(file);
            // read success dialog
            const h = this.$createElement;
            this.$notify({
              title: "Load ROM Success",
              message: h("i", { style: "color: teal" }, e.target.files[0].name),
            });
        };
        reader.readAsDataURL(e.target.files[0]);
      },
      reset() {}
    }
  });
}

main();
