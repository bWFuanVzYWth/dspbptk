CC := gcc

SRC_LIBDEFLATE := lib/libdeflate/lib/*.c lib/libdeflate/lib/*/*.c
SRC_TURBO_BASE64 := lib/Turbo-Base64/turbob64c.c lib/Turbo-Base64/turbob64d.c lib/Turbo-Base64/turbob64v128.c lib/Turbo-Base64/turbob64v256.c

SRC_BPOPT := app/bpopt.c
SRC_LIBDSPBPTK := lib/*.c lib/*.h $(SRC_LIBDEFLATE) $(SRC_TURBO_BASE64)

CFLAGS := -fexec-charset=GBK -Wall -Ofast -flto -pipe -march=x86-64 -mtune=generic

#CFLAGS += -g -fsanitize=address -fno-omit-frame-pointer

bpopt: $(SRC_LIBDSPBPTK) $(SRC_BPOPT)
	$(CC) -o $@ $^ $(CFLAGS)

libdspbptk.dll: $(SRC_LIBDSPBPTK)
	$(CC) -o $@ $^ $(CFLAGS) -shared -fpic

all: bpopt libdspbptk.dll

clear:
	rm bpopt* libdspbptk*