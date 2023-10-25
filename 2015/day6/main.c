#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>

#define MAX_LINE_LENGTH 50

#define GRIDWIDTH 1000
#define LIGHTCOUNT (GRIDWIDTH * GRIDWIDTH)
#define YARDBYTES (LIGHTCOUNT / 8)
#define ROWBYTES (GRIDWIDTH / 8)
#define BYTESIZE 8

/* ALL RANGES INCLUSIVE */
/* ROWS/COLS NUMBERED 0 TO 999 */

/* memory to represent all lights */
uint8_t bitYard[YARDBYTES] = {0};
uint16_t brightnessYard[LIGHTCOUNT] = {0};

/* LUTs */
uint8_t setBitCountLUT[256] = {0};
const uint8_t startBytesLUT[8] = {255, 127, 63, 31, 15, 7, 3, 1};
const uint8_t endBytesLUT[8] = {128, 192, 224, 240, 248, 252, 254, 255};

void initBitCountLUT()
{   
    uint8_t index = 0;
    while(1)
    {
        setBitCountLUT[index] = (index & 0x1) +
                                ((index >> 1) & 0x1) +
                                ((index >> 2) & 0x1) +
                                ((index >> 3) & 0x1) +
                                ((index >> 4) & 0x1) +
                                ((index >> 5) & 0x1) +
                                ((index >> 6) & 0x1) +
                                ((index >> 7) & 0x1);
        index++;
        if (index == 0)
        {   /* wrap around*/
            break;
        }
    }   
}


void applyToYard(uint32_t byteIndex, uint8_t operator, char operation)
{
    if (operation == 't')
    {
        bitYard[byteIndex] ^= operator;
    }
    else if (operation == 'u')
    {
        bitYard[byteIndex] |= operator;
    }
    else if (operation == 'd')
    {
        bitYard[byteIndex] &= (~operator) & 0xFF;
    }
    else
    {
        printf("ERROR: Invalid operation %c!\n");
    }
}

void manipulateRow(uint32_t rowNumber, uint32_t from, uint32_t to, char operation)
{
    uint32_t startByteIndex = (rowNumber * ROWBYTES) + (from / BYTESIZE);
    uint8_t startByteOperator = startBytesLUT[from % BYTESIZE];
    uint32_t endByteIndex = (rowNumber * ROWBYTES) + (to / BYTESIZE);
    uint8_t endByteOperator = endBytesLUT[to % BYTESIZE];

    if (startByteIndex == endByteIndex)
    {   /* combine operators, they need to performed on one byte */
        startByteOperator &= endByteOperator;
    }

    /* apply start byte operation */
    applyToYard(startByteIndex, startByteOperator, operation);

    /* loop over middle section (if any) */
    for (uint32_t byteIndex = startByteIndex + 1; byteIndex < endByteIndex; byteIndex++)
    {
        applyToYard(byteIndex, 0b11111111, operation);
    }

    /* apply end byte operation if not same as start byte */
    if (startByteIndex < endByteIndex)
    {
        applyToYard(endByteIndex, endByteOperator, operation);
    }
}

void manipulateYard(uint32_t rowStart, uint32_t rowEnd, uint32_t colStart, uint32_t colEnd, char operation)
{
    if (!(operation == 't' || operation == 'u' || operation == 'd')) /* use up/down to distinguish on/off */
    {
        printf("ERROR: Invalid operation %c!\n", operation);
        return;
    }

    /* apply operation row by row */
    for (uint32_t rowIndex = rowStart; rowIndex <= rowEnd; rowIndex++)
    {
        manipulateRow(rowIndex, colStart, colEnd, operation);
    }
}


uint32_t numberOfLampsOn()
{
    uint32_t totalOn = 0;
    for (uint32_t index = 0; index < YARDBYTES; index++)
    {
        totalOn += setBitCountLUT[bitYard[index]];
    }
    return totalOn;
}

/****** PART 2 ******/
void adjustBrightness(uint32_t rowStart, uint32_t rowEnd, uint32_t colStart, uint32_t colEnd, char command)
{
    for(uint32_t rowIndex = rowStart; rowIndex <= rowEnd; rowIndex++)
    {
        for(uint32_t colIndex = colStart; colIndex <= colEnd; colIndex++)
        {
            uint32_t lightIndex = rowIndex * GRIDWIDTH + colIndex;
            if (command == 't')
            {
                brightnessYard[lightIndex] += 2;
            }
            else if(command == 'u')
            {
                brightnessYard[lightIndex] += 1;
            }
            else if((command == 'd') && (brightnessYard[lightIndex] > 0))
            {
                brightnessYard[lightIndex] -= 1;
            }
        }
    }
}

uint32_t totalBrightness()
{
    uint64_t brightnessAcc = 0;
    for (uint32_t index = 0; index < LIGHTCOUNT; index++)
    {
        brightnessAcc += brightnessYard[index];
    }
    return brightnessAcc;
}
/****** PART 2 DONE ******/


int main(void) {
    FILE *fp = fopen("input.txt", "r");

    if (fp == NULL)
    {
        printf("ERROR: Could not open input file!");
        return 1;
    }

    initBitCountLUT();
    memset(bitYard, 0, sizeof(bitYard));
    printf("Setup done.\n");

    char line[MAX_LINE_LENGTH];

    while(fgets(line, MAX_LINE_LENGTH, fp))
    {
        uint32_t rowStart;
        uint32_t rowEnd;
        uint32_t colStart;
        uint32_t colEnd;
        char command;
        char *token;

        /* parse command */
        token = strtok(line, " ");
        if (strcmp(token, "toggle") == 0)
        {
            command = 't';
        }
        else if (strcmp(token, "turn") == 0)
        {
            token = strtok(NULL, " ");
            if (strcmp(token, "on") == 0)
            {
                command = 'u';
            }
            else if (strcmp(token, "off") == 0)
            {
                command = 'd';
            }
            else
            {
                printf("ERROR: Invalid token %s in %s.\n", token, line);
            }
        }
        else
        {
            printf("ERROR: Invalid token %s in %s.\n", token, line);
        }

        /* parse parameters */
        token = strtok(NULL, ",");
        if (token == NULL)
        {
            printf("ERROR: NULL token!\n");
            return 1;
        }
        rowStart = (uint32_t) atoi(token);
        token = strtok(NULL, " ");
        if (token == NULL)
        {
            printf("ERROR: NULL token!\n");
            return 1;
        }
        colStart = (uint32_t) atoi(token);

        token = strtok(NULL, " ");
        if (strcmp(token, "through") != 0)
        {
            printf("ERROR: Expected 'trough', instead got %s!\n", token);
            return 1;
        }

        token = strtok(NULL, ",");
        if (token == NULL)
        {
            printf("ERROR: NULL token!\n");
            return 1;
        }
        rowEnd = (uint32_t) atoi(token);
        token = strtok(NULL, " ");
        if (token == NULL)
        {
            printf("ERROR: NULL token!\n");
            return 1;
        }
        colEnd = (uint32_t) atoi(token);
        if(strtok(NULL, " ") != NULL)
        {
            printf("ERROR: There was more to be parsed!\n");
            return 1;
        }

        printf("Parsed command %c from %d,%d to %d,%d.\n", command, rowStart, colStart, rowEnd, colEnd);
        manipulateYard(rowStart, rowEnd, colStart, colEnd, command);
        adjustBrightness(rowStart, rowEnd, colStart, colEnd, command);
        printf("INFO: Current brightness is %d\n", totalBrightness());
    }

    fclose(fp);

    printf("\nAfter all steps there are %d lamps on in the yard.\n", numberOfLampsOn());
    printf("\nWhen translating right the total brightness is %d.\n", totalBrightness());
    return 0;
}