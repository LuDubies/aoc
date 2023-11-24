#include <stdint.h>
#include <stdio.h>
#include <string.h>

#define MAX_LINE_LENGTH 100


int main(void)
{
    FILE *fp = fopen("input.txt", "r");

    if (fp == NULL)
    {
        printf("ERROR: Could not open input file!");
        return 1;
    }

    char line[MAX_LINE_LENGTH];

    uint32_t charsInLiteral = 0;
    uint32_t totalChars = 0;
    uint32_t bytesNeeded = 0;
    uint32_t totalBytes = 0;
    uint32_t fullEscapeChars = 0;
    uint32_t totalExpanded = 0;
    size_t lineLength = 0;

    while (fgets(line, MAX_LINE_LENGTH, fp))
    {
        line[strcspn(line, "\r\n")] = 0; /* get rid of \n */
        lineLength = strlen(line);
        charsInLiteral = 2;
        bytesNeeded = 0;
        fullEscapeChars = 6;
        uint8_t i = 1;
        while(i < (lineLength - 1))
        {
            if ('\\' == line[i])
            {
                if ('x' == line[i+1])
                {
                    charsInLiteral += 4;
                    bytesNeeded += 1;
                    fullEscapeChars += 5;
                    i += 4;
                }
                else
                {
                    charsInLiteral += 2;
                    bytesNeeded += 1;
                    fullEscapeChars += 4;
                    i += 2;
                }
            }
            else
            {
                charsInLiteral += 1;
                bytesNeeded += 1;
                fullEscapeChars += 1;
                ++i;
            }
        }
        totalChars += charsInLiteral;
        totalBytes += bytesNeeded;
        totalExpanded += fullEscapeChars;
        printf("INFO: Line %s (len %d) is %d chars as literal, %d chars in memory and %d fully escaped.\n",
         line, lineLength, charsInLiteral, bytesNeeded, fullEscapeChars);
    }

    printf("\nRESULT: %d - %d = %d\n", totalChars, totalBytes, (totalChars - totalBytes));
    printf("\nRESULT: %d - %d = %d\n", totalExpanded, totalChars, (totalExpanded - totalChars));

    fclose(fp);
    return 0;
}