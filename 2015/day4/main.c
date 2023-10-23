#include <stdio.h>
#include <stdlib.h>
#include <string.h>

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

    char *linepart;
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

    unsigned char inputBuffer[BUFSIZE];
    unsigned char hashBuffer[MD5LEN];
    int inputSize = 0;
    long result = 0;

    while (LONG_MAX >= search)
    {
        /* prep buffer */


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

        if (!CryptHashData(hHash, inputBuffer, inputSize, 0)) /* never expect to hasm more than 1024 bytes s*/
        {
            printf("Hashing failed!\n");
            return 1;
        }

        int cbHash = MD5LEN;
        if (!CryptGetHashParam(hHash, HP_HASHVAL, hashBuffer, &cbHash, 0))
        {
            printf("Retrieving hash result failed!\n");
            return 1;
        }
        for (int i = 0; i < cbHash; i++)
        {
            printf("%c%c", hexDigits[hashBuffer[i] >> 4], hexDigits[hashBuffer[i] & 0xf]);
        }
        printf("\n");

        CryptReleaseContext(hProv, 0);
        CryptDestroyHash(hHash);

        /* check result */
    }

    

    if (0 == result)
    {
        printf("ERROR: No result could be founcd!");
    }
    else
    {
        printf("RESULT: The lowest number to get a qualified has is %d.", result);
    }

    fclose(fp);
    return 0;
}