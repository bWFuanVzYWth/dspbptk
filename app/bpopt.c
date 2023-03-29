#include <stdio.h>
#include <stdlib.h>
#include <time.h>

#include "../lib/libdspbptk.h"

uint64_t get_timestamp(void) {
    struct timespec t;
    clock_gettime(0, &t);
    return (uint64_t)t.tv_sec * 1000000000 + (uint64_t)t.tv_nsec;
}

double d_t(uint64_t t1, uint64_t t0) {
    return (double)(t1 - t0) / 1000000.0;
}

int main(int argc, char* argv[]) {
#ifdef DSPBPTK_DEBUG
    for(int i = 0; i < argc; i++)
        printf("argv[%d]=%s\n", i, argv[i]);
#endif
    char* str_i = (char*)calloc(BLUEPRINT_MAX_LENGTH, sizeof(char));
    char* str_o = (char*)calloc(BLUEPRINT_MAX_LENGTH, sizeof(char));
    FILE* fp = fopen(argv[1], "r");
    if(fp == NULL)
        return -1;
    size_t parameters_count = fscanf(fp, "%s", str_i);
    fclose(fp);
    if(parameters_count < 1)
        fprintf(stderr, "warning: no string\n");

    blueprint_t bp;
    dspbptk_error_t errorlevel;

    uint64_t t_dec_0 = get_timestamp();
    errorlevel = blueprint_decode(&bp, str_i);
    uint64_t t_dec_1 = get_timestamp();
    fprintf(stderr, "dec time = %.3lf ms\n", d_t(t_dec_1, t_dec_0));
    if(errorlevel) {
        fprintf(stderr, "dec error: %d\n", errorlevel);
        goto error;
    }

    uint64_t t_enc_0 = get_timestamp();
    errorlevel = blueprint_encode(&bp, str_o);
    uint64_t t_enc_1 = get_timestamp();
    fprintf(stderr, "enc time = %.3lf ms\n", d_t(t_enc_1, t_enc_0));
    if(errorlevel) {
        fprintf(stderr, "enc error: %d\n", errorlevel);
        goto error;
    }

    size_t strlen_i = strlen(str_i);
    size_t strlen_o = strlen(str_o);
    fprintf(stderr, "strlen_i = %zu\nstrlen_o = %zu (%.3lf%%)\n",
        strlen_i, strlen_o, ((double)strlen_o / (double)strlen_i - 1.0) * 100.0);
    if(strlen_o < strlen_i) {
        FILE* fp = fopen(argv[1], "w");
        if(fp == NULL)
            return -1;
        fprintf(fp, "%s", str_o);
        fclose(fp);
    }
    else {
        fprintf(stderr, "Origin blueprint is smaller. Nothing Changed.\n");
    }
    fclose(fp);

    free_blueprint(&bp);

    free(str_o);
    free(str_i);
    printf("Finish.\n");
    return 0;

error:
    return errorlevel;
}