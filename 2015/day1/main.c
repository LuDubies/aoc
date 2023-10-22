#include <stdio.h>

const char* filename = "input.txt";

int main(void)
{
    FILE *fp = fopen(filename, "r"); 

    if (fp == NULL) {
        printf("ERROR: Could not open file %s!", filename);
        return 1;
    }

    int floor = 0;
    unsigned int current_position = 1;
    unsigned int cellar_position = -1;
    char inchar;
    while((inchar = fgetc(fp)) != EOF)
    {
        if (inchar == '(') {
            ++floor;
        } else if (inchar == ')')
        {
            --floor;
        }

        if (floor < 0 && cellar_position == -1) {
            cellar_position = current_position;
        }
        ++current_position;
    }
    fclose(fp);
    
    printf("RESULT: Santa is on floor %d.\n", floor);
    if (cellar_position > 0) {
        printf("RESULT: He first entered the cellar with the instruction on position %d.\n", cellar_position);
    }
     else
    {
        printf("RESULT: He never entered the cellar.\n");
    }
    return 0;
}