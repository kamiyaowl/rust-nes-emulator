#include "mbed.h"
#include "stm32f769i_discovery.h"
#include "stm32f769i_discovery_lcd.h"
#include "stm32f769i_discovery_sdram.h"

#include "rust_nes_emulator_embedded.h"

static void Fill_Buffer(uint32_t *pBuffer, uint32_t uwBufferLenght, uint32_t uwOffset);
static uint8_t Buffercmp(uint32_t* pBuffer1, uint32_t* pBuffer2, uint16_t BufferLength);

#define BUFFER_SIZE            ((uint32_t)0x0100)
#define WRITE_READ_ADDR        ((uint32_t)0x0800)
#define SDRAM_WRITE_READ_ADDR  ((uint32_t)0xC0177000)

uint32_t sdram_aTxBuffer[BUFFER_SIZE];
uint32_t sdram_aRxBuffer[BUFFER_SIZE];

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
    printf("\n\n SDRAM EXAMPLE FOR DISCO-F769NI START:\n");

    /* Init LCD and display example information */
    BSP_LCD_Init();
    BSP_LCD_LayerDefaultInit(0, LCD_FB_START_ADDRESS);
    BSP_LCD_Clear(LCD_COLOR_WHITE);
    BSP_LCD_SetTextColor(LCD_COLOR_BLUE);
    BSP_LCD_FillRect(0, 0, BSP_LCD_GetXSize(), 40);
    BSP_LCD_SetTextColor(LCD_COLOR_WHITE);
    BSP_LCD_SetBackColor(LCD_COLOR_BLUE);
    BSP_LCD_SetFont(&Font24);
    BSP_LCD_DisplayStringAt(0, 0, (uint8_t *)"kamiyaowl/mbed_DISCO-F769NI - GitHub", CENTER_MODE);

    /* SDRAM device configuration */
    if(BSP_SDRAM_Init() != SDRAM_OK) {
        BSP_LCD_DisplayStringAt(20, 100, (uint8_t *)"SDRAM Initialization : FAILED", LEFT_MODE);
    } else {
        BSP_LCD_DisplayStringAt(20, 100, (uint8_t *)"SDRAM Initialization : OK", LEFT_MODE);
    }
    /* Fill the buffer to write */
    Fill_Buffer(sdram_aTxBuffer, BUFFER_SIZE, 0xA244250F);

    /* Write data to the SDRAM memory */
    if(BSP_SDRAM_WriteData(SDRAM_WRITE_READ_ADDR + WRITE_READ_ADDR, sdram_aTxBuffer, BUFFER_SIZE) != SDRAM_OK) {
        BSP_LCD_DisplayStringAt(20, 130, (uint8_t *)"SDRAM WRITE : FAILED", LEFT_MODE);
    } else {
        BSP_LCD_DisplayStringAt(20, 130, (uint8_t *)"SDRAM WRITE : OK", LEFT_MODE);
    }

    /* Read back data from the SDRAM memory */
    if(BSP_SDRAM_ReadData(SDRAM_WRITE_READ_ADDR + WRITE_READ_ADDR, sdram_aRxBuffer, BUFFER_SIZE) != SDRAM_OK) {
        BSP_LCD_DisplayStringAt(20, 160, (uint8_t *)"SDRAM READ : FAILED", LEFT_MODE);
    } else {
        BSP_LCD_DisplayStringAt(20, 160, (uint8_t *)"SDRAM READ  : OK", LEFT_MODE);
    }

    if(Buffercmp(sdram_aTxBuffer, sdram_aRxBuffer, BUFFER_SIZE) > 0) {
        BSP_LCD_DisplayStringAt(20, 190, (uint8_t *)"SDRAM COMPARE : FAILED", LEFT_MODE);
    } else {
        BSP_LCD_DisplayStringAt(20, 190, (uint8_t *)"SDRAM Test  : OK", LEFT_MODE);
    }

    /* Emulator Test */
    EmbeddedEmulator_init();
    if (EmbeddedEmulator_load()) {
        BSP_LCD_DisplayStringAt(20, 220, (uint8_t *)"Emulator ROM Load  : OK", LEFT_MODE);
    } else {
        BSP_LCD_DisplayStringAt(20, 220, (uint8_t *)"Emulator ROM Load  : FAILED", LEFT_MODE);
    }

    BSP_LCD_Clear(LCD_COLOR_BLACK);
    
    for (uint32_t counter = 0 ; ; ++counter ) {
        EmbeddedEmulator_update_screen(&fb);
        // 描画やりまくると遅いので間引く
        if (counter & 0x04) {
            print_framebuffer(150, 0, 2);
        }
    }
}


/**
  * @brief  Fills buffer with user predefined data.
  * @param  pBuffer: pointer on the buffer to fill
  * @param  uwBufferLenght: size of the buffer to fill
  * @param  uwOffset: first value to fill on the buffer
  * @retval None
  */
static void Fill_Buffer(uint32_t *pBuffer, uint32_t uwBufferLenght, uint32_t uwOffset)
{
    uint32_t tmpIndex = 0;

    /* Put in global buffer different values */
    for (tmpIndex = 0; tmpIndex < uwBufferLenght; tmpIndex++ ) {
        pBuffer[tmpIndex] = tmpIndex + uwOffset;
    }
}

/**
  * @brief  Compares two buffers.
  * @param  pBuffer1, pBuffer2: buffers to be compared.
  * @param  BufferLength: buffer's length
  * @retval 1: pBuffer identical to pBuffer1
  *         0: pBuffer differs from pBuffer1
  */
static uint8_t Buffercmp(uint32_t* pBuffer1, uint32_t* pBuffer2, uint16_t BufferLength)
{
    while (BufferLength--) {
        if (*pBuffer1 != *pBuffer2) {
            return 1;
        }

        pBuffer1++;
        pBuffer2++;
    }

    return 0;
}
