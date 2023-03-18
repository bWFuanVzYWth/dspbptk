CC := gcc
LIBSRC := lib/*.c lib/libdeflate/lib/*.c lib/libdeflate/lib/*/*.c lib/Turbo-Base64/*.c lib/zopfli/*.c lib/yyjson/src/*.c

test: app/test.c
	$(CC) $(LIBSRC) app/test.c -o test -static -fexec-charset=GBK -Ofast -flto -pipe -march=native -mtune=native -fopenmp -DDSPBPTK_DEBUG

clear:
	rm dspbptk.exe test.exe