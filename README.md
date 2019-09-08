[![CircleCI](https://circleci.com/gh/kamiyaowl/rust-nes-emulator.svg?style=svg)](https://circleci.com/gh/kamiyaowl/rust-nes-emulator)

# rust-nes-emulator

NES Emulator written by Rust

## Screenshot

### Super Mario Bros.

Work In Progress...

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
$ ./target/release/rust-nes-emulator     # Other Platform
```

## Test ROMs

Thank you for the wonderful !

| path | from | url |
| ---- | ---- | --- |
| roms/other/hello.nes | コンソールゲーム機研究所 | http://hp.vector.co.jp/authors/VA042397/nes/sample.html |
| roms/nes-test-roms | christopherpow/nes-test-roms - GitHub | https://github.com/christopherpow/nes-test-roms |
