/*
Copyright 2011 Google Inc. All Rights Reserved.
Copyright 2015 Mr_KrzYch00. All Rights Reserved.

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.

Author: lode.vandevenne@gmail.com (Lode Vandevenne)
Author: jyrki.alakuijala@gmail.com (Jyrki Alakuijala)
*/

/*
Several utilities, including: #defines to try different compression results,
basic deflate specification values and generic program options.
*/

#ifndef ZOPFLI_UTIL_H_
#define ZOPFLI_UTIL_H_

#include <stdlib.h>

/* Minimum and maximum length that can be encoded in deflate. */
#define ZOPFLI_MAX_MATCH 258
#define ZOPFLI_MIN_MATCH 3

/* Number of distinct literal/length and distance symbols in DEFLATE */
#define ZOPFLI_NUM_LL 288
#define ZOPFLI_NUM_D 32

/*
The window size for deflate. Must be a power of two. This should be 32768, the
maximum possible by the deflate spec. Anything less hurts compression more than
speed.
*/
#define ZOPFLI_WINDOW_SIZE 32768

/*
The window mask used to wrap indices into the window. This is why the
window size must be a power of two.
*/
#define ZOPFLI_WINDOW_MASK (ZOPFLI_WINDOW_SIZE - 1)

/*
A block structure of huge, non-smart, blocks to divide the input into, to allow
operating on huge files without exceeding memory, such as the 1GB wiki9 corpus.
The whole compression algorithm, including the smarter block splitting, will
be executed independently on each huge block.
Dividing into huge blocks hurts compression, but not much relative to the size.
Set this to, for example, 20MB (20971520). Set it to 0 to disable master blocks.
100MB:104857600 bytes
*/
#define ZOPFLI_MASTER_BLOCK_SIZE 104857600

/*
Used to initialize costs for example
*/
#define ZOPFLI_LARGE_FLOAT 1e30

/*
For longest match cache. max 256. Uses huge amounts of memory but makes it
faster. Uses this many times three bytes per single byte of the input data.
This is so because longest match finding has to find the exact distance
that belongs to each length for the best lz77 strategy.
Good values: 7, 8, 9.
*/
#define ZOPFLI_CACHE_LENGTH 8

/*
To limit maximum memory usage by cache, let's use maximum 512MB.
This value may be exceeded when master block is set too high.
*/
#define ZOPFLI_MAX_CACHE_MEMORY 524288000

/*
Realloc buffer used by ZopfliStoreLitLenDist and TraceBackwards to reduce
amount of memory requests sent to system and stabilize memory usage. Assigned
memory will have up to this many bytes overhead. This speeds up Zopfli on
Odroid U3 (and possibly other systems as well) by 2%. Higher values will
increase memory usage slightly and may reduce performance as well.
*/
#define ZOPFLI_REALLOC_BUFFER 16384

/*
limit the max hash chain hits for this hash value. This has an effect only
on files where the hash value is the same very often. On these files, this
gives worse compression (the value should ideally be 32768, which is the
ZOPFLI_WINDOW_SIZE, while zlib uses 4096 even for best level), but makes it
faster on some specific files.
Good value: e.g. 8192.
*/
#define ZOPFLI_MAX_CHAIN_HITS 32768

/*
Whether to use the longest match cache for ZopfliFindLongestMatch. This cache
consumes a lot of memory but speeds it up. No effect on compression size.
*/
#define ZOPFLI_LONGEST_MATCH_CACHE 1

/*
Enable to remember amount of successive identical bytes in the hash chain for
finding longest match
required for ZOPFLI_HASH_SAME_HASH and ZOPFLI_SHORTCUT_LONG_REPETITIONS
This has no effect on the compression result, and enabling it increases speed.
*/
#define ZOPFLI_HASH_SAME 1

/*
Switch to a faster hash based on the info from ZOPFLI_HASH_SAME once the
best length so far is long enough. This is way faster for files with lots of
identical bytes, on which the compressor is otherwise too slow. Regular files
are unaffected or maybe a tiny bit slower.
This has no effect on the compression result, only on speed.
*/
#define ZOPFLI_HASH_SAME_HASH 1

/*
Enable this, to avoid slowness for files which are a repetition of the same
character more than a multiple of ZOPFLI_MAX_MATCH times. This should not affect
the compression result.
Doesn't seem to alter iteration reductions and provides a little speed up by 1% or less.
Tested by copressing lodepng.cpp.
*/
#define ZOPFLI_SHORTCUT_LONG_REPETITIONS 1

/*
Malloc wrapper loop limiter. By default (0) the malloc will retry infinite amount
of times as we still have hope to find enough memory, for example when other threads
finish their work. For libzopfli it will try only once.
Set to anything larger than 0 to limit malloc tries.
*/
#ifdef NLIB
#define ZOPFLI_MALLOC_RETRY 0
#else
#define ZOPFLI_MALLOC_RETRY 1
#endif

/*
Whether to use lazy matching in the greedy LZ77 implementation. This gives a
better result of ZopfliLZ77Greedy, but the effect this has on the optimal LZ77
varies from file to file. Moved, to be used as command line option.

Appends value to dynamically allocated memory, doubling its allocation size
whenever needed.

value: the value to append, type T
data: pointer to the dynamic array to append to, type T**
size: pointer to the size of the array to append to, type size_t*. This is the
size that you consider the array to be, not the internal allocation size.
Precondition: allocated size of data is at least a power of two greater than or
equal than *size.
*/
#ifdef __cplusplus /* C++ cannot assign void* from malloc to *data */
#define ZOPFLI_APPEND_DATA(/* T */ value, /* T** */ data, /* size_t* */ size) {\
  if (!((*size) & ((*size) - 1))) {\
    /*zfloat alloc size if it's a power of two*/\
    void** data_void = reinterpret_cast<void**>(data);\
    *data_void = (*size) == 0 ? Zmalloc(sizeof(**data))\
                              : Zrealloc((*data), (*size) * 2 * sizeof(**data));\
  }\
  (*data)[(*size)] = (value);\
  (*size)++;\
}
#else /* C gives problems with strict-aliasing rules for (void**) cast */
#define ZOPFLI_APPEND_DATA(/* T */ value, /* T** */ data, /* size_t* */ size) {\
  if (!((*size) & ((*size) - 1))) {\
    /*zfloat alloc size if it's a power of two*/\
    (*data) = (*size) == 0 ? Zmalloc(sizeof(**data))\
                           : Zrealloc((*data), (*size) * 2 * sizeof(**data));\
  }\
  (*data)[(*size)] = (value);\
  (*size)++;\
}
#endif

void *Zmalloc(size_t size);

void *Zcalloc(size_t ElemAm, size_t ElemSize);

void *Zrealloc(void* ptr, size_t size);

#endif  /* ZOPFLI_UTIL_H_ */
