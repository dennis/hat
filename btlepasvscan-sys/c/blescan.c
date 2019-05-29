#include <signal.h>

#include "btlepasvscan.h"
#include <errno.h>

static int stop_scan = 0;

void sigint_handler(int sig) {
  stop_scan = 1;
}

int main(int argc, char **argv) {
  signal(SIGINT, sigint_handler);

  btlepasvscan_ctx* ctx = btlepasvscan_open();

  if(ctx) {
    puts("Got context");

    if(ctx->error == BTLEPASVSCAN_OK) {
      while(!stop_scan && btlepasvscan_read(ctx)) {
        printf("* %s [ ", ctx->address);
        for (int i = 0; i < ctx->length; i++) {
          printf("0x%02X ", ctx->data[i]);
        }
        puts("]");
      }
    }
    else {
      printf("btlepasvscan-error: %d, errno: %d\n", ctx->error, errno);
      perror("btlepasvscan");
    }
    btlepasvscan_close(ctx);
    ctx = NULL;
    return 0;
  }
  else {
    return 1;
  }
}
