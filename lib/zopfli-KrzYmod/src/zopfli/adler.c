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
#include "adler.h"

/* Calculates the adler32 checksum of the data */

static unsigned long adler32(const unsigned char* data, size_t size, unsigned long adler) __attribute__((pure));
static unsigned long adler32(const unsigned char* data, size_t size, unsigned long adler) {
  static const unsigned sums_overflow = 5550;
  unsigned s1 = adler & 0xffff;
  unsigned s2 = (adler >> 16) & 0xffff;

  while (size > 0) {
    size_t amount = size > sums_overflow ? sums_overflow : size;
    size -= amount;
    while (amount > 0) {
      s1 += (*data++);
      s2 += s1;
      amount--;
    }
    s1 %= 65521;
    s2 %= 65521;
  }
  return (s2 << 16) | s1;
}

DLL_PUBLIC void adler32u(const unsigned char* data, size_t size, unsigned long* adler) {
  *adler=adler32(data,size,*adler);
}
