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
#include "crc32.h"
#include "util.h"

/* Table of CRCs of all 8-bit messages. */
static unsigned long crc_table[256];

/* Flag: has the table been computed? Initially false. */
static int crc_table_computed = 0;

/* Makes the table for a fast CRC. */
static void MakeCRCTable(void) {
  unsigned long c;
  size_t n, k;
  for (n = 0; n < 256; n++) {
    c = (unsigned long) n;
    for (k = 0; k < 8; k++) {
      if (c & 1) {
        c = 0xedb88320L ^ (c >> 1);
      } else {
        c = c >> 1;
      }
    }
    crc_table[n] = c;
  }
  crc_table_computed = 1;
}


/*
Updates a running crc with the bytes buf[0..len-1] and returns
the updated crc. The crc should be initialized to zero.
*/
static unsigned long UpdateCRC(unsigned long crc,
                               const unsigned char *buf, size_t len) {
  unsigned long c = crc ^ 0xffffffffL;
  size_t n;

  if (!crc_table_computed)
    MakeCRCTable();
  for (n = 0; n < len; n++) {
    c = crc_table[(c ^ buf[n]) & 0xff] ^ (c >> 8);
  }
  return c ^ 0xffffffffL;
}

/* Returns the CRC of the bytes buf[0..len-1]. */
unsigned long CRC(const unsigned char* buf, size_t len) {
  size_t offset = 0;
  size_t thislen;
  unsigned long cur_crc = 0L;
  do {
    thislen = len - offset;
    if(thislen>ZOPFLI_MASTER_BLOCK_SIZE) thislen=ZOPFLI_MASTER_BLOCK_SIZE;
    cur_crc=UpdateCRC(cur_crc, buf+offset, thislen);
    offset+=ZOPFLI_MASTER_BLOCK_SIZE;
  } while(offset<len);
  return cur_crc;
}

DLL_PUBLIC void CRCu(const unsigned char* buf, size_t len, unsigned long* crc) {
  if(crc==NULL) {
    *crc=0L;
  }
    *crc=UpdateCRC(*crc,buf,len);
}
