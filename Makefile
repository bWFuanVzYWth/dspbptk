CC := gcc

LIBSRC := lib/*.c lib/libdeflate/lib/*.c lib/libdeflate/lib/*/*.c lib/Turbo-Base64/*.c
APPSRC := app/dspbptk.c

<<<<<<< HEAD
CFLAGS := -static -fexec-charset=GBK -Ofast -flto -pipe -march=native -mtune=native -fsanitize=address
=======
CFLAGS := -static -s -fexec-charset=GBK -Ofast -flto -march=native -mtune=native -Wall -Wextra
>>>>>>> e10ee1b386ffc00f01ef994c086b96323d8aa0ed

dspbptk: $(LIBSRC) $(APPSRC)
	$(CC) -o $@ $^ $(CFLAGS)

libdspbptk.dll: $(LIBSRC)
	$(CC) -o $@ $^ $(CFLAGS) -shared -fpic

all: dspbptk libdspbptk.dll

clear:
	rm dspbptk* libdspbptk*