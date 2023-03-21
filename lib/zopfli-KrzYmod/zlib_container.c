/*
Copyright 2013 Google Inc. All Rights Reserved.

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
#include "zlib_container.h"
#include "util.h"
#include "adler.h"
#include "deflate.h"


void ZopfliZlibCompress(const ZopfliOptions* options,
                        const unsigned char* in, size_t insize,
                        unsigned char** out, size_t* outsize,
                        ZopfliPredefinedSplits* sp) {
  unsigned long checksum = 1L;
  unsigned cmf = 120;  /* CM 8, CINFO 7. See zlib spec.*/
  unsigned cmfflg;
  unsigned fcheck;
  unsigned char bp=0;

  adler32u(in, insize,&checksum);

  cmfflg = 256 * cmf + 192;
  fcheck = 31 - cmfflg % 31;
  cmfflg += fcheck;
  ZOPFLI_APPEND_DATA(cmfflg / 256, out, outsize);
  ZOPFLI_APPEND_DATA(cmfflg % 256, out, outsize);

  ZopfliDeflate(options, 2 /* dynamic block */, 1,
                in, insize, &bp, out, outsize, sp);

  for(bp=4;bp!=0;--bp) ZOPFLI_APPEND_DATA((checksum >> ((bp-1)*8)) % 256, out, outsize);

  if (options->verbose>1) PrintSummary(insize,*outsize,0);

}

#else
  typedef int dummy;
#endif
