CC := gcc

LIBSRC := lib/*.c lib/libdeflate/lib/*.c lib/libdeflate/lib/*/*.c lib/Turbo-Base64/*.c lib/yyjson/*.c
APPSRC := app/dspbptk.c

CFLAGS := -static -s -fexec-charset=GBK -Ofast -flto -march=native -mtune=native -Wall

dspbptk: $(LIBSRC) $(APPSRC)
	$(CC) -o $@ $^ $(CFLAGS)

libdspbptk.dll: $(LIBSRC)
	$(CC) -o $@ $^ $(CFLAGS) -shared -fpic

all: dspbptk libdspbptk.dll

clear:
	rm dspbptk* libdspbptk*