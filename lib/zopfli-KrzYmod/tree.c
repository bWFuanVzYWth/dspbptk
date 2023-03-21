/*
Copyright 2011 Google Inc. All Rights Reserved.

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

#include "defines.h"
#include "tree.h"

#include <assert.h>
#include <math.h>

#include "katajainen.h"
#include "util.h"

void ZopfliLengthsToSymbols(const unsigned* lengths, size_t n, unsigned maxbits,
                            unsigned* symbols) {
  size_t* bl_count = Zmalloc(sizeof(size_t) * (maxbits + 1));
  size_t* next_code = Zmalloc(sizeof(size_t) * (maxbits + 1));
  unsigned bits, i;
  unsigned code;

  memset(symbols, 0, sizeof(*symbols) * n);

  /* 1) Count the number of codes for each code length. Let bl_count[N] be the
  number of codes of length N, N >= 1. */
  memset(bl_count, 0, sizeof(*bl_count) * (maxbits + 1));
  for (i = 0; i < n; i++) {
    assert(lengths[i] <= maxbits);
    bl_count[lengths[i]]++;
  }
  /* 2) Find the numerical value of the smallest code for each code length. */
  code = 0;
  bl_count[0] = 0;
  for (bits = 1; bits <= maxbits; bits++) {
    code = (code + bl_count[bits-1]) << 1;
    next_code[bits] = code;
  }
  /* 3) Assign numerical values to all codes, using consecutive values for all
  codes of the same length with the base values determined at step 2. */
  for (i = 0;  i < n; i++) {
    unsigned len = lengths[i];
    if (len != 0) {
      symbols[i] = next_code[len];
      next_code[len]++;
    }
  }

  free(next_code);
  free(bl_count);
}

void ZopfliCalculateEntropy(const size_t* count, size_t n, zfloat* bitlengths) {
  size_t sum = 0;
  size_t i;
  zfloat log2sum;
  for (i = 0; i < n; ++i) {
    sum += count[i];
  }
  log2sum = (sum == 0 ? ZLOG(n) : ZLOG(sum)) * ZOPFLI_INVLOG2;
  for (i = 0; i < n; ++i) {
    /* When the count of the symbol is 0, but its cost is requested anyway, it
    means the symbol will appear at least once anyway, so give it the cost as if
    its count is 1.*/
    if (count[i] == 0) bitlengths[i] = log2sum;
    else bitlengths[i] = log2sum - ZLOG(count[i]) * ZOPFLI_INVLOG2;
    /* Depending on compiler and architecture, the above subtraction of two
    floating point numbers may give a negative result very close to zero
    instead of zero (e.g. -5.973954e-17 with gcc 4.1.2 on Ubuntu 11.4). Clamp
    it to zero. These floating point imprecisions do not affect the cost model
    significantly so this is ok. */
    if (bitlengths[i] < 0 && bitlengths[i] > ZOPFLI_CLOSENEGATIVE) bitlengths[i] = 0;
    assert(bitlengths[i] >= 0);
  }
}

void ZopfliCalculateBitLengths(const size_t* count, size_t n, int maxbits,
                               unsigned* bitlengths, int revcounts) {
  int error = ZopfliLengthLimitedCodeLengths(count, n, maxbits, bitlengths, revcounts);
  (void) error;
  assert(!error);
}
