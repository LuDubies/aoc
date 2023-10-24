/* This solve needs the GNU C lib regex.h */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>
#include <regex.h> /* For windows get from mingw-w64-ucrt-x86_64-libgnurx */


#define MAX_LINE_LENGTH 20

/** Returns 1 if string contains a doule char (adjacent), else 0 */
int containsDouble(char *string)
{   
    int result = 0;
    for (uint8_t charPosition = 1; charPosition < strlen(string); charPosition++)
    {
        if (string[charPosition] == string[charPosition - 1])
        {
            result = 1;
            break;
        }
    }
    return result;
}

int main(void) {
    FILE *fp = fopen("input.txt", "r");

    if (fp == NULL)
    {
        printf("ERROR: Could not open input file!");
        return 1;
    }
    uint32_t niceLinesCount = 0;
    uint32_t niceLinesCountPartTwo = 0;

    re_syntax_options = RE_SYNTAX_GREP; /* use egrep dialect */

    /* prep regexes for part 1 */
    regex_t vowelRegex;
    vowelRegex.translate = 0;
    vowelRegex.fastmap = 0;
    vowelRegex.buffer = 0;
    vowelRegex.allocated = 0;

    regex_t badPatternRegex;
    badPatternRegex.translate = 0;
    badPatternRegex.fastmap = 0;
    badPatternRegex.buffer = 0;
    badPatternRegex.allocated = 0;

    const char *vowelPattern = ".*[aeiou].*[aeiou].*[aeiou].*";
    re_compile_pattern(vowelPattern, strlen(vowelPattern), &vowelRegex);
    const char *badPattern = "^.*\\(ab\\|cd\\|pq\\|xy\\).*";
    re_compile_pattern(badPattern, strlen(badPattern), &badPatternRegex);

    /* prep regexes for part 2 */
    regex_t repeatRegex;
    repeatRegex.translate = 0;
    repeatRegex.fastmap = 0;
    repeatRegex.buffer = 0;
    repeatRegex.allocated = 0;

    const char *repeatPattern = ".*\\([a-z]\\{2\\}\\).*\\1.*";
    re_compile_pattern(repeatPattern, strlen(repeatPattern), &repeatRegex);

    regex_t hugRegex;
    hugRegex.translate = 0;
    hugRegex.fastmap = 0;
    hugRegex.buffer = 0;
    hugRegex.allocated = 0;

    const char *hugPattern = ".*\\([a-z]\\).\\1.*";
    re_compile_pattern(hugPattern, strlen(hugPattern), &hugRegex);

    /* check line by line for naughtiness */
    char line[MAX_LINE_LENGTH];
    int vowelRes;
    int doubleRes;
    int badRes;
    int repeatRes;
    int hugRes;
    while(fgets(line, MAX_LINE_LENGTH, fp))
    {
        /* check old rules */
        vowelRes = re_match(&vowelRegex, line, strlen(line), 0, NULL) >= 0;
        doubleRes = containsDouble(line);
        badRes =  re_match(&badPatternRegex, line, strlen(line), 0, NULL) >= 0;
        if (vowelRes && doubleRes && !badRes)
        {
            ++niceLinesCount;
        }

        /* check new rules */
       repeatRes = re_match(&repeatRegex, line, strlen(line), 0, NULL) >= 0;
       hugRes = re_match(&hugRegex, line, strlen(line), 0, NULL) >= 0;
       if (repeatRes && hugRes)
       {
            ++niceLinesCountPartTwo;
       }

    }


    printf("RESULT: Found %d nice lines according to the first rules.\n", niceLinesCount);
    printf("RESULT: The new rules result in %d nice lines.\n", niceLinesCountPartTwo);

    fclose(fp);
    return 0;
}