#include <signal.h>

#include "btlepasvscan.h"

static int stop_scan = 0;

void sigint_handler(int sig) {
  stop_scan = 1;
}

int main(int argc, char **argv) {
  signal(SIGINT, sigint_handler); 

  btlepasvscan_ctx* ctx = btlepasvscan_open();

  if(ctx) {
    while(!stop_scan && btlepasvscan_read(ctx)) {
      printf("* %s [ ", ctx->address);
      for (int i = 0; i < ctx->length; i++) {
        printf("0x%02X ", ctx->data[i]);
      }
      puts("]");
    }
    btlepasvscan_close(ctx);
    ctx = NULL;
    return 0;
  }
  else {
    return 1;
  }
}
