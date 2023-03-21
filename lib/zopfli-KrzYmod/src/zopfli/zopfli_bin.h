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

#ifndef ZOPFLI_BIN_H_
#define ZOPFLI_BIN_H_

/*
Options used by BIN only.
*/
typedef struct ZopfliBinOptions {

  /*
  Use scandir to get list of files to compress to ZIP. File will be updated
  on-fly after every file successfully gets compressed. So it should be 
  posible to copy it at any time while holding already successfully compressed files,
  or break the operation with CTRL+C and resume it later by manually getting rid
  of already compressed files in pointed directory.
  */
  int usescandir;

  /*
  Allows to set custom block size. This uses simple block splitting instead
  of zopfli auto guessing.
  */
  unsigned int blocksize;

  /*
  Allows to set custom number of blocks. This uses simple block splitting instead
  of zopfli auto guessing.
  */
  unsigned int numblocks;

  /*
  Custom block start points in hexadecimal format comma separated.
  */
  size_t *custblocksplit;

  /*
  Save block splits to file and exit zopfli
  */
  const char* dumpsplitsfile;

  /*
  Runs zopfli splitting between manual/custom start points
  */
  int additionalautosplits;

} ZopfliBinOptions;

typedef struct ZipCDIR {
  unsigned char* data;
  unsigned long size;
  unsigned long offset;
  unsigned short fileid;
  size_t fullsize;
} ZipCDIR;

#endif  /* ZOPFLI_BIN_H_ */
