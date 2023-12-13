ifeq ($(OS),Windows_NT)
	SHLIB_SUFFIX = .dll
else
	UNAME_S := $(shell uname -s)
	ifeq ($(UNAME_S),Darwin)
		SHLIB_SUFFIX = .dylib
	else
		SHLIB_SUFFIX = .so
	endif
endif

CC := gcc
OBJ_bpopt := app/bpopt.o
OBJ_LIBDSPBPTK = $(patsubst %.c, %.o, $(wildcard lib/*.c lib/libdeflate/lib/*.c lib/libdeflate/lib/*/*.c))
TB64_TARGET = libtb64.a
TB64_PATH = lib/Turbo-Base64
TB64_LIB = $(TB64_PATH)/$(TB64_TARGET)

CFLAGS := -fexec-charset=GBK -Wall -O3 -pipe -static -march=x86-64 -mtune=generic -mavx2 -flto
#CFLAGS += -g -fsanitize=address -fno-omit-frame-pointer
CFLAGS_APP := -Ilib

APPS = bpopt

.PHONY: clean

.SECONDEXPANSION:
$(APPS): $$(OBJ_$$@) libdspbptk.a $(TB64_LIB)
	$(CC) -o $@ $(CFLAGS) $(CFLAGS_APP) $^

$(OBJ_bpopt): %.o: %.c
	$(CC) -c -o $@ $(CFLAGS) $(CFLAGS_APP) $<

$(OBJ_LIBDSPBPTK): %.o: %.c
	$(CC) -c -o $@ $(CFLAGS) $<

libdspbptk.a: $(OBJ_LIBDSPBPTK)
	$(AR) -rc $@ $^

libdspbptk$(SHLIB_SUFFIX): $(OBJ_LIBDSPBPTK) $(TB64_LIB)
	$(CC) -o $@ $(CFLAGS) -shared -fpic $^ $(TB64_LIB)

$(TB64_LIB): $(TB64_PATH)
	+ $(MAKE) -C $^ $(TB64_TARGET)

clean:
	rm -f $(TB64_LIB) $(TB64_PATH)/*.o $(OBJ_LIBDSPBPTK) bpopt* libdspbptk.a libdspbptk$(SHLIB_SUFFIX) app/*.o
