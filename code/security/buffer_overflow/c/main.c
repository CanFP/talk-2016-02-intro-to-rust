#include <stdio.h>
#include <string.h>

int main(int argc, char *argv[]) {
    char hello[] = "hello";
    char buffer[10] = {0};
    //stringcopy(argv[1], buffer);
    strcpy(buffer, argv[1]);
    printf("hello: %s\n", hello);
    printf("buffer: %s\n", buffer);
}

void stringcopy(char *src, char *dst);
