async function main() {
  const { memory } = await import(
    "./node_modules/rust-nes-emulator-wasm/rust_nes_emulator_wasm_bg"
  );
  const {
    WasmEmulator,
    KeyEvent,
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
  emu.reset();
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

  // FPS制御とか
  const emulateFps = 60;
  const emulateInterval = 1000.0 / emulateFps;
  let isEmulateEnable = false;

  // Animation Frame Firedには依存せずに実行する
  function emulate_loop() {
    const start = new Date().getTime();
    if (isEmulateEnable) {
      emu.step_line();
    }
    const elapsed = ((new Date().getTime()) - start);
    const diffTime = emulateInterval - elapsed;
    // めちゃはやだったら待たせるし、間に合ってなければ即
    const sleepTime = diffTime < 0 ? 0 : diffTime;
    setTimeout(emulate_loop, sleepTime);
  }
  // Animation Frame Firedに同期してcanvasだけ書き換える
  function draw_loop() {
    if (isEmulateEnable) {
      draw();
    }
    requestAnimationFrame(draw_loop);
  }
  emulate_loop();
  draw_loop();

  function release_key(key) {
    if (isEmulateEnable) {
      switch (key) {
        case "j":
          emu.update_key(KeyEvent.ReleaseA);
          break;
        case "k":
          emu.update_key(KeyEvent.ReleaseB);
          break;
        case "u":
          emu.update_key(KeyEvent.ReleaseSelect);
          break;
        case "i":
          emu.update_key(KeyEvent.ReleaseStart);
          break;
        case "w":
          emu.update_key(KeyEvent.ReleaseUp);
          break;
        case "s":
          emu.update_key(KeyEvent.ReleaseDown);
          break;
        case "a":
          emu.update_key(KeyEvent.ReleaseLeft);
          break;
        case "d":
          emu.update_key(KeyEvent.ReleaseRight);
          break;
      }
    }
  }
  function press_key(key) {
    if (isEmulateEnable) {
      switch (key) {
        case "j":
          emu.update_key(KeyEvent.PressA);
          break;
        case "k":
          emu.update_key(KeyEvent.PressB);
          break;
        case "u":
          emu.update_key(KeyEvent.PressSelect);
          break;
        case "i":
          emu.update_key(KeyEvent.PressStart);
          break;
        case "w":
          emu.update_key(KeyEvent.PressUp);
          break;
        case "s":
          emu.update_key(KeyEvent.PressDown);
          break;
        case "a":
          emu.update_key(KeyEvent.PressLeft);
          break;
        case "d":
          emu.update_key(KeyEvent.PressRight);
          break;
      }
    }
  }

  ELEMENT.locale("ja", ELEMENT.lang.ja);
  const app = new Vue({
    el: "#app",
    data: {
      navbarVisible: true,
      loadRomVisible: false,
      keyconfigVisible: false,
      gamepadVisible: false,
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
        };
        // あとはcallbackで
        reader.readAsArrayBuffer(e.target.files[0]);
      },
      reset() {
        // emulate start時のみ
        if (isEmulateEnable) {
          isEmulateEnable = false;
          emu.reset();
          // notify
          this.$notify({
            title: "Emulator Reset"
          });
          // start
          isEmulateEnable = true;
        }
      },
      press_key(key) {
        console.log("press", key);
        press_key(key);
      },
      release_key(key) {
        console.log("release", key);
        release_key(key);
      }
    },
    mounted() {
      window.addEventListener("keyup", e => {
        release_key(e.key);
      });
      window.addEventListener("keydown", e => {
        press_key(e.key);
      });
    }
  });
}

main();
