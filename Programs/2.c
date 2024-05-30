#include <stdio.h>
#include <stdlib.h>

void main()
{
    size_t size = sizeof(int) * 1000000000; // bytes
    int *x = malloc(size);

    free(x);
    // .. tasks
    free(x);
}