#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>

#define MAX_LINE_LENGTH 50
#define HASH_TABLE_SIZE 251 /* cant hurt to use a prime (hopefully) */

/* hash table entry */
typedef struct te
{
    struct te *next;
    char *identifier;
    uint8_t set;
    uint16_t signal;
} tableEntry;



int main(void) {
    FILE *fp = fopen("input.txt", "r");

    if (fp == NULL)
    {
        printf("ERROR: Could not open input file!");
        return 1;
    }

    char line[MAX_LINE_LENGTH];

    while(fgets(line, MAX_LINE_LENGTH, fp))
    {

    }

    fclose(fp);
    return 0;
}