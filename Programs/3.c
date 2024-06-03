#include <stdio.h>

char string_array[6][5] = {"Hey", "Hi,", "Hello", "Hi", "Crazy", "Right"};
int main()
{
    int j = 0;
    for (j = 0; j < 6; j++)
    {
        printf("%s\n", string_array[j]);
    }
}