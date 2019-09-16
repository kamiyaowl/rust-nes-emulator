# rust-nes-emulator

[![CircleCI](https://circleci.com/gh/kamiyaowl/rust-nes-emulator.svg?style=svg&circle-token=5863f12e124bd230551101e146947e7a49e5765f)](https://circleci.com/gh/kamiyaowl/rust-nes-emulator)
[![Netlify Status](https://api.netlify.com/api/v1/badges/7ae6eff9-8c7a-416a-bad7-3e78b00ad95b/deploy-status)](https://app.netlify.com/sites/rust-nes-emulator/deploys)


NES Emulator written in Rust

| Super Mario Bros. | Donkey Kong |
| - | - |
| ![mario-gif](https://user-images.githubusercontent.com/4300987/64512802-1bc8bd00-d322-11e9-8a70-26df62bb5ee1.gif) | ![donkey-gif](https://user-images.githubusercontent.com/4300987/64512801-1bc8bd00-d322-11e9-9e6c-0a149fb05c1b.gif) |

| Mario Bros. | - |
| - | - |
| ![mariobros-gif](https://user-images.githubusercontent.com/4300987/64917495-06dda500-d7cc-11e9-9037-f5f8bd7de061.gif) | - |

---


## Let's Play

rustc 1.37.0 required

### Desktop Application

```shell
$ cd desktop
$ cargo run --release
```

#### on Docker

```shell
$ docker-compose run build-desktop-release
```

### WebAssembly Application

[(Work In Progress...) rust-nes-emulator.netlify.com](https://rust-nes-emulator.netlify.com/)


```shell
$ cd wasm
$ wasm-pack build --release
$ npm install
$ npm run build
```

#### on Docker

```shell
$ docker-compose run build-wasm-release
$ docker-compose run build-wasm-webpage
```

---

## Screenshot (Operation checked)

### Super Mario Bros.

![mario](https://raw.githubusercontent.com/kamiyaowl/rust-nes-emulator/master/screenshot/mario.bmp)


### Donkey Kong

![donkey](https://raw.githubusercontent.com/kamiyaowl/rust-nes-emulator/master/screenshot/donkey.bmp)

### Mario Bros.

![mariobros](https://raw.githubusercontent.com/kamiyaowl/rust-nes-emulator/master/screenshot/mariobros.bmp)

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
    - [ ] Snapshot
    - [ ] Restore
    - [ ] ROM Selection Bootloader
    
## Test ROMs

Thank you for the wonderful !

| path | from | url |
| ---- | ---- | --- |
| roms/other/hello.nes | コンソールゲーム機研究所 | http://hp.vector.co.jp/authors/VA042397/nes/sample.html |
| roms/nes-test-roms | christopherpow/nes-test-roms - GitHub | https://github.com/christopherpow/nes-test-roms |
