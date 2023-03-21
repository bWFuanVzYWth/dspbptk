#define BUFFER 8192

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

static void run(const char* cmd, char **data){
    FILE* pipe = NULL;
    char buffer[BUFFER];
    int dist=0;
    *data = 0;
    int size;
    pipe = popen(cmd, "r");
    if (pipe) {
      while(!feof(pipe)) {
          size=(int)fread(buffer,1,BUFFER, pipe);
          *data = realloc(*data,(dist + size + 1) * sizeof(char*));
          memcpy((*data)+dist,buffer,size);
          dist+=size;
      }
      (*data)[dist]='\0';
      pclose(pipe);
    }
}

static char* AddStrings(const char* str1, const char* str2) {
  int a,b;
  char* result;
  for(a=0;str1[a]!='\0';a++) {}
  for(b=0;str2[b]!='\0';b++) {}
  result = (char*)malloc((a+b) + 1);
  if (!result) exit(-1); /* Allocation failed. */
  strcpy(result, str1);
  strcat(result, str2);
  return result;
}

static char StringsEqual(const char* str1, const char* str2) {
  return strcmp(str1, str2) == 0;
}

int main(int argc, char* argv[]) {
  int i, j, k;
  const char* commandline = 0;
  const char* file = 0;
  char* outcbs = 0;
  int export0 = 1;
  int export1 = 1;
  int export2 = 1;
  int docbs = 0;
  char lastblock[1] = {'3'};
  unsigned int outcbssize = 0;
  char *data = NULL;
  fprintf(stderr,"DefDB Parser v1.02 by Mr_KrzYch00\n\n");
  for (i = 1; i < argc; ++i) {
    const char* arg = argv[i];
    if(arg[0]=='-' && arg[1]=='-' && arg[2]=='b' && arg[3]=='=' && arg[4]!='\0') {
      export0=0;
      export1=0;
      export2=0;
      for(j=4;arg[j]!='\0';j++) {
        switch(arg[j]) {
          case '0':export0=1;break;
          case '1':export1=1;break;
          case '2':export2=1;break;
        }
      }
    }
  }

 for (i = 1; i < argc; i++) {
   if (argv[i][0] != '-') {
     file = argv[i];
   }
 }
 if(!file) {
   fprintf(stderr,"Reads defdb output and parses it to be used with Zopfli KrzYmod's\n"
   "custom block split points command.\n\n"
   "Arguments:\n"
   "--b=#       Export info for # block types only (0-2, d: 012)\n"
   "The output can be safely piped to file or command line\n\n"
   "Example:\n"
   "%s filename.gz --b=0 >filename.cbs\n", argv[0]);
 } else {
   commandline = AddStrings("defdb ",file);
   commandline = AddStrings(commandline," 2>&1");
   run(commandline,&data);
   if(data!=NULL) {
     for(i=0;data[i]!='\0';i++) {
       if(data[i]=='\n') {
         j=i+1;
         switch(data[j]) {
           case '0':
            if(export0==1 && data[j+1]==' ') docbs=1;
            if(export0==0 && lastblock[0] == '1') {data[j]='2'; docbs=1;}
            break;
           case '1':
            if(export1==1 && data[j+1]==' ') docbs=1;
            if(export1==0 && lastblock[0] == '0') {data[j]='2'; docbs=1;}
            break;
           case '2':
            if(export2==1 && data[j+1]==' ') docbs=1;
            if(export2==0 && (lastblock[0] == '0' || lastblock[0] == '1')) docbs=1;
            break;
         }
         if(docbs==1 || lastblock[0] == '3') {
           for(k=j+1;data[k]==' ';k++) {}
           while(data[k]!=' ') {
             outcbs = realloc(outcbs,(++outcbssize) * sizeof(char*));
             outcbs[outcbssize-1]=data[k];
             ++k;
           }
           outcbs = realloc(outcbs,(outcbssize + 1) * sizeof(char*));
           outcbs[outcbssize++]=',';
           lastblock[0]=data[j];
           i=k;
         }
         docbs=0;
       }
     }
     fprintf(stderr,"========================\nDEFDB OUTPUT [stderr]:\n========================\n");
     fprintf(stderr,"%s",data);
     fprintf(stderr,"========================\nCBS OUTPUT [stdout]:\n========================\n");
     if(outcbssize>0) {
       outcbs[--outcbssize]='\0';
       fprintf(stdout,"%s",outcbs);
       if(export2==0) {
         fprintf(stderr,"\n\n"
         "Please note that when block types 2 are not selected each block\n"
         "type 0 and 1 will be followed by block type 2 to mark correct\n"
         "boundaries of mentioned blocks.\n");
       }
     } else {
       fprintf(stderr,"Error: No blocks found in defdb output.\n");
       return EXIT_FAILURE;
     }
   } else {
     fprintf(stderr,"Unknown error.\n");
     return EXIT_FAILURE;
   }
 }
 return 0;
}
