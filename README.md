[![CircleCI](https://circleci.com/gh/kamiyaowl/rust-nes-emulator.svg?style=svg)](https://circleci.com/gh/kamiyaowl/rust-nes-emulator)

# rust-nes-emulator

NES Emulator written in Rust

| Super Mario Bros. | Donkey Kong |
| - | - |
| ![mario-gif](https://user-images.githubusercontent.com/4300987/64512802-1bc8bd00-d322-11e9-8a70-26df62bb5ee1.gif) | ![donkey-gif](https://user-images.githubusercontent.com/4300987/64512801-1bc8bd00-d322-11e9-9e6c-0a149fb05c1b.gif) |

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
  - [ ] Puluse Wave1
  - [ ] Puluse Wave2
  - [ ] Tri Wave
  - [ ] Noise
  - [ ] DMC
- [ ] Emulation feature
    - [ ] Snapshot
    - [ ] Restore
    - [ ] ROM Selection Bootloader

## Screenshot (Operation checked)

### Super Mario Bros.

![mario](https://raw.githubusercontent.com/kamiyaowl/rust-nes-emulator/master/screenshot/mario.bmp)


### Donkey Kong

![donkey](https://raw.githubusercontent.com/kamiyaowl/rust-nes-emulator/master/screenshot/donkey.bmp)

### nestest (unofficial opcode)

![nestest_extra](https://raw.githubusercontent.com/kamiyaowl/rust-nes-emulator/master/screenshot/nestest_extra.bmp)

### nestest (official opcode)

![nestest_normal](https://raw.githubusercontent.com/kamiyaowl/rust-nes-emulator/master/screenshot/nestest_normal.bmp)

### Hello World

![hello](https://raw.githubusercontent.com/kamiyaowl/rust-nes-emulator/master/screenshot/hello.bmp)

---

## Build & Run

```
$ cargo run --release
```

rustc 1.37.0 required

## Build (on Docker)

```
$ docker-compose run build-release
$ ./target/release/rust-nes-emulator
```

## Test ROMs

Thank you for the wonderful !

| path | from | url |
| ---- | ---- | --- |
| roms/other/hello.nes | コンソールゲーム機研究所 | http://hp.vector.co.jp/authors/VA042397/nes/sample.html |
| roms/nes-test-roms | christopherpow/nes-test-roms - GitHub | https://github.com/christopherpow/nes-test-roms |
