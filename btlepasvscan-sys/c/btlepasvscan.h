#ifndef btlepasvscan_h__
#define btlepasvscan_h__

#include <stdint.h>
#include <bluetooth/bluetooth.h>
#include <bluetooth/hci.h>

#if HCI_MAX_FRAME_SIZE == 1500
#else
#error Unexpected HCI_MAX_FRAME_SIZE size
#endif

const uint8_t BTLEPASVSCAN_OK;
const uint8_t BTLEPASVSCAN_ERR_RECV;
const uint8_t BTLEPASVSCAN_ERR_BAD_DATA;
const uint8_t BTLEPASVSCAN_ERR_SETSOCKOPT;
const uint8_t BTLEPASVSCAN_ERR_BIND;
const uint8_t BTLEPASVSCAN_ERR_HCI_LE_SET_SCAN_PARAMETERS;
const uint8_t BTLEPASVSCAN_ERR_HCI_LE_SET_SCAN_ENABLE;
const uint8_t BTLEPASVSCAN_ERR_HCI_LE_SET_SCAN_DISABLE;
const uint8_t BTLEPASVSCAN_ERR_OPEN;


typedef struct {
  int sock;
  uint8_t buf[HCI_MAX_FRAME_SIZE];
  char address[18];
  uint8_t* data;
  uint32_t length;
  uint8_t error;
  uint8_t scan; // true/false if blescan was activated
} btlepasvscan_ctx;

btlepasvscan_ctx* btlepasvscan_open();
int btlepasvscan_read(btlepasvscan_ctx*);
void btlepasvscan_close(btlepasvscan_ctx*);

#endif
