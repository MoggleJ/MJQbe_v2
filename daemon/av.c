/*
 * av.c — HDMI-CEC + IR (LIRC) + Bluetooth (HC-05) for mjqbe-daemon.
 *
 * Off-Pi / without the tools, every subsystem reports "unavailable" and the
 * daemon still runs. The `*_inject` helpers let the mapping logic be tested
 * without any hardware.
 *
 * CEC   : shells out to `cec-client` (package cec-utils) — no libCEC linkage.
 * IR    : connects to the LIRC socket (LIRC_SOCKET, default /var/run/lirc/lircd),
 *         reads button events, maps them via daemon/ir-map.json.
 * BT    : reads newline commands from a serial device (BT_SERIAL,
 *         default /dev/serial0), e.g. "TV_ON".
 */
#include "av.h"

#include <errno.h>
#include <fcntl.h>
#include <pthread.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/socket.h>
#include <sys/stat.h>
#include <sys/un.h>
#include <termios.h>
#include <unistd.h>

#define IR_MAP_PATH_DEFAULT "/etc/mjqbe/ir-map.json"
#define IR_MAP_PATH_FALLBACK "ir-map.json"

static cJSON *g_ir_map = NULL;      /* { "KEY_POWER": "hub_on", ... } */
static int g_cec_ok = 0;
static int g_ir_ok = 0;
static int g_bt_ok = 0;
static pthread_mutex_t g_lock = PTHREAD_MUTEX_INITIALIZER;

/* forward: implemented in main.c (relay/gpio primitives) */
int daemon_relay_set(int relay, int state);

/* ---- action dispatch -------------------------------------------------- */

static const char *dispatch_action(const char *action) {
    if (!action) return "ignored";

    if (strncmp(action, "tv_", 3) == 0 || strncmp(action, "ps4_", 4) == 0) {
        cJSON *r = av_cec(action);
        int sent = cJSON_IsTrue(cJSON_GetObjectItem(r, "sent"));
        cJSON_Delete(r);
        return sent ? "cec_sent" : "cec_unavailable";
    }
    if (strcmp(action, "hub_on") == 0) {
        return daemon_relay_set(1, 1) == 0 ? "hub_on" : "hub_error";
    }
    if (strcmp(action, "hub_off") == 0) {
        return daemon_relay_set(1, 0) == 0 ? "hub_off" : "hub_error";
    }
    /* navigation etc. — logged for now, wired to real targets later */
    return "noop";
}

/* ---- CEC ------------------------------------------------------------------ */

static int have_cmd(const char *cmd) {
    char probe[128];
    snprintf(probe, sizeof(probe), "command -v %s >/dev/null 2>&1", cmd);
    return system(probe) == 0;
}

cJSON *av_cec(const char *action) {
    cJSON *out = cJSON_CreateObject();
    cJSON_AddStringToObject(out, "action", action ? action : "");

    const char *seq = NULL;
    if (!action) seq = NULL;
    else if (!strcmp(action, "tv_on")) seq = "on 0";
    else if (!strcmp(action, "tv_off")) seq = "standby 0";
    else if (!strcmp(action, "tv_toggle")) seq = "pow 0";
    else if (!strcmp(action, "ps4_on")) seq = "tx 4F:82:10:00"; /* active-source PS4 */
    else if (!strcmp(action, "ps4_off")) seq = "tx 4F:36";      /* standby broadcast */

    if (!seq) {
        cJSON_AddBoolToObject(out, "sent", 0);
        cJSON_AddStringToObject(out, "error", "unknown cec action");
        return out;
    }
    if (!g_cec_ok) {
        cJSON_AddBoolToObject(out, "sent", 0);
        cJSON_AddStringToObject(out, "error", "cec-client unavailable");
        return out;
    }

    /* `timeout` guards against cec-client hanging when no adapter is present. */
    char cmd[256];
    snprintf(cmd, sizeof(cmd),
             "timeout 6 sh -c \"echo '%s' | cec-client -s -d 1\" >/dev/null 2>&1", seq);
    int rc = system(cmd);
    cJSON_AddBoolToObject(out, "sent", rc == 0);
    if (rc != 0) cJSON_AddStringToObject(out, "error", "cec-client failed");
    return out;
}

/* ---- IR (LIRC) ---------------------------------------------------------- */

static void load_ir_map(void) {
    const char *paths[] = {getenv("MJQBE_IR_MAP"), IR_MAP_PATH_DEFAULT, IR_MAP_PATH_FALLBACK};
    for (size_t i = 0; i < sizeof(paths) / sizeof(paths[0]); i++) {
        if (!paths[i]) continue;
        FILE *f = fopen(paths[i], "r");
        if (!f) continue;
        fseek(f, 0, SEEK_END);
        long n = ftell(f);
        fseek(f, 0, SEEK_SET);
        if (n > 0 && n < 65536) {
            char *buf = malloc((size_t)n + 1);
            if (buf && fread(buf, 1, (size_t)n, f) == (size_t)n) {
                buf[n] = '\0';
                g_ir_map = cJSON_Parse(buf);
            }
            free(buf);
        }
        fclose(f);
        if (g_ir_map) return;
    }
    /* built-in default */
    g_ir_map = cJSON_Parse(
        "{\"KEY_POWER\":\"hub_on\",\"KEY_UP\":\"nav_up\",\"KEY_DOWN\":\"nav_down\","
        "\"KEY_LEFT\":\"nav_left\",\"KEY_RIGHT\":\"nav_right\",\"KEY_OK\":\"nav_ok\","
        "\"KEY_HOME\":\"nav_home\"}");
}

static const char *ir_action_for(const char *button) {
    if (!g_ir_map || !button) return NULL;
    const cJSON *v = cJSON_GetObjectItemCaseSensitive(g_ir_map, button);
    return cJSON_IsString(v) ? v->valuestring : NULL;
}

static void *ir_thread(void *arg) {
    (void)arg;
    const char *path = getenv("LIRC_SOCKET");
    if (!path || !*path) path = "/var/run/lirc/lircd";

    int fd = socket(AF_UNIX, SOCK_STREAM, 0);
    struct sockaddr_un addr;
    memset(&addr, 0, sizeof(addr));
    addr.sun_family = AF_UNIX;
    snprintf(addr.sun_path, sizeof(addr.sun_path), "%s", path);
    if (fd < 0 || connect(fd, (struct sockaddr *)&addr, sizeof(addr)) != 0) {
        if (fd >= 0) close(fd);
        return NULL;
    }
    pthread_mutex_lock(&g_lock);
    g_ir_ok = 1;
    pthread_mutex_unlock(&g_lock);

    char buf[512];
    ssize_t n;
    while ((n = read(fd, buf, sizeof(buf) - 1)) > 0) {
        buf[n] = '\0';
        /* LIRC line: "<code> <repeat> <button> <remote>" — act only on repeat 00 */
        char *line = strtok(buf, "\n");
        while (line) {
            char code[64], rep[8], btn[64];
            if (sscanf(line, "%63s %7s %63s", code, rep, btn) == 3 &&
                strcmp(rep, "00") == 0) {
                const char *action = ir_action_for(btn);
                if (action) dispatch_action(action);
            }
            line = strtok(NULL, "\n");
        }
    }
    close(fd);
    pthread_mutex_lock(&g_lock);
    g_ir_ok = 0;
    pthread_mutex_unlock(&g_lock);
    return NULL;
}

/* ---- Bluetooth (HC-05 over UART) -------------------------------------- */

static const char *bt_action_for(const char *line) {
    if (!line) return NULL;
    if (!strcasecmp(line, "TV_ON")) return "tv_on";
    if (!strcasecmp(line, "TV_OFF")) return "tv_off";
    if (!strcasecmp(line, "PS4_ON")) return "ps4_on";
    if (!strcasecmp(line, "PS4_OFF")) return "ps4_off";
    if (!strcasecmp(line, "HUB_ON")) return "hub_on";
    if (!strcasecmp(line, "HUB_OFF")) return "hub_off";
    return NULL;
}

static void *bt_thread(void *arg) {
    (void)arg;
    const char *dev = getenv("BT_SERIAL");
    if (!dev || !*dev) dev = "/dev/serial0";

    int fd = open(dev, O_RDONLY | O_NOCTTY);
    if (fd < 0) return NULL;

    struct termios tio;
    if (tcgetattr(fd, &tio) == 0) {
        cfmakeraw(&tio);
        cfsetispeed(&tio, B9600);
        tcsetattr(fd, TCSANOW, &tio);
    }
    pthread_mutex_lock(&g_lock);
    g_bt_ok = 1;
    pthread_mutex_unlock(&g_lock);

    char line[128];
    size_t used = 0;
    char c;
    while (read(fd, &c, 1) == 1) {
        if (c == '\n' || c == '\r') {
            if (used) {
                line[used] = '\0';
                const char *action = bt_action_for(line);
                if (action) dispatch_action(action);
                used = 0;
            }
        } else if (used < sizeof(line) - 1) {
            line[used++] = c;
        }
    }
    close(fd);
    pthread_mutex_lock(&g_lock);
    g_bt_ok = 0;
    pthread_mutex_unlock(&g_lock);
    return NULL;
}

/* ---- public ---------------------------------------------------------------- */

void av_init(void) {
    load_ir_map();
    g_cec_ok = have_cmd("cec-client");

    pthread_t t;
    if (pthread_create(&t, NULL, ir_thread, NULL) == 0) pthread_detach(t);
    if (pthread_create(&t, NULL, bt_thread, NULL) == 0) pthread_detach(t);
}

cJSON *av_status(void) {
    cJSON *o = cJSON_CreateObject();
    pthread_mutex_lock(&g_lock);
    cJSON_AddBoolToObject(o, "cec", g_cec_ok);
    cJSON_AddBoolToObject(o, "ir", g_ir_ok);
    cJSON_AddBoolToObject(o, "bt", g_bt_ok);
    pthread_mutex_unlock(&g_lock);
    return o;
}

cJSON *av_ir_map(void) {
    return g_ir_map ? cJSON_Duplicate(g_ir_map, 1) : cJSON_CreateObject();
}

cJSON *av_inject_ir(const char *button) {
    const char *action = ir_action_for(button);
    cJSON *o = cJSON_CreateObject();
    cJSON_AddStringToObject(o, "name", button ? button : "");
    cJSON_AddStringToObject(o, "action", action ? action : "");
    cJSON_AddBoolToObject(o, "handled", action != NULL);
    if (action) cJSON_AddStringToObject(o, "result", dispatch_action(action));
    return o;
}

cJSON *av_inject_bt(const char *line) {
    const char *action = bt_action_for(line);
    cJSON *o = cJSON_CreateObject();
    cJSON_AddStringToObject(o, "line", line ? line : "");
    cJSON_AddStringToObject(o, "action", action ? action : "");
    cJSON_AddBoolToObject(o, "handled", action != NULL);
    if (action) cJSON_AddStringToObject(o, "result", dispatch_action(action));
    return o;
}
