CC := gcc

SRC_DSPBPTK := app/dspbptk.c
SRC_LIBDSPBPTK := lib/*.c
SRC_LIBDEFLATE := lib/libdeflate-1.18/lib/*.c lib/libdeflate-1.18/lib/*/*.c
SRC_TURBO_BASE64 := lib/Turbo-Base64/turbob64c.c lib/Turbo-Base64/turbob64d.c lib/Turbo-Base64/turbob64v128.c lib/Turbo-Base64/turbob64v256.c

# Use -flto so no need *.o
CFLAGS := -static -fexec-charset=GBK -Ofast -flto -march=native -mtune=native -Wall
CFLAGS_LIBDEF := -ffreestanding -DFREESTANDING

dspbptk: $(SRC_LIBDSPBPTK) $(SRC_LIBDEFLATE) $(SRC_TURBO_BASE64) $(SRC_DSPBPTK)
	$(CC) -o $@ $^ $(CFLAGS) $(CFLAGS_LIBDEF)

libdspbptk.dll: $(SRC_LIBDSPBPTK) $(SRC_LIBDEFLATE) $(SRC_TURBO_BASE64)
	$(CC) -o $@ $^ $(CFLAGS) $(CFLAGS_LIBDEF) -shared -fpic

all: dspbptk libdspbptk.dll

clear:
	rm dspbptk* libdspbptk*