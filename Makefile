CC := gcc
MAKE := make

LIBSRC := lib/*.c lib/libdeflate/lib/*.c lib/libdeflate/lib/*/*.c lib/Turbo-Base64/*.c lib/yyjson/*.c lib/zopfli-KrzYmod/*.c

CFLAGS := -static -fexec-charset=GBK -Ofast -flto -pipe -march=native -mtune=native

dspbptk.exe: app/dspbptk.c $(LIBSRC)
	$(CC) $^ -o $@ $(CFLAGS)

test.exe: app/test.c $(LIBSRC)
	$(CC) $^ -o $@ $(CFLAGS) -DDSPBPTK_DEBUG

clear:
	rm dspbptk.exe test.exe