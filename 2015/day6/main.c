#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>

#define MAX_LINE_LENGTH 50

#define ROWCNT 1000
#define COLCNT 1000
#define YARDBYTES ((ROWCNT * COLCNT) / 8)

int main(void) {
    FILE *fp = fopen("input.txt", "r");

    if (fp == NULL)
    {
        printf("ERROR: Could not open input file!");
        return 1;
    }

    char line[MAX_LINE_LENGTH];

    uint8_t yard[YARDBYTES] = {0};

    // while(fgets(line, MAX_LINE_LENGTH, fp))
    // {

    // }

    fclose(fp);
    return 0;
}