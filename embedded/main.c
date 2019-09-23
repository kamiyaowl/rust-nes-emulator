#include <stdio.h>
#include <stdint.h>

uint32_t hello();

int main(void) {
    printf("hello rust. hello()=%x\n", hello());

    return 0;
}