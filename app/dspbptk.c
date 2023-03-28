#include <stdio.h>
#include <stdlib.h>

#include "../lib/libdspbptk.h"

int main(int argc, char* argv[]) {
    char* str = (char*)calloc(1 << 24, sizeof(char));
    FILE* fp = fopen(argv[1], "rw");

    fscanf(fp, "%s", str);

    blueprint_t bp;
    dspbptk_error_t errorlevel_dec = blueprint_decode(&bp, str);
    dspbptk_error_t errorlevel_enc = blueprint_encode(&bp, str);
    if(errorlevel_dec)
        fprintf(stderr, "dec err: %d\n", errorlevel_dec);
    if(errorlevel_enc)
        fprintf(stderr, "enc err: %d\n", errorlevel_enc);

    free_blueprint(&bp);

    fclose(fp);
    free(str);
    fprintf(stderr, "Finish.\n");
    return 0;
}