// test for FFI
// g++ -o main main.cpp target/debug/rust_nes_emulator_embedded.lib

#include <stdio.h>
#include <stdint.h>

#include "rust_nes_emulator_embedded.h"

int main(void) {
    printf("hello rust. hello()=%x\n", hello());

    return 0;
}