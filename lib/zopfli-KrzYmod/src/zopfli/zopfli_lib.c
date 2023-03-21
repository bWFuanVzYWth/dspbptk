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

#ifndef NLIB

#include "defines.h"
#include "zopfli.h"
#include "deflate.h"
#include "gzip_container.h"
#include "zip_container.h"
#include "zlib_container.h"
#include "inthandler.h"
#include <stdio.h>

void intHandler(int exit_code);

/* You can use this function in Your own lib calls/applications.
   ZopfliFormat is required but can be passed as simple number.
   ZopfliOptions, ZopfliPredefinedSplits and ZopfliAdditionalData
   structures are optional and can be NULL.

   ZopfliPredefinedSplits structure holds 3 variables being
   an array of block split offsets, number of block split offsets
   and if to perform additional auto splitting between predefined
   split points (0 or 1). size_t array, size_t and int.

   ZopfliAdditonalData structure can hold 2 variables being
   file timestamp and name. If timestamp is omitted, lower possible
   value will be used for GZ and ZIP. If file name is omitted
   GZ will not store filename and ZIP will use input data CRC32 as
   file name.
*/
DLL_PUBLIC void ZopfliCompress(ZopfliOptions* options, const ZopfliFormat output_type,
                    const unsigned char* in, size_t insize,
                    unsigned char** out, size_t* outsize, ZopfliPredefinedSplits* sp,
                    const ZopfliAdditionalData* moredata) {
  if(in == NULL || out == NULL || outsize == NULL) {
    fprintf(stderr,"Critical Error: one or more required pointers are NULL\n");
    exit(EXIT_FAILURE);
  } else {
    ZopfliOptions* optionslib = Zmalloc(sizeof(*optionslib));
    if(options == NULL) {
      ZopfliInitOptions(optionslib);
      optionslib->verbose = 0;
    } else {
      optionslib = options;
      mui = options->maxfailiterations;
    }
    if (output_type == ZOPFLI_FORMAT_GZIP || output_type == ZOPFLI_FORMAT_GZIP_NAME) {
      ZopfliGzipCompress(optionslib, in, insize, out, outsize, sp, moredata);
    } else if (output_type == ZOPFLI_FORMAT_ZLIB) {
      ZopfliZlibCompress(optionslib, in, insize, out, outsize, sp);
    } else if (output_type == ZOPFLI_FORMAT_ZIP) {
      ZopfliZipCompress(optionslib, in, insize, out, outsize, sp, moredata);
    } else if (output_type == ZOPFLI_FORMAT_DEFLATE) {
      unsigned char bp = 0;
      ZopfliDeflate(optionslib, 2 /* Dynamic block */, 1,
                    in, insize, &bp, out, outsize, sp);
    } else {
      fprintf(stderr,"Error: No output format specified.\n");
      exit (EXIT_FAILURE);
    }
    free(optionslib);
  }
}
#else
  typedef int dummy;
#endif
