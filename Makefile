CC := gcc
MAKE := make
LIBSRC := lib/*.c lib/libdeflate/lib/*.c lib/libdeflate/lib/*/*.c lib/Turbo-Base64/*.c lib/yyjson/src/*.c

test: app/test.c lib/zopfli-KrzYmod/libzopfli.a
	$(CC) $(LIBSRC) app/test.c lib/zopfli-KrzYmod/libzopfli.a -o test -static -fexec-charset=GBK -Ofast -flto -pipe -march=native -mtune=native -fopenmp -DDSPBPTK_DEBUG

dspbptk: app/dspbptk.c lib/zopfli-KrzYmod/libzopfli.a
	$(CC) $(LIBSRC) app/dspbptk.c lib/zopfli-KrzYmod/libzopfli.a -o dspbptk -static -fexec-charset=GBK -Ofast -flto -pipe -march=native -mtune=native -fopenmp -DDSPBPTK_DEBUG

lib/zopfli-KrzYmod/libzopfli.a:
	cd lib/zopfli-KrzYmod && make libzopfli.a

clear:
	rm dspbptk.exe test.exe