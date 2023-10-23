/** see grid.h */
#include "grid.h"

/** create new coordinate and fill with defaults
 * so all pointers are NULL
*/
coordinate *newCoordinate(int idx)
{
    coordinate *newCord;
    newCord = (coordinate *) malloc(sizeof(*newCord));
    newCord->prev = NULL;
    newCord->next = NULL;
    newCord->index = idx;
    newCord->content = NULL;
    return newCord;
}

/** Get the coordinate with given index, starting search from the searchcoordinate */
coordinate *searchDimension(coordinate *searchcoord, int index)
{
    coordinate *foundCoordinate = NULL;
    while (foundCoordinate == NULL)
    {      
        if (index == searchcoord->index) /* found the right xcoordinate */
        {
            foundCoordinate = searchcoord;
        }
        else if (index < searchcoord->index) /* check lower coordinates */
        {
            if (searchcoord->prev == NULL) /* create new lower coordinate */
            {
                foundCoordinate = newCoordinate(index);
                if (foundCoordinate == NULL)
                {
                    return NULL;
                }
                searchcoord->prev = foundCoordinate;
                foundCoordinate->next = searchcoord;
            }
            else if (searchcoord->prev->index < index) /* insert new coordinate into list */
            {
                foundCoordinate = newCoordinate(index);
                if (foundCoordinate == NULL)
                {
                    return NULL;
                }
                searchcoord->prev->next = foundCoordinate;
                foundCoordinate->prev = searchcoord->prev;
                foundCoordinate->next = searchcoord;
                searchcoord->prev = foundCoordinate;
            }
            else /* search lower*/
            {
                searchcoord = searchcoord->prev;
            }
            
        }
        else if (index > searchcoord->index) /* check higher coordinates */
        {
            if (searchcoord->next == NULL) /* create new higher coordinate */
            {
                foundCoordinate = newCoordinate(index);
                if (foundCoordinate == NULL)
                {
                    return NULL;
                }
                searchcoord->next = foundCoordinate;
                foundCoordinate->prev = searchcoord;
            }
            else if (searchcoord->next->index > index) /* insert new coordinate into the list */
            {
                foundCoordinate = newCoordinate(index);
                if (foundCoordinate == NULL)
                {
                    return NULL;
                }
                searchcoord->next->prev = foundCoordinate;
                foundCoordinate->next = searchcoord->next;
                foundCoordinate->prev = searchcoord;
                searchcoord->next = foundCoordinate;
            }
            else /* search higher */
            {
                searchcoord = searchcoord->next;
            }
        }
        else
        {
            return NULL;
        }
    }
    return foundCoordinate;
}


/** Get first x coordinate with given index */
coordinate *gridInit(int index)
{
    coordinate *origin = newCoordinate(index);
    return origin;
}

/** Return the y coordinate accessed with x, y */
coordinate *getGridPosition(coordinate *grid, int x, int y)
{
    coordinate *pXc = NULL;
    coordinate *pYc = NULL;

    if (NULL == grid)
    {
        return NULL;
    }

    pXc = searchDimension(grid, x);
    if (pXc == NULL)
    {
        return NULL;
    }

    /* find or create y coordinate */
    if (pXc->content == NULL)
    {
        pYc = newCoordinate(y);
        if (pYc == NULL)
        {
            return NULL;
        }
        pXc->content = pYc;  
    }
    else
    {
        pYc = searchDimension((coordinate *) pXc->content, y);
        if (pYc == NULL)
        {
            return NULL;
        }
    }

    return pYc;
}

/** Count the number of y coordinates in the grid */
int countGrid(coordinate *grid)
{
    int totalPositions = 0;

    /** find lowes x coordinate */
    while (NULL != grid->prev)
    {
        grid = grid->prev;
    }

    while(NULL != grid)
    {
        /* find lowest y coordinate */
        coordinate *subdim = (coordinate *) grid->content;
        while (NULL != subdim->prev)
        {
            subdim = subdim->prev;
        }
        /* add all y coordinates in this x dim to dimensions */
        while (NULL != subdim)
        {
            ++totalPositions;
            subdim = subdim->next;
        }
        grid = grid->next;
    }

    return totalPositions;
}