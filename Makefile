CC := gcc
MAKE := make

LIBSRC := lib/*.c lib/libdeflate/lib/*.c lib/libdeflate/lib/*/*.c lib/Turbo-Base64/*.c lib/yyjson/*.c

CFLAGS := -static -fexec-charset=GBK -Ofast -flto -pipe -march=native -mtune=native -fsanitize=address

dspbptk.exe: app/dspbptk.c $(LIBSRC)
	$(CC) $^ -o $@ $(CFLAGS)

test.exe: app/test.c $(LIBSRC)
	$(CC) $^ -o $@ $(CFLAGS)

clear:
	rm dspbptk.exe test.exe