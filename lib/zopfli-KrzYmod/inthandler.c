#include "defines.h"
#include <stdlib.h>
#include <stdio.h>
#include "inthandler.h"

unsigned int mui;

void intHandler(int exit_code) {
  if(exit_code==2) {
    if(mui == 1) exit(EXIT_FAILURE);
    fprintf(stderr,"                                                              \n"
                   " (!!) CTRL+C detected! Setting --mui to 1 to finish work ASAP!\n"
                   " (!!) Restore points won't be saved from now on!              \n"
                   " (!!) Press it again to abort work.\n");
    mui=1;
  }
}
