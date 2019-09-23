#include "mbed.h"

// for peripheral
#include "stm32f769i_discovery.h"
#include "stm32f769i_discovery_lcd.h"
#include "stm32f769i_discovery_sdram.h"
#include "stm32f769i_discovery_ts.h"
#include "stm32f769i_discovery_sd.h"
#include "stm32f769i_discovery_audio.h"

// for system
#include "stm32f7xx_hal_flash.h"
#include "core_cm7.h"

// for emulator
#include "rust_nes_emulator_embedded.h"


// typedef uint8_t (*FrameBuffer_t)[EMBEDDED_EMULATOR_VISIBLE_SCREEN_HEIGHT][EMBEDDED_EMULATOR_VISIBLE_SCREEN_WIDTH][EMBEDDED_EMULATOR_NUM_OF_COLOR];
uint8_t fb[EMBEDDED_EMULATOR_VISIBLE_SCREEN_HEIGHT][EMBEDDED_EMULATOR_VISIBLE_SCREEN_WIDTH][EMBEDDED_EMULATOR_NUM_OF_COLOR];

// TODO: DMAにしたい
void print_framebuffer(uint32_t offset_x, uint32_t offset_y, uint32_t scale) {
    for(uint32_t j = 0 ; j < EMBEDDED_EMULATOR_VISIBLE_SCREEN_HEIGHT ; ++j) {
        for(uint32_t i = 0 ; i < EMBEDDED_EMULATOR_VISIBLE_SCREEN_WIDTH ; ++i) {
            const uint32_t argb = (0xff << 24) | (fb[j][i][0] << 16) | (fb[j][i][1] << 8) | (fb[j][i][2] << 0);

            for (uint32_t iter = 0 ; iter < scale ; ++iter) {
                const uint32_t x = offset_x + (i * scale) + iter;
                const uint32_t y = offset_y + (j * scale) + iter;
                BSP_LCD_DrawPixel(x, y, argb);
            }
        }
    }
}

int main()
{
     // for performance
    __HAL_FLASH_ART_ENABLE();
    __HAL_FLASH_PREFETCH_BUFFER_ENABLE();
    SCB_EnableDCache();
    SCB_EnableICache();
    __DMB();
    
   printf("\n\n SDRAM EXAMPLE FOR DISCO-F769NI START:\n");

    /* Init LCD and display example information */
    BSP_LCD_Init();
    BSP_LCD_LayerDefaultInit(0, LCD_FB_START_ADDRESS);
    BSP_LCD_Clear(LCD_COLOR_WHITE);
    BSP_LCD_SetTextColor(LCD_COLOR_BLACK);
    BSP_LCD_FillRect(0, 0, BSP_LCD_GetXSize(), 40);
    BSP_LCD_SetTextColor(LCD_COLOR_WHITE);
    BSP_LCD_SetBackColor(LCD_COLOR_BLACK);
    BSP_LCD_SetFont(&Font24);
    BSP_LCD_DisplayStringAt(0, 5, (uint8_t *)"kamiyaowl/rust-nes-emulator", CENTER_MODE);
    BSP_SD_Init();

    /* SDRAM device configuration */
    if(BSP_SDRAM_Init() != SDRAM_OK) {
        BSP_LCD_DisplayStringAt(20, 100, (uint8_t *)"SDRAM Initialization : FAILED", LEFT_MODE);
    } else {
        BSP_LCD_DisplayStringAt(20, 100, (uint8_t *)"SDRAM Initialization : OK", LEFT_MODE);
    }

     /* Touchscreen initialization */
    if (BSP_TS_Init(BSP_LCD_GetXSize(), BSP_LCD_GetYSize()) == TS_ERROR) {
        BSP_LCD_DisplayStringAt(20, 130, (uint8_t *)"Touchscreen Initialization : FAILED", LEFT_MODE);
    } else {
        BSP_LCD_DisplayStringAt(20, 130, (uint8_t *)"Touchscreen Initialization : OK", LEFT_MODE);
    }   

    /* Emulator Test */
    EmbeddedEmulator_init();
    if (EmbeddedEmulator_load()) {
        BSP_LCD_DisplayStringAt(20, 160, (uint8_t *)"Emulator ROM Load  : OK", LEFT_MODE);
    } else {
        BSP_LCD_DisplayStringAt(20, 160, (uint8_t *)"Emulator ROM Load  : FAILED", LEFT_MODE);
    }

    char msg[32];
    sprintf(msg, "Core Clock: %d Hz", SystemCoreClock );
    BSP_LCD_DisplayStringAt(20, 190, (uint8_t *)msg, LEFT_MODE);

    wait_ms(3000);
    BSP_LCD_Clear(LCD_COLOR_BLACK);

    for (uint32_t counter = 0 ; ; ++counter ) {
        EmbeddedEmulator_update_screen(&fb);
        print_framebuffer(150, 10, 2);

        sprintf(msg, "%d", counter);
        BSP_LCD_DisplayStringAt(5, 5, (uint8_t *)msg, LEFT_MODE);

        /* Touchscreen test */
        // TS_StateTypeDef  TS_State = {0};
        // BSP_TS_GetState(&TS_State);
        // if(TS_State.touchDetected) {
        //     const uint16_t x1 = TS_State.touchX[0];
        //     const uint16_t y1 = TS_State.touchY[0];
        //     BSP_LCD_SetTextColor(LCD_COLOR_BLUE);
        //     BSP_LCD_FillCircle(x1, y1, 10);
        // }
    }
}