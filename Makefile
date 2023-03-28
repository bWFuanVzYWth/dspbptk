CC := gcc

SRC_DSPBPTK := app/dspbptk.c
SRC_LIBDSPBPTK := lib/*.c
SRC_LIBDEFLATE := lib/libdeflate/lib/*.c lib/libdeflate/lib/*/*.c
SRC_TURBO_BASE64 := lib/Turbo-Base64/turbob64c.c lib/Turbo-Base64/turbob64d.c lib/Turbo-Base64/turbob64v128.c lib/Turbo-Base64/turbob64v256.c

# Use -flto so no need *.o
CFLAGS := -fexec-charset=GBK -Wall -Ofast -flto -pipe
CFLAGS_LIBDEF := -ffreestanding -DFREESTANDING
CFLAGS_MATCH := -march=native -mtune=native

CFLAGS += -g -fsanitize=address -fno-omit-frame-pointer

dspbptk: $(SRC_LIBDSPBPTK) $(SRC_LIBDEFLATE) $(SRC_TURBO_BASE64) $(SRC_DSPBPTK)
	$(CC) -o $@ $^ $(CFLAGS) $(CFLAGS_LIBDEF) $(CFLAGS_MATCH)

libdspbptk.dll: $(SRC_LIBDSPBPTK) $(SRC_LIBDEFLATE) $(SRC_TURBO_BASE64)
	$(CC) -o $@ $^ $(CFLAGS) $(CFLAGS_LIBDEF) $(CFLAGS_MATCH) -shared -fpic

all: dspbptk libdspbptk.dll

clear:
	rm test* dspbptk* libdspbptk*