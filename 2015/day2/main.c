#include <stdio.h>
#include <string.h>
#include <stdlib.h>

#define max(a,b) (((a) > (b)) ? (a) : (b))
#define min(a,b) (((a) < (b)) ? (a) : (b))

#define MAX_LINE_LENGTH 256

void calcSides(int lengths[], int sides[])
{
    sides[0] = lengths[0] * lengths[1];
    sides[1] = lengths[0] * lengths[2];
    sides[2] = lengths[1] * lengths[2];
}

unsigned int calcPaperNeeds(unsigned int areas[]) {
    unsigned int smallest = min(min(areas[0], areas[1]),areas[2]);
    return 2 * areas[0] + 2 * areas[1] + 2 * areas[2] + smallest;
}

unsigned int calcRibbon(unsigned int sides[]) {
    unsigned int bow = sides[0] * sides[1] * sides[2];
    unsigned int circumv = 2 * sides[0] + 2 * sides[1];
    circumv = min(circumv, 2 * sides[0] + 2 * sides[2]);
    circumv = min(circumv, 2 * sides[1] + 2* sides[2]);
    return bow + circumv;
}

int main(void) {
    FILE *fp = fopen("input.txt", "r");

    if (fp == NULL)
    {
        printf("ERROR: Could not open input file!");
        return 1;
    }

    char *linepart;
    char line[MAX_LINE_LENGTH];
    unsigned int lengths[3];
    unsigned int sideAreas[3];
    unsigned long totalPaperNeed = 0;
    unsigned long totalRibbonNeed = 0;

    while(fgets(line, MAX_LINE_LENGTH, fp))
    {
        /* read metrics */
        char * slength = strtok(line, "x");
        lengths[0] = (unsigned int) atoi(slength);
        slength = strtok(NULL, "x");
        lengths[1] = (unsigned int) atoi(slength);
        slength = strtok(NULL, "x");
        lengths[2] = (unsigned int) atoi(slength);

        /* calculate paper */
        calcSides(lengths, sideAreas);
        totalPaperNeed += calcPaperNeeds(sideAreas);

        /* calculate ribbon */
        totalRibbonNeed += calcRibbon(lengths);
    }

    fclose(fp);

    printf("RESULT: Total need is %d square feet of paper!\n", totalPaperNeed);
    printf("RESULT: Total ribbon needed is %d feet!\n", totalRibbonNeed);
}