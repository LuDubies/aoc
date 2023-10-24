#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include <windows.h>
#include <Wincrypt.h>

#define BUFSIZE 1024
#define MD5LEN  16
#define MAX_LINE_LENGTH 20

int main(void) {
    FILE *fp = fopen("input.txt", "r");

    if (fp == NULL)
    {
        printf("ERROR: Could not open input file!");
        return 1;
    }

    char line[MAX_LINE_LENGTH];
    long search = 0;

    fgets(line, MAX_LINE_LENGTH, fp);
    if (strlen(line) == 0)
    {
        printf("ERROR: String has length 0.");
    }

    HCRYPTPROV hProv = 0;
    HCRYPTHASH hHash = 0;
    char hexDigits[] = "0123456789abcdef";

    BYTE inputBuffer[BUFSIZE];
    BYTE hashBuffer[MD5LEN];
    int inputSize = 0;
    long result5 = 0;
    int result6 = 0;

    while (LONG_MAX >= search)
    {
        ++search;

        /* prep buffer */
        strcpy(inputBuffer, line);
        int digits = sprintf(inputBuffer + strlen(line), "%d", search);
        inputSize = strlen(line) + digits;

        if (search % 10000 == 0)
        {
            printf("UPDATE: Search up to %d, with buffer %s of length %d.\n", search, inputBuffer, inputSize);
        }

        /* calc MD5 */
        if (!CryptAcquireContext(&hProv, NULL, NULL, PROV_RSA_FULL,CRYPT_VERIFYCONTEXT))
        {
            printf("CryptAcquireContext failed!\n");
            return 1;
        }

        if (!CryptCreateHash(hProv, CALG_MD5, 0, 0, &hHash))
        {
            printf("CryptCreateHash failed!\n");
            return 1;
        }

        if (!CryptHashData(hHash, inputBuffer, inputSize, 0)) /* never expect to hash more than 1024 bytes */
        {
            printf("Hashing failed!\n");
            return 1;
        }

        DWORD cbHash = MD5LEN;
        if (!CryptGetHashParam(hHash, HP_HASHVAL, hashBuffer, &cbHash, 0))
        {
            printf("Retrieving hash result failed!\n");
            return 1;
        }

        CryptReleaseContext(hProv, 0);
        CryptDestroyHash(hHash);

        /* check result */
        if ((((hashBuffer[0] >> 4) & 0xf) == 0) &&
            ((hashBuffer[0] & 0xf) == 0) &&
            (((hashBuffer[1] >> 4) & 0xf) == 0) &&
            ((hashBuffer[1] & 0xf) == 0) &&
            (((hashBuffer[2] >> 4) & 0xf) == 0))
        {
            if (result5 == 0)
            {
                result5 = search;
                printf("Found solution for 5 zeros:\n");
                for (DWORD i = 0; i < cbHash; i++)
                {
                    printf("%c%c", hexDigits[hashBuffer[i] >> 4],hexDigits[hashBuffer[i] & 0xf]);
                }
                printf("\n");  
            }

            if ((hashBuffer[2] & 0xf) == 0)
            {
                result6 = search;
                printf("Found solution for 6 zeros:\n");
                for (DWORD i = 0; i < cbHash; i++)
                {
                    printf("%c%c", hexDigits[hashBuffer[i] >> 4],hexDigits[hashBuffer[i] & 0xf]);
                }
                printf("\n");
                break;
            }
            
        }
    }

    /* print results */
    if (0 == result5)
    {
        printf("ERROR: No result could be found!");
    }
    else
    {
        printf("RESULT: The lowest number to result in a 5 zero hash is %d.\n", result5);
        printf("RESULT: The lowest number to result in a 6 zero hash is %d.\n", result6);
    }

    fclose(fp);
    return 0;
}