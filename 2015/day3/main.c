#include <stdio.h>

#include "grid.h"

/** search the given location in the grid and increment the presents count */
int visit(coordinate *house) {
    if (NULL == house)
    {
        return 1; /* fail if no house is given */
    }

    /* create or increment present counter */
    int *presents;
    if (house->content == NULL)
    {
        presents = (int *) malloc(sizeof(*presents));
        if (presents == NULL)
        {
            return 1;
        }
        *presents = 1;
        house->content = (void *) presents;
    }
    else{
        presents = (int*) house->content;
        *presents += 1;
        house->content = (void *) presents;
    }

    return 0;
}


int main(void)
{
    FILE *fp = fopen("input.txt", "r");
    if (fp == NULL)
    {
        printf("ERROR: Could not open input file!");
        return 1;
    }

    char command;
    int x = 0;
    int y = 0;

    /* tracking for step 2 */
    int santaX = 0;
    int santaY = 0;
    int roboX = 0;
    int roboY = 0;
    unsigned int step = 0;
        
    coordinate *grid = gridInit(0);
    coordinate *sharedGrid = gridInit(0);

    coordinate *house = getGridPosition(grid, x, y);
    if (0 != visit(house))
    {
        printf("ERROR: Visit failed!");
        return 1;
    }
    house = getGridPosition(sharedGrid, x, y);
    if (0 != visit(house))
    {
        printf("ERROR: Visit failed!");
        return 1;
    }


    while((command = fgetc(fp)) != EOF)
    {
        ++step;
        /* parse the elves command */
        if ('^' == command)
        {
            ++y; /* up */
            santaY += step % 2;
            roboY += (step+1)%2;
        }
        else if ('v' == command)
        {
            --y; /* down */
            santaY -= step % 2;
            roboY -= (step+1)%2;
        }
        else if ('<' == command)
        {
            --x; /* left */
            santaX -= step % 2;
            roboX -= (step+1)%2;
        }
        else if ('>' == command)
        {
            ++x; /* right */
            santaX += step % 2;
            roboX += (step+1)%2;
        }
        else
        {
            printf("ERROR: Invalid command: %c!", command);
            return 1;
        }
        
        /* visit house */
        house = getGridPosition(grid, x, y);
        if (0 != visit(house))
        {
            printf("ERROR: Visit failed!");
            return 1;
        }
        if ((step % 2) == 1)
        {
            house = getGridPosition(sharedGrid, santaX, santaY);
        }
        else{
            house = getGridPosition(sharedGrid, roboX, roboY);
        }
        if (0 != visit(house))
        {
            printf("ERROR: Visit failed!");
            return 1;
        }
        
    }

    /* calculate results */
    int housesVisited = countGrid(grid);
    printf("Santa alone visited a total of %d different houses.\n", housesVisited);
    int housesVisitedWithRoboHelp = countGrid(sharedGrid);
    printf("With Robo-Santas help, they managed to visit %d houses!\n", housesVisitedWithRoboHelp);
    
    fclose(fp);
}