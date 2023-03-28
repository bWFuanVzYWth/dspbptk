#include <stdio.h>
#include <stdlib.h>

#include "../lib/libdspbptk.h"

int main(int argc, char* argv[]) {
    char* str = (char*)calloc(BLUEPRINT_MAX_LENGTH, sizeof(char));
    FILE* fp = fopen(argv[1], "rw");

    blueprint_t bp;
    blueprint_decode(&bp, str);
    blueprint_encode(&bp, str);

    fclose(fp);
    free(str);
    fprintf(stderr, "Finish.\n");
    return 0;
}