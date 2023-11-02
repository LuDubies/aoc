#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>
#include <ctype.h>

#define MAX_LINE_LENGTH 50  /* max input line size */
#define MAX_ID_LEN 3        /* max register name length */
#define HASH_TABLE_SIZE 701 /* cant hurt to use a prime (hopefully) */

enum Operation
{
    IMM,
    NOT,
    AND,
    OR,
    RSHIFT,
    LSHIFT
};

/* hash table entry */
typedef struct te
{
    char identifier[MAX_ID_LEN + 1];
    uint8_t set;                 /* has the signal been computed*/
    enum Operation operation;    /* how the signal is computed from the inputs */
    uint16_t signal;             /* the signal carried by the wire */
    char input1[MAX_ID_LEN + 1]; /* identifier of first input, can be ignored if operation is imm */
    char input2[MAX_ID_LEN + 1]; /* identifier of second input, can be ignored for unary operations */
} wire;

/** Get mem for a new wire, set defaults and return pointer to memory */
wire *getNewWire(char *ident)
{
    if (strlen(ident) > 3)
    { /* identifier to long */

        return NULL;
    }
    wire *pNewWire = (wire *)malloc(sizeof(*pNewWire));
    if (NULL == pNewWire)
    { /* malloc failed */
        return NULL;
    }
    (void *)strcpy(pNewWire->identifier, ident);
    pNewWire->operation = 0xFF;
    pNewWire->signal = 0;
    pNewWire->set = 0;
    return pNewWire;
}

/* hash table */
wire *wireTable[HASH_TABLE_SIZE] = {0};

/** print table content */
void printTable()
{
    for (uint16_t index = 0; index < HASH_TABLE_SIZE; index++)
    {
        if (wireTable[index] != NULL)
        {
            printf("Table entry %d contains Wire with identifier %s.\n", index, wireTable[index]->identifier);
            if (wireTable[index]->set)
            {
                printf("It's signal is calculated as %d.\n", wireTable[index]->signal);
            }
        }
    }
}

/* super bad hashing algorithm for table key */
uint16_t badHash(char *key)
{
    uint16_t digest = 0xA5A5;

    for (uint8_t charIndex = 0; key[charIndex] != 0; charIndex++)
    {
        char c = key[charIndex];
        uint16_t operation1 = (((uint16_t)c) * 251) % UINT16_MAX;
        uint16_t operation2 = (uint16_t)c;
        operation2 = operation2 << (c % 8);
        digest = digest ^ (operation1 ^ operation2);
    }

    return digest % HASH_TABLE_SIZE;
}

/** retreibe wire by identifier from the table */
wire *getWire(char *wireIdentifier)
{
    wire *inspectedElement;
    uint16_t hash = badHash(wireIdentifier);

    while (wireTable[hash] != NULL)
    {
        inspectedElement = wireTable[hash];
        if (strcmp(inspectedElement->identifier, wireIdentifier) == 0)
        {
            return inspectedElement;
        }
        hash = (hash + 1) % HASH_TABLE_SIZE;
    }
    return NULL;
}

/** insert given wire into the table */
wire *insertWire(char *wireIdentifier)
{
    if (getWire(wireIdentifier) != NULL) /* if already in table, return that*/
    {
        return getWire(wireIdentifier);
    }

    wire *pWireElement = getNewWire(wireIdentifier);
    uint16_t hash = badHash(wireIdentifier);

    while (wireTable[hash] != NULL) /* linear probing */
    {
        hash = (hash + 1) % HASH_TABLE_SIZE;
    }
    wireTable[hash] = pWireElement;
    return pWireElement;
}

void parseInputLine(char *line)
{
    char *token;
    wire *pParsedWire;
    uint16_t signal = 0;
    char op1buf[4];
    char op2buf[4];
    enum Operation tmpOp;

    token = strtok(line, " ");

    if (strcmp(token, "NOT") == 0)
    {
        /* parse NOT unary operation */
        token = strtok(NULL, " ");
        strcpy(op1buf, token);
        token = strtok(NULL, " "); /* ignore -> */
        token = strtok(NULL, "\n");

        pParsedWire = insertWire(token);
        pParsedWire->signal = signal;
        pParsedWire->operation = NOT;
        strcpy(pParsedWire->input1, op1buf);
        pParsedWire->set = 0;
    }
    else
    {
        /* parse big operation */
        strcpy(op1buf, token);

        token = strtok(NULL, " ");

        if (strcmp("->", token) == 0)
        {
            /* parse immediate value for wire */
            signal = atoi(op1buf);
            token = strtok(NULL, "\n");

            pParsedWire = insertWire(token);
            pParsedWire->signal = signal;
            pParsedWire->operation = IMM;
            pParsedWire->set = 1;
        }
        else if (strcmp("AND", token) == 0)
        {
            tmpOp = AND;
        }
        else if (strcmp("OR", token) == 0)
        {
            tmpOp = OR;
        }
        else if (strcmp("LSHIFT", token) == 0)
        {
            tmpOp = LSHIFT;
        }
        else if (strcmp("RSHIFT", token) == 0)
        {
            tmpOp = RSHIFT;
        }
        else
        {
            printf("ERROR: Failed to parse operation token %s.", token);
            return;
        }

        token = strtok(NULL, " ");
        strcpy(op2buf, token);
        token = strtok(NULL, " "); /* ignore -> */
        token = strtok(NULL, "\n");

        pParsedWire = insertWire(token);
        pParsedWire->signal = signal;
        pParsedWire->operation = tmpOp;
        strcpy(pParsedWire->input1, op1buf);
        strcpy(pParsedWire->input2, op2buf);

        pParsedWire->set = 0;
    }
}

uint8_t computeSignal(char *ident)
{
    wire *pTarget = getWire(ident);
    if (pTarget->set)
    {
        /* already computed */
        return 1;
    }

    /* need to get input 1 for all operations except IMM, which is alwys set */
    wire *pInput1 = getWire(pTarget->input1);
    if (NULL == pInput1)
    {
        printf("ERROR: Unable to retrieve Input1 (%s) for wire %s.\n", pTarget->input1, ident);
        return 0;
    }
    if (0 == pInput1->set)
    {
        return 0; /* cant compute from uncomputed value */
    }

    if (NOT == pTarget->operation)
    {
        pTarget->signal = (~pInput1->signal) & 0xFFFF;
        pTarget->set = 1;
    }
    else if (RSHIFT == pTarget->operation)
    {
        uint8_t shiftamnt = (uint8_t)atoi(pTarget->input2);
        pTarget->signal = (pInput1->signal >> shiftamnt) & 0xFFFF;
        pTarget->set = 1;
    }
    else if (LSHIFT == pTarget->operation)
    {
        uint8_t shiftamnt = (uint8_t)atoi(pTarget->input2);
        pTarget->signal = (pInput1->signal << shiftamnt) & 0xFFFF;
        pTarget->set = 1;
    }
    else
    {
        wire *pInput2 = getWire(pTarget->input2);
        if (NULL == pInput2)
        {
            printf("ERROR: Unable to retrieve Input2 (%s) for wire %s.\n", pTarget->input2, ident);
            return 0;
        }
        if (0 == pInput2->set)
        {
            return 0; /* cant compute from uncomputed value */
        }

        if (AND == pTarget->operation)
        {
            pTarget->signal = (pInput1->signal & pInput2->signal) & 0xFFFF;
            pTarget->set = 1;
        }
        else
        {
            pTarget->signal = (pInput1->signal | pInput2->signal) & 0xFFFF;
            pTarget->set = 1;
        }
    }

    return pTarget->set;
}

uint16_t computeSignalSweep()
{
    uint16_t totalCount = 0;
    uint16_t calcedCount = 0;

    for (uint16_t index = 0; index < HASH_TABLE_SIZE; index++)
    {
        if (NULL != wireTable[index])
        {
            totalCount += 1;
            calcedCount += computeSignal(wireTable[index]->identifier);
        }
    }

    return totalCount - calcedCount;
}

int main(void)
{
    FILE *fp = fopen("input.txt", "r");

    if (fp == NULL)
    {
        printf("ERROR: Could not open input file!");
        return 1;
    }

    char line[MAX_LINE_LENGTH];

    while (fgets(line, MAX_LINE_LENGTH, fp))
    {
        /* save all wires in hash table */
        parseInputLine(line);
    }

    printTable();

    /* compute signals */
    uint16_t unsolved = 0xFFFF;
    uint16_t abortCntr = 0;
    while (0 != unsolved && abortCntr < 100)
    {
        unsolved = computeSignalSweep();
        abortCntr += 1;
        printf("DEBUG: Have %d uncomputed signals after round %d.\n", unsolved, abortCntr);
    }

    printTable();

    fclose(fp);
    return 0;
}