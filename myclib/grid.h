/** Module to hold a simple x, y grid holding arbitrary contents
 * dimensions are implemented as doubly linked lists
*/
#ifndef GRID_H
#define GRID_H

#include <stdio.h>
#include <stdlib.h>

#ifdef __cplusplus
extern "C"
{
#endif

typedef struct coordinate {
    struct coordinate *prev;
    struct coordinate *next;
    int index;
    void *content;
} coordinate;

/** Create new coordinate struct with pointers set to NULL */
coordinate *newCoordinate(int idx);

/** get the coordinate with the given index in the dimension of searchcoord.
 * Will create and return new coordinate if not found.
*/
coordinate *searchDimension(coordinate *searchcoord, int index);

/* get first coordinate of the grid with the given index */
coordinate *gridInit(int index);

/* get the lowest level coordinate for the given indices */
coordinate *getGridPosition(coordinate *grid, int x, int y);

/* count all the y coordinates in the grid */
int countGrid(coordinate *grid);

#ifdef __cplus_plus__
}
#endif

#endif /* GRID_H*/