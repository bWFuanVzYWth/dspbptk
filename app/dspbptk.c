#include <stdio.h>
#include <stdlib.h>

#include "../lib/libdspbptk.h"

int main(int argc, char* argv[]) {
#ifdef DSPBPTK_DEBUG
    for(int i = 0; i < argc; i++)
        printf("argv[%d]=%s\n", i, argv[i]);
#endif
    char* str = (char*)calloc(1 << 28, sizeof(char));
        char* str_out = (char*)calloc(1 << 28, sizeof(char));
    FILE* fp = fopen(argv[1], "r");

    size_t parameters_count = fscanf(fp, "%s", str);
    if(parameters_count < 1)
        printf("warning: no string\n");

    fclose(fp);

    blueprint_t bp;
    dspbptk_error_t errorlevel;
    errorlevel = blueprint_decode(&bp, str);
    if(errorlevel) {
        printf("dec err: %d\n", errorlevel);
        goto error;
    }

    errorlevel = blueprint_encode(&bp, str_out);
    if(errorlevel) {
        printf("enc err: %d\n", errorlevel);
        goto error;
    }

    fp = fopen("o.txt", "w");
    fprintf(fp, "%s", str_out);
    fclose(fp);

    free_blueprint(&bp);

    free(str_out);
    free(str);
    printf("Finish.\n");
    return 0;

error:
    return errorlevel;
}