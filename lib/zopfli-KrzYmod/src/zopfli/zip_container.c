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
#include "util.h"
#include "zip_container.h"
#include "crc32.h"
#include "deflate.h"

/*
Compresses the data according to the zip specification.
*/

void ZopfliZipCompress(const ZopfliOptions* options,
                        const unsigned char* in, size_t insize,
                        unsigned char** out, size_t* outsize,
                        ZopfliPredefinedSplits* sp,
                        const ZopfliAdditionalData* moredata) {

  static const unsigned char filePKh[10]     = { 80, 75,  3,  4, 20,  0,  2,  0,  8,  0};
  static const unsigned char CDIRPKh[12]     = { 80, 75,  1,  2, 20,  0, 20,  0,  2,  0,  8,  0};
  static const unsigned char CDIRPKs[12]     = {  0,  0,  0,  0,  0,  0,  0,  0, 32,  0,  0,  0};
  static const unsigned char EndCDIRPKh[12]  = { 80, 75,  5,  6,  0,  0,  0,  0,  1,  0,  1,  0};
  static const unsigned long defTimestamp    = 50;

  unsigned long crcvalue = CRC(in, insize);
  unsigned long i;
  char* tempfilename = NULL;
  const char* infilename = NULL;
  unsigned long fullsize = insize & 0xFFFFFFFFUL;
  unsigned long rawdeflsize = 0;
  unsigned char bp = 0;
  size_t max = 0;
  unsigned long cdirsize = 0;
  unsigned long cdiroffset;
  if(moredata==NULL) {
    tempfilename = Zmalloc(9 * sizeof(char*));
    sprintf(tempfilename,"%08lx",crcvalue & 0xFFFFFFFFUL);
    infilename = tempfilename;
  } else {
    infilename = moredata->filename;
  }

 /* File PK STATIC DATA + CM */

  for(i=0;i<sizeof(filePKh);++i) ZOPFLI_APPEND_DATA(filePKh[i],out,outsize);

 /* MS-DOS TIME */
  if(moredata == NULL) {
    for(i=0;i<sizeof(defTimestamp);++i) ZOPFLI_APPEND_DATA(defTimestamp >> (i*8) % 256, out, outsize);
  } else {
    for(i=0;i<4;++i) ZOPFLI_APPEND_DATA((moredata->timestamp >> (i*8)) % 256, out, outsize);
  }

 /* CRC */
  for(i=0;i<4;++i) ZOPFLI_APPEND_DATA((crcvalue >> (i*8)) % 256, out, outsize);

 /* OSIZE NOT KNOWN YET - WILL UPDATE AFTER COMPRESSION */
  for(i=0;i<4;++i) ZOPFLI_APPEND_DATA(0, out, outsize);

 /* ISIZE */
  if(fullsize<insize) fullsize=insize;
  for(i=0;i<4;++i) ZOPFLI_APPEND_DATA((fullsize >> (i*8)) % 256, out, outsize);

 /* FNLENGTH */
  for(max=0;infilename[max] != '\0';max++) {}
  for(i=0;i<2;++i) ZOPFLI_APPEND_DATA((max >> (i*8)) % 256, out, outsize);

 /* NO EXTRA FLAGS */
  for(i=0;i<2;++i) ZOPFLI_APPEND_DATA(0, out, outsize);

 /* FILENAME */
  for(i=0;i<max;++i) ZOPFLI_APPEND_DATA(infilename[i], out, outsize);
  rawdeflsize = *outsize;

  if(fullsize<insize) fullsize=insize;
  ZopfliDeflate(options, 2 /* Dynamic block */, 1,
                in, insize, &bp, out, outsize, sp);

  rawdeflsize = *outsize - rawdeflsize;

 /* C-DIR PK HEADER STATIC DATA */
  cdirsize = *outsize;
  for(i=0;i<sizeof(CDIRPKh);++i) ZOPFLI_APPEND_DATA(CDIRPKh[i],out,outsize);

 /* MS-DOS TIME, CRC, OSIZE, ISIZE FROM */

  if(moredata == NULL) {
    for(i=0;i<sizeof(defTimestamp);++i) ZOPFLI_APPEND_DATA(defTimestamp >> (i*8) % 256, out, outsize);
  } else {
    for(i=0;i<4;++i) ZOPFLI_APPEND_DATA((moredata->timestamp >> (i*8)) % 256,out,outsize);
  }
 /* CRC */
  for(i=0;i<4;++i) ZOPFLI_APPEND_DATA((crcvalue >> (i*8)) % 256,out,outsize);

 /* OSIZE + UPDATE IN PK HEADER */
  for(i=0;i<4;++i) ZOPFLI_APPEND_DATA((rawdeflsize >> (i*8)) % 256,out,outsize);
  for(i=0;i<4;++i) (*out)[18+i]=(rawdeflsize >> (i*8)) % 256;

 /* ISIZE */
  for(i=0;i<25;i+=8) ZOPFLI_APPEND_DATA((fullsize >> i) % 256,out,outsize);

 /* FILENAME LENGTH */
  for(max=0;infilename[max] != '\0';max++) {}
  for(i=0;i<2;++i) ZOPFLI_APPEND_DATA((max >> (i*8)) % 256,out,outsize);

 /* C-DIR STATIC DATA */
  for(i=0;i<sizeof(CDIRPKs);++i) ZOPFLI_APPEND_DATA(CDIRPKs[i],out,outsize);

 /* FilePK offset in ZIP file */
  for(i=0;i<4;++i) ZOPFLI_APPEND_DATA(0,out,outsize);
  cdiroffset=(unsigned long)(rawdeflsize+30+max);

 /* FILENAME */
  for(i=0; i<max;++i) ZOPFLI_APPEND_DATA(infilename[i],out,outsize);
  free(tempfilename);
  cdirsize = *outsize - cdirsize;

 /* END C-DIR PK STATIC DATA + TOTAL FILES (ALWAYS 1) */
  for(i=0;i<sizeof(EndCDIRPKh);++i) ZOPFLI_APPEND_DATA(EndCDIRPKh[i],out,outsize);

 /* C-DIR SIZE */
  for(i=0;i<4;++i) ZOPFLI_APPEND_DATA((cdirsize >> (i*8)) % 256,out, outsize);

 /* C-DIR OFFSET */
  for(i=0;i<4;++i) ZOPFLI_APPEND_DATA((cdiroffset >> (i*8)) % 256,out, outsize);

 /* NO COMMENTS IN END C-DIR */
  for(i=0;i<2;++i) ZOPFLI_APPEND_DATA(0, out, outsize);

  if (options->verbose>1) {
    max=(cdiroffset+cdirsize)+22;
    PrintSummary(fullsize,max,0);
  }

}

#else
  typedef int dummy;
#endif
