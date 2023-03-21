/*
Copyright 2013 Google Inc. All Rights Reserved.
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

#ifndef NLIB

#include "defines.h"
#include "gzip_container.h"
#include "util.h"
#include "crc32.h"

#include "deflate.h"

/*
Compresses the data according to the gzip specification.
*/
void ZopfliGzipCompress(const ZopfliOptions* options,
                        const unsigned char* in, size_t insize,
                        unsigned char** out, size_t* outsize,
                        ZopfliPredefinedSplits* sp,
                        const ZopfliAdditionalData* moredata) {

  static const unsigned char headerstart[3]  = {  31, 139,   8 };
  static const unsigned char headerend[2]    = {   2,   3 };
  static const unsigned long defTimestamp = 0;

  unsigned long crcvalue = CRC(in, insize);
  unsigned int i;
  const char* infilename = NULL;
  unsigned char bp=0;
  if(moredata!=NULL) infilename = moredata->filename;

  for(i=0;i<sizeof(headerstart);++i) ZOPFLI_APPEND_DATA(headerstart[i], out, outsize);

  if(infilename==NULL) {
    ZOPFLI_APPEND_DATA(0, out, outsize);  /* FLG */
  } else {
    ZOPFLI_APPEND_DATA(8, out, outsize);  /* FLG */
  }
  /* MTIME */
  if(moredata == NULL) {
    for(i=0;i<sizeof(defTimestamp);++i) ZOPFLI_APPEND_DATA((defTimestamp >> (i*8)) % 256, out, outsize);
  } else {
    for(i=0;i<4;++i) ZOPFLI_APPEND_DATA((moredata->timestamp >> (i*8)) % 256, out, outsize);
  }

  for(i=0;i<sizeof(headerend);++i) ZOPFLI_APPEND_DATA(headerend[i], out, outsize);

  if(infilename!=NULL) {
    for(i=0;infilename[i] != '\0';i++) {
      ZOPFLI_APPEND_DATA(infilename[i], out, outsize);
    }
    ZOPFLI_APPEND_DATA(0, out, outsize);
  }

  ZopfliDeflate(options, 2 /* Dynamic block */, 1,
                in, insize, &bp, out, outsize, sp);

  /* CRC */
  for(i=0;i<4;++i) ZOPFLI_APPEND_DATA((crcvalue >> (i*8)) % 256, out, outsize);

  /* ISIZE */
  for(i=0;i<4;++i) ZOPFLI_APPEND_DATA((insize >> (i*8)) % 256, out, outsize);

  if (options->verbose>1) PrintSummary(insize,*outsize,0);

}

#else
  typedef int dummy;
#endif
