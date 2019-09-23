#include "mbed.h"
#include "stm32f769i_discovery.h"
#include "stm32f769i_discovery_lcd.h"
#include "stm32f769i_discovery_sdram.h"

static void Fill_Buffer(uint32_t *pBuffer, uint32_t uwBufferLenght, uint32_t uwOffset);
static uint8_t Buffercmp(uint32_t* pBuffer1, uint32_t* pBuffer2, uint16_t BufferLength);

#define BUFFER_SIZE            ((uint32_t)0x0100)
#define WRITE_READ_ADDR        ((uint32_t)0x0800)
#define SDRAM_WRITE_READ_ADDR  ((uint32_t)0xC0177000)

uint32_t sdram_aTxBuffer[BUFFER_SIZE];
uint32_t sdram_aRxBuffer[BUFFER_SIZE];

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

    while (1) {

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
