CC := gcc

SRC_BPOPT := app/bpopt.c
SRC_LIBDSPBPTK := lib/*.c lib/*.h lib/libdeflate/lib/*.c lib/libdeflate/lib/*/*.c lib/Turbo-Base64/libtb64.a

CFLAGS := -fexec-charset=GBK -Wall -Ofast -flto -pipe -march=native -mtune=native -static
#CFLAGS += -g -fsanitize=address -fno-omit-frame-pointer

bpopt: $(SRC_LIBDSPBPTK) $(SRC_BPOPT)
	$(CC) -o $@ $^ $(CFLAGS)

libdspbptk.dll: $(SRC_LIBDSPBPTK)
	$(CC) -o $@ $^ $(CFLAGS) -shared -fpic

lib/Turbo-Base64/libtb64.a: lib/Turbo-Base64
	cd lib/Turbo-Base64 && make libtb64.a

clear:
	rm bpopt* libdspbptk*