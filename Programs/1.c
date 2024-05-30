#include <stdio.h>
#include <stdlib.h>

void main()
{
    size_t size = sizeof(int) * 1000000000; // bytes
    int i = 1;
    int *x;
    while (i >= 1)
    {
        x = malloc(size);
        printf("Iteration :%d \n", i);
        i++;
        if (x == NULL)
            break;
    }
}