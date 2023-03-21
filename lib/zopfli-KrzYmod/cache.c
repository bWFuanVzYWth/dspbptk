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
#include "cache.h"

#include <assert.h>
#include <stdio.h>
#include <string.h>

#ifdef ZOPFLI_LONGEST_MATCH_CACHE

void ZopfliInitCache(size_t blocksize, ZopfliLongestMatchCache* lmc) {
  size_t i;
  lmc->length = Zmalloc(sizeof(unsigned short) * blocksize);
  lmc->dist = Zmalloc(sizeof(unsigned short) * blocksize);
  /* Rather large amount of memory. */
  lmc->cache_length = ZOPFLI_CACHE_LENGTH;
  while(lmc->cache_length*3*blocksize+blocksize*4>ZOPFLI_MAX_CACHE_MEMORY && lmc->cache_length > 1) {
    --lmc->cache_length;
  }
  lmc->sublen = Zmalloc(lmc->cache_length * 3 * blocksize);

  /* length > 0 and dist 0 is invalid combination, which indicates on purpose
  that this cache value is not filled in yet. */
  for (i = 0; i < blocksize; i++) lmc->length[i] = 1;
  memset(lmc->dist, 0, blocksize * sizeof(lmc->dist[0]));
  memset(lmc->sublen, 0, lmc->cache_length * blocksize * 3 * sizeof(lmc->sublen[0]));
}

void ZopfliCleanCache(ZopfliLongestMatchCache* lmc) {
  free(lmc->sublen);
  free(lmc->dist);
  free(lmc->length);
}

void ZopfliSublenToCache(const unsigned short* sublen,
                         size_t pos, size_t length,
                         ZopfliLongestMatchCache* lmc) {
  size_t i;
  size_t j = 0;
  size_t bestlength = 0;
  unsigned char* cache;

#if ZOPFLI_CACHE_LENGTH == 0
  return;
#endif

  cache = &lmc->sublen[lmc->cache_length * pos * 3];
  if (length < 3) return;
  for (i = 3; i <= length; i++) {
    if (i == length || sublen[i] != sublen[i + 1]) {
      cache[j * 3] = i - 3;
      memcpy(cache + (j * 3) + 1, sublen + i, sizeof(*sublen));
      bestlength = i;
      j++;
      if (j >= lmc->cache_length) break;
    }
  }
  if (j < lmc->cache_length) {
    assert(bestlength == length);
    cache[(lmc->cache_length - 1) * 3] = bestlength - 3;
  } else {
    assert(bestlength <= length);
  }
  assert(bestlength == ZopfliMaxCachedSublen(lmc, pos, length));
}

void ZopfliCacheToSublen(const ZopfliLongestMatchCache* lmc,
                         size_t pos, size_t length,
                         unsigned short* sublen) {
  size_t i, j;
  size_t maxlength = ZopfliMaxCachedSublen(lmc, pos, length);
  size_t prevlength = 0;
  unsigned char* cache;
#if ZOPFLI_CACHE_LENGTH == 0
  return;
#endif
  if (length < 3) return;
  cache = &lmc->sublen[lmc->cache_length * pos * 3];
  for (j = 0; j < lmc->cache_length; j++) {
    unsigned length2 = cache[j * 3] + 3;
    unsigned dist = cache[j * 3 + 1] + 256 * cache[j * 3 + 2];
    for (i = prevlength; i <= length2; i++) {
      sublen[i] = dist;
    }
    if (length2 == maxlength) break;
    prevlength = length2 + 1;
  }
}

/*
Returns the length up to which could be stored in the cache.
*/
size_t ZopfliMaxCachedSublen(const ZopfliLongestMatchCache* lmc,
                               size_t pos, size_t length) {
  unsigned char* cache;
#if ZOPFLI_CACHE_LENGTH == 0
  return 0;
#endif
  cache = &lmc->sublen[lmc->cache_length * pos * 3];
  (void)length;
  if (cache[1] == 0 && cache[2] == 0) return 0;  /* No sublen cached. */
  return cache[(lmc->cache_length - 1) * 3] + 3;
}

#endif  /* ZOPFLI_LONGEST_MATCH_CACHE */
