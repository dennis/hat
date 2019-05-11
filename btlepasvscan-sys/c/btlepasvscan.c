// https://github.com/IanHarvey/bluepy/blob/53ce2f2388a936663b94f00636cc2e3677325182/bluepy/bluepy-helper.c
// http://tvaira.free.fr/flower-power/ble-scan.c
// https://github.com/greatscottgadgets/ubertooth/wiki/One-minute-to-understand-BLE-advertising-data-package
// https://github.com/jdleesmiller/bluetrax/blob/master/bluetrax_scan.c
#include <sys/types.h>
#include <sys/socket.h>
#include <stdlib.h>
#include <unistd.h>
#include <netdb.h>
#include <assert.h>
#include <bluetooth/bluetooth.h>
#include <bluetooth/hci.h>
#include <bluetooth/hci_lib.h>

#include "btlepasvscan.h"

const uint8_t BTLEPASVSCAN_ERR_RECV = 1;
const uint8_t BTLEPASVSCAN_ERR_BAD_DATA = 2;
const uint8_t BTLEPASVSCAN_ERR_SETSOCKOPT = 3;
const uint8_t BTLEPASVSCAN_ERR_BIND = 4;
const uint8_t BTLEPASVSCAN_ERR_HCI_LE_SET_SCAN_PARAMETERS = 5;
const uint8_t BTLEPASVSCAN_ERR_HCI_LE_SET_SCAN_ENABLE = 6;
const uint8_t BTLEPASVSCAN_ERR_HCI_LE_SET_SCAN_DISABLE = 7;

static int setup_hci_filter(btlepasvscan_ctx* ctx, int sock) {
  struct hci_filter filter;

  hci_filter_clear(&filter);
  hci_filter_all_ptypes(&filter);
  hci_filter_all_events(&filter);

  int retval = setsockopt(sock, SOL_HCI, HCI_FILTER, &filter, sizeof(filter));
  if(-1 == retval) {
    ctx->error = BTLEPASVSCAN_ERR_SETSOCKOPT;
  }

  return -1 != retval;
}

static int bind_socket(btlepasvscan_ctx* ctx, int sock) {
  struct sockaddr_hci addr;

  memset(&addr, 0, sizeof(addr));
  addr.hci_family = AF_BLUETOOTH;
  addr.hci_dev = 0;

  int retval = bind(sock, (struct sockaddr *)&addr, sizeof(addr));
  if (-1 == retval) {
    ctx->error = BTLEPASVSCAN_ERR_BIND;
  }

  return -1 != retval;
}

static int set_hci_parameters(btlepasvscan_ctx* ctx, int sock) {
  const uint8_t scan_type = 0x00; /* Passive */
  const uint16_t interval = htobs(0x0010);
  const uint16_t window = htobs(0x0010);
  const uint8_t own_type = LE_PUBLIC_ADDRESS;
  const uint8_t filter_policy = 0x00; /* 1 -> Whitelist */

  int retval = hci_le_set_scan_parameters(sock, scan_type, interval, window, own_type, filter_policy, 10000);
  if (retval < 0) {
    ctx->error = BTLEPASVSCAN_ERR_HCI_LE_SET_SCAN_PARAMETERS;
  }

  return (retval >= 0);
}

static int enable_scan(btlepasvscan_ctx* ctx, int sock) {
  int retval = hci_le_set_scan_enable(sock, 1 /* 1 - turn on, 0 - turn off */, 0 /* 0-filtering disabled, 1-filter out duplicates */, 1000  /* timeout */);

  if (retval < 0) {
    ctx->error = BTLEPASVSCAN_ERR_HCI_LE_SET_SCAN_ENABLE;
  }

  return (retval >= 0);
}

static void disable_scan(btlepasvscan_ctx* ctx) {
  int retval = hci_le_set_scan_enable(ctx->sock, 0 /* 1 - turn on, 0 - turn off */, 0 /* 0-filtering disabled, 1-filter out duplicates */, 1000  /* timeout */);

  if (retval < 0) {
    ctx->error = BTLEPASVSCAN_ERR_HCI_LE_SET_SCAN_DISABLE;
  }
}

btlepasvscan_ctx* btlepasvscan_open() {
  int sock = socket(AF_BLUETOOTH, SOCK_RAW, BTPROTO_HCI);

  if(sock != -1) {
    btlepasvscan_ctx* ctx = malloc(sizeof(btlepasvscan_ctx));

    if(setup_hci_filter(ctx, sock) && bind_socket(ctx, sock) && set_hci_parameters(ctx, sock) && enable_scan(ctx, sock)) {
      ctx->sock = sock;

      return ctx;
    }
    else {
      free(ctx);
    }
  }

  return NULL;
}

int btlepasvscan_read(btlepasvscan_ctx* ctx) {
  while(!0) {
    memset(ctx->buf, 0, sizeof(ctx->buf));
    int retval = recv(ctx->sock, ctx->buf, sizeof(ctx->buf), 0);
    int len;

    if(-1 == retval) {
      ctx->error = BTLEPASVSCAN_ERR_RECV;
      return 0;
    }

    switch (ctx->buf[1])  {
      case EVT_CMD_STATUS: // 0x0F
        if (ctx->buf[3]) { // Error
          ctx->error = BTLEPASVSCAN_ERR_BAD_DATA;
          return 0;
        }
        break;

      case EVT_INQUIRY_COMPLETE: // 0x01
        return 0;
        break;

      case EVT_LE_META_EVENT: // 0x3E
        len = retval;
        evt_le_meta_event *meta = (void *)(ctx->buf + (1 + HCI_EVENT_HDR_SIZE));

        len -= (1 + HCI_EVENT_HDR_SIZE);

        if (meta->subevent == EVT_LE_ADVERTISING_REPORT) {
          le_advertising_info *info = (le_advertising_info *) (meta->data + 1);

          ba2str(&info->bdaddr, ctx->address);

          ctx->data   = info->data;
          ctx->length = info->length;

          return 1;
        }
        break;
    }
  }

  return 0;
}

void btlepasvscan_close(btlepasvscan_ctx* ctx) {
  assert(ctx);
  assert(ctx->sock != -1);

  disable_scan(ctx);
  close(ctx->sock);
  free(ctx);
}
