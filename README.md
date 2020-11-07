# rust-nes-emulator

[![Test](https://github.com/kamiyaowl/rust-nes-emulator/workflows/Test/badge.svg)](https://github.com/kamiyaowl/rust-nes-emulator/actions?query=workflow%3ATest)
[![Deploy](https://github.com/kamiyaowl/rust-nes-emulator/workflows/Deploy/badge.svg)](https://github.com/kamiyaowl/rust-nes-emulator/actions?query=workflow%3ADeploy)
[![Build for Windows](https://github.com/kamiyaowl/rust-nes-emulator/workflows/Build%20for%20Windows/badge.svg)](https://github.com/kamiyaowl/rust-nes-emulator/actions?query=workflow%3A%22Build+for+Windows%22)

NES Emulator written in Rust

| Super Mario Bros. | Donkey Kong | Mario Bros. |
| - | - | - |
| ![mario-gif](https://user-images.githubusercontent.com/4300987/64512802-1bc8bd00-d322-11e9-8a70-26df62bb5ee1.gif) | ![donkey-gif](https://user-images.githubusercontent.com/4300987/64512801-1bc8bd00-d322-11e9-9e6c-0a149fb05c1b.gif) | ![mariobros-gif](https://user-images.githubusercontent.com/4300987/64917495-06dda500-d7cc-11e9-9037-f5f8bd7de061.gif) |


---


## Let's Play

rustc 1.39.0-nightly required (for embedded optimization...)

### Desktop Application

High Performance.

![desktop](https://github.com/kamiyaowl/rust-nes-emulator/blob/master/screenshot/desktop.PNG?raw=true)

```shell
$ cd desktop
$ cargo run --release
```

#### Build on Docker

```shell
$ docker-compose run build-desktop-release
```

---

### WebAssembly Application

![wasm](https://raw.githubusercontent.com/kamiyaowl/rust-nes-emulator/master/screenshot/wasm.PNG?raw=true)

[Playgound: http://kamiya.tech/rust-nes-emulator/](http://kamiya.tech/rust-nes-emulator/index.html)

#### Build locally

##### environment(for ubuntu)

```shell
$ sudo apt install nodejs npm
$ sudo npm install -g n
$ sudo n 10.15.1
$ cargo install wasm-pack
```

##### build

```shell
$ cd wasm
$ wasm-pack build --release
$ npm install
$ npm run build
```

#### Build on Docker

```shell
$ docker-compose run build-wasm-release
$ docker-compose run build-wasm-webpage
```


---

### Embedded for stm32f769

![embedded](https://raw.githubusercontent.com/kamiyaowl/rust-nes-emulator/master/screenshot/embedded.jpg)

[STM32F769I-DISCO - STMicroelectronics](https://www.st.com/ja/evaluation-tools/32f769idiscovery.html)

Work in Progress...

The following are derived repositories that are analyzing and implementing improvements to the performance issues.

[kamiyaowl/rust-nes-emulator-embedded - GitHub](https://github.com/kamiyaowl/rust-nes-emulator-embedded)

#### Build locally

##### environment(for ubuntu)

```shell
$ rustup install nightly
$ rustup run nightly rustup target add thumbv6m-none-eabi thumbv7m-none-eabi thumbv7em-none-eabi thumbv7em-none-eabihf
$ sudo apt install gcc-arm-none-eabi gcc g++
```

##### build

```shell
$ cd embedded
$ rustup run nightly cargo build --release
$ make clean && make
```

#### Build on Docker

```shell
$ docker-compose run build-embedded-lib
$ docker-compose run build-mbed
```

---

## Build Artifacts

see [Github Actions#Deploy](https://github.com/kamiyaowl/rust-nes-emulator/actions?query=workflow%3ADeploy).

---

## Screenshot (Operation checked)

### Super Mario Bros.

![mario](https://raw.githubusercontent.com/kamiyaowl/rust-nes-emulator/master/screenshot/mario.bmp)

### Donkey Kong

![donkey](https://raw.githubusercontent.com/kamiyaowl/rust-nes-emulator/master/screenshot/donkey.bmp)

### Mario Bros.

![mariobros](https://raw.githubusercontent.com/kamiyaowl/rust-nes-emulator/master/screenshot/mariobros.bmp)

### Ice Climber.

![iceclimber](https://raw.githubusercontent.com/kamiyaowl/rust-nes-emulator/master/screenshot/iceclimber.bmp)

### nestest (unofficial opcode)

![nestest_extra](https://raw.githubusercontent.com/kamiyaowl/rust-nes-emulator/master/screenshot/nestest_extra.bmp)

### nestest (official opcode)

![nestest_normal](https://raw.githubusercontent.com/kamiyaowl/rust-nes-emulator/master/screenshot/nestest_normal.bmp)

### Hello World

![hello](https://raw.githubusercontent.com/kamiyaowl/rust-nes-emulator/master/screenshot/hello.bmp)

---

## Feature & Known Issue

- [x] CPU
  - [x] Register
  - [x] Interrupt
  - [x] Official opcode
  - [x] Unofficial opcode
- [x] Cassette(Mapper)
  - [x] NROM(Mapper0)
  - [ ] UNROM
  - [ ] MMC1
  - [ ] MMC3
- [x] PPU
  - [x] OAM DMA
  - [x] BG
    - [x] Nametable Mirroring
    - [x] Scroll
      - [ ] Vertical Scroll Bug(#87)
  - [x] Sprite
    - [x] 8*8
    - [x] 8*16
    - [ ] Sprite 0 hit bug(#40)
- [x] PAD
  - [x] Joypad1
- [ ] APU
  - [ ] Pulse Wave1
  - [ ] Pulse Wave2
  - [ ] Tri Wave
  - [ ] Noise
  - [ ] DMC
- [ ] Emulation feature
    - [x] Snapshot
    - [x] Restore
    - [ ] ROM Selection Bootloader
    
## Test ROMs

Thank you for the wonderful !

| path | from | url |
| ---- | ---- | --- |
| roms/other/hello.nes | コンソールゲーム機研究所 | http://hp.vector.co.jp/authors/VA042397/nes/sample.html |
| roms/nes-test-roms | christopherpow/nes-test-roms - GitHub | https://github.com/christopherpow/nes-test-roms |
