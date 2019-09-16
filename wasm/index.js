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
  const NUM_OF_COLORS = get_num_of_colors(); // rust上での扱い、imageDataはalphaもある
  const emu = new WasmEmulator();
  const rustBuf = new Uint8Array(memory.buffer);
  const fbBasePtr = emu.get_fb_ptr();

  function draw() {
    const canvas = document.getElementById("fb");
    const ctx = canvas.getContext("2d");
    const imageData = ctx.getImageData(0, 0, SCREEN_WIDTH, SCREEN_HEIGHT);
    for (let j = 0; j < SCREEN_HEIGHT; j++) {
      for (let i = 0; i < SCREEN_WIDTH; i++) {
        const imageDataPtr = j * (SCREEN_WIDTH * 4) + i * 4;
        const rustDataPtr =
          fbBasePtr + j * (SCREEN_WIDTH * NUM_OF_COLORS) + i * NUM_OF_COLORS;
        imageData.data[imageDataPtr + 0] = rustBuf[rustDataPtr + 0]; // red
        imageData.data[imageDataPtr + 1] = rustBuf[rustDataPtr + 1]; // green
        imageData.data[imageDataPtr + 2] = rustBuf[rustDataPtr + 2]; // blue
        imageData.data[imageDataPtr + 3] = 255; //alpha
      }
    }
    ctx.putImageData(imageData, 0, 0);
  }

  emu.reset();
  draw();
  let isEmulateEnable = false;

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
          const arrayBuf = file.target.result;
          const src = new Uint8Array(arrayBuf);
          // stop emulate
          isEmulateEnable = false;
          // cassette load
          if (!emu.load(src)) {
            // error notify
            const h = this.$createElement;
            this.$notify({
              title: "Load ROM Error"
            });
            return;
          }
          // read success notify
          const h = this.$createElement;
          this.$notify({
            title: "Load ROM Success",
            message: h("i", { style: "color: teal" }, e.target.files[0].name)
          });
          // start emulate
          emu.reset();
          isEmulateEnable = true;
          // test
          emu.step_line();
          emu.step_line();
          emu.step_line();
          emu.step_line();
          emu.step_line();
          emu.step_line();
          draw();
        };
        // あとはcallbackで
        reader.readAsArrayBuffer(e.target.files[0]);
      },
      reset() {
        isEmulateEnable = false;
        emu.reset();
        // notify
        const h = this.$createElement;
        this.$notify({
          title: "Emulator Reset"
        });
        // start
        isEmulateEnable = true;
      }
    }
  });
}

main();
