// test for FFI
// g++ -o main main.cpp target/debug/rust_nes_emulator_embedded.so

#include <iostream>
#include <cstdint>

#include "rust_nes_emulator_embedded.h"

uint8_t fb[EMBEDDED_EMULATOR_VISIBLE_SCREEN_HEIGHT][EMBEDDED_EMULATOR_VISIBLE_SCREEN_WIDTH][EMBEDDED_EMULATOR_NUM_OF_COLOR];

void print_framebuffer() {
    std::cout << "#print_framebuffer()" << std::endl;

    for(uint32_t j = 0 ; j < EMBEDDED_EMULATOR_VISIBLE_SCREEN_HEIGHT ; ++j) {
        for(uint32_t i = 0 ; i < EMBEDDED_EMULATOR_VISIBLE_SCREEN_WIDTH ; ++i) {
            if ((fb[j][i][0] == 0) && (fb[j][i][0] == 0) && (fb[j][i][0] == 0)) {
                std::cout << ".";
            } else {
                std::cout << "#";
            }
        }
        std::cout << std::endl;
    }
}
int main(void) {
    // Emulator initialize
    EmbeddedEmulator_init();
    if (EmbeddedEmulator_load() == false) {
        std::cout << "emulator load error" << std::endl;
        return 1;
    }

    // Emulator run
    EmbeddedEmulator_update_screen(&fb);
    print_framebuffer();

    return 0;
}