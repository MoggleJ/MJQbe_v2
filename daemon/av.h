/* av.h — AV control for mjqbe-daemon: HDMI-CEC, IR (LIRC), Bluetooth (HC-05). */
#ifndef MJQBE_AV_H
#define MJQBE_AV_H

#include <cjson/cJSON.h>

/* Detect cec-client / LIRC socket / serial device and start listener threads.
   Safe to call once at startup; everything degrades to "unavailable". */
void av_init(void);

/* Run one CEC action: "tv_on" | "tv_off" | "ps4_on" | "ps4_off" | "tv_toggle".
   Returns a cJSON object (caller owns) with {"action","sent":bool} or an error
   field when cec-client is not available. */
cJSON *av_cec(const char *action);

/* Current AV subsystem status (cec/ir/bt availability). Caller owns. */
cJSON *av_status(void);

/* Loaded IR button -> action map. Caller owns. */
cJSON *av_ir_map(void);

/* Simulate a received IR button / BT line (for testing the mapping off-Pi).
   Returns {"name"/"line", "action", "handled":bool}. Caller owns. */
cJSON *av_inject_ir(const char *button);
cJSON *av_inject_bt(const char *line);

#endif
