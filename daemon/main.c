/*
 * mjqbe-daemon — hardware control daemon.
 *
 * Listens on a Unix socket, one JSON object per line:
 *   → {"id":"7","cmd":"gpio_set","pin":23,"value":1}
 *   ← {"id":"7","ok":true,"data":{"pin":23,"value":1}}
 *   ← {"id":"7","ok":false,"error":"..."}
 *
 * GPIO is driven through the sysfs interface (/sys/class/gpio). When that is not
 * available (dev box, container without /sys/class/gpio, or MJQBE_GPIO_STUB=1)
 * the daemon runs in "stub" mode: it validates and echoes requests without
 * touching hardware, so the Rust / Python clients can be developed off-Pi.
 *
 * Sprint 7: gpio_set, gpio_get, relay_set, led_set, ping, info.
 */
#include "av.h"

#include <cjson/cJSON.h>
#include <ctype.h>
#include <errno.h>
#include <fcntl.h>
#include <signal.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/socket.h>
#include <sys/stat.h>
#include <sys/un.h>
#include <unistd.h>

#define SOCK_PATH_DEFAULT "/run/mjqbe/daemon.sock"
#define BUF_SIZE 8192
#define GPIO_SYSFS "/sys/class/gpio"

/* relay index (1-based) -> BCM GPIO pin. Documented in daemon/README.md. */
static const int RELAY_PINS[] = {0, 23, 24, 25, 12}; /* [0] unused */
#define RELAY_COUNT 4

static int g_server_fd = -1;
static char g_sock_path[108] = SOCK_PATH_DEFAULT;
static int g_stub = 0;               /* 1 = no real GPIO */
static int g_relay_active_low = 1;   /* most relay boards are active-low */

static void cleanup(int sig) {
    (void)sig;
    if (g_server_fd >= 0) close(g_server_fd);
    unlink(g_sock_path);
    _exit(0);
}

/* ---- sysfs GPIO ------------------------------------------------------- */

static int write_file(const char *path, const char *value) {
    int fd = open(path, O_WRONLY);
    if (fd < 0) return -1;
    ssize_t n = write(fd, value, strlen(value));
    close(fd);
    return (n < 0) ? -1 : 0;
}

static int gpio_export(int pin) {
    char path[64];
    snprintf(path, sizeof(path), GPIO_SYSFS "/gpio%d", pin);
    struct stat st;
    if (stat(path, &st) == 0) return 0; /* already exported */
    char num[16];
    snprintf(num, sizeof(num), "%d", pin);
    return write_file(GPIO_SYSFS "/export", num);
}

static int gpio_write(int pin, int value) {
    if (g_stub) return 0;
    if (gpio_export(pin) != 0) return -1;
    char path[80];
    snprintf(path, sizeof(path), GPIO_SYSFS "/gpio%d/direction", pin);
    write_file(path, "out");
    snprintf(path, sizeof(path), GPIO_SYSFS "/gpio%d/value", pin);
    return write_file(path, value ? "1" : "0");
}

static int gpio_read(int pin, int *out) {
    if (g_stub) { *out = 0; return 0; }
    if (gpio_export(pin) != 0) return -1;
    char path[80];
    snprintf(path, sizeof(path), GPIO_SYSFS "/gpio%d/value", pin);
    int fd = open(path, O_RDONLY);
    if (fd < 0) return -1;
    char c = '0';
    ssize_t n = read(fd, &c, 1);
    close(fd);
    if (n < 1) return -1;
    *out = (c == '1');
    return 0;
}

/* ---- request handling ---------------------------------------------------- */

static cJSON *ok_data(const char *id, cJSON *data) {
    cJSON *r = cJSON_CreateObject();
    if (id) cJSON_AddStringToObject(r, "id", id);
    cJSON_AddBoolToObject(r, "ok", 1);
    cJSON_AddItemToObject(r, "data", data ? data : cJSON_CreateObject());
    return r;
}

static cJSON *err(const char *id, const char *msg) {
    cJSON *r = cJSON_CreateObject();
    if (id) cJSON_AddStringToObject(r, "id", id);
    cJSON_AddBoolToObject(r, "ok", 0);
    cJSON_AddStringToObject(r, "error", msg);
    return r;
}

static int json_int(const cJSON *o, const char *key, int fallback) {
    const cJSON *v = cJSON_GetObjectItemCaseSensitive(o, key);
    if (cJSON_IsNumber(v)) return v->valueint;
    if (cJSON_IsBool(v)) return cJSON_IsTrue(v) ? 1 : 0;
    return fallback;
}

static int valid_pin(int pin) { return pin >= 0 && pin <= 53; }

/* Relay helper, also called from av.c (IR/BT "hub_on" actions). */
int daemon_relay_set(int relay, int state) {
    if (relay < 1 || relay > RELAY_COUNT) return -1;
    int level = g_relay_active_low ? !state : state;
    return gpio_write(RELAY_PINS[relay], level);
}

/*
 * Real GPIO only when we can be confident we're on a Pi:
 *   - MJQBE_GPIO_STUB=1        -> always stub
 *   - MJQBE_GPIO_FORCE=1       -> always real (advanced / other SBCs)
 *   - otherwise: real iff the device-tree model says "raspberry pi"
 *     ( /sys/class/gpio exists on any Linux kernel, so it is not enough )
 */
static int should_stub(void) {
    if (getenv("MJQBE_GPIO_STUB")) return 1;
    if (getenv("MJQBE_GPIO_FORCE")) return 0;

    struct stat st;
    if (stat(GPIO_SYSFS, &st) != 0) return 1;

    const char *paths[] = {"/proc/device-tree/model",
                           "/sys/firmware/devicetree/base/model"};
    for (size_t i = 0; i < sizeof(paths) / sizeof(paths[0]); i++) {
        FILE *f = fopen(paths[i], "r");
        if (!f) continue;
        char model[256] = {0};
        size_t n = fread(model, 1, sizeof(model) - 1, f);
        fclose(f);
        for (size_t j = 0; j < n; j++) model[j] = (char)tolower((unsigned char)model[j]);
        if (strstr(model, "raspberry pi")) return 0;
    }
    return 1;
}

static cJSON *handle(const cJSON *req) {
    const cJSON *id_item = cJSON_GetObjectItemCaseSensitive(req, "id");
    const char *id = cJSON_IsString(id_item) ? id_item->valuestring : NULL;

    const cJSON *cmd_item = cJSON_GetObjectItemCaseSensitive(req, "cmd");
    if (!cJSON_IsString(cmd_item)) return err(id, "missing cmd");
    const char *cmd = cmd_item->valuestring;

    if (strcmp(cmd, "ping") == 0) {
        cJSON *d = cJSON_CreateObject();
        cJSON_AddBoolToObject(d, "pong", 1);
        return ok_data(id, d);
    }

    if (strcmp(cmd, "info") == 0) {
        cJSON *d = cJSON_CreateObject();
        cJSON_AddStringToObject(d, "backend", g_stub ? "stub" : "sysfs");
        cJSON_AddBoolToObject(d, "pi", !g_stub);
        cJSON_AddNumberToObject(d, "relays", RELAY_COUNT);
        return ok_data(id, d);
    }

    if (strcmp(cmd, "gpio_set") == 0) {
        int pin = json_int(req, "pin", -1);
        int value = json_int(req, "value", -1);
        if (!valid_pin(pin) || (value != 0 && value != 1)) return err(id, "bad pin/value");
        if (gpio_write(pin, value) != 0) return err(id, strerror(errno));
        cJSON *d = cJSON_CreateObject();
        cJSON_AddNumberToObject(d, "pin", pin);
        cJSON_AddNumberToObject(d, "value", value);
        return ok_data(id, d);
    }

    if (strcmp(cmd, "gpio_get") == 0) {
        int pin = json_int(req, "pin", -1);
        if (!valid_pin(pin)) return err(id, "bad pin");
        int value = 0;
        if (gpio_read(pin, &value) != 0) return err(id, strerror(errno));
        cJSON *d = cJSON_CreateObject();
        cJSON_AddNumberToObject(d, "pin", pin);
        cJSON_AddNumberToObject(d, "value", value);
        return ok_data(id, d);
    }

    if (strcmp(cmd, "relay_set") == 0) {
        int relay = json_int(req, "relay", -1);
        int state = json_int(req, "state", -1);
        if (relay < 1 || relay > RELAY_COUNT || (state != 0 && state != 1))
            return err(id, "bad relay/state");
        if (daemon_relay_set(relay, state) != 0) return err(id, strerror(errno));
        cJSON *d = cJSON_CreateObject();
        cJSON_AddNumberToObject(d, "relay", relay);
        cJSON_AddNumberToObject(d, "state", state);
        cJSON_AddNumberToObject(d, "pin", RELAY_PINS[relay]);
        return ok_data(id, d);
    }

    if (strcmp(cmd, "led_set") == 0) {
        /* RGB LED on three GPIOs (overridable via env). Any component > 0 = on. */
        int r = json_int(req, "r", 0), g = json_int(req, "g", 0), b = json_int(req, "b", 0);
        int rp = getenv("MJQBE_LED_R") ? atoi(getenv("MJQBE_LED_R")) : 5;
        int gp = getenv("MJQBE_LED_G") ? atoi(getenv("MJQBE_LED_G")) : 6;
        int bp = getenv("MJQBE_LED_B") ? atoi(getenv("MJQBE_LED_B")) : 13;
        if (gpio_write(rp, r > 0) != 0 || gpio_write(gp, g > 0) != 0 ||
            gpio_write(bp, b > 0) != 0)
            return err(id, strerror(errno));
        cJSON *d = cJSON_CreateObject();
        cJSON_AddNumberToObject(d, "r", r > 0);
        cJSON_AddNumberToObject(d, "g", g > 0);
        cJSON_AddNumberToObject(d, "b", b > 0);
        return ok_data(id, d);
    }

    /* ---- AV: HDMI-CEC / IR / Bluetooth (Sprint 8) ---- */

    if (strcmp(cmd, "cec_send") == 0) {
        const cJSON *a = cJSON_GetObjectItemCaseSensitive(req, "action");
        if (!cJSON_IsString(a)) return err(id, "missing action");
        return ok_data(id, av_cec(a->valuestring));
    }

    if (strcmp(cmd, "av_status") == 0) return ok_data(id, av_status());
    if (strcmp(cmd, "ir_map") == 0) return ok_data(id, av_ir_map());

    if (strcmp(cmd, "ir_inject") == 0) {
        const cJSON *n = cJSON_GetObjectItemCaseSensitive(req, "name");
        if (!cJSON_IsString(n)) return err(id, "missing name");
        return ok_data(id, av_inject_ir(n->valuestring));
    }

    if (strcmp(cmd, "bt_inject") == 0) {
        const cJSON *l = cJSON_GetObjectItemCaseSensitive(req, "line");
        if (!cJSON_IsString(l)) return err(id, "missing line");
        return ok_data(id, av_inject_bt(l->valuestring));
    }

    return err(id, "unknown cmd");
}

/* ---- one client connection: line-delimited JSON ------------------------- */

static void serve_client(int fd) {
    char buf[BUF_SIZE];
    size_t used = 0;

    for (;;) {
        ssize_t n = read(fd, buf + used, sizeof(buf) - used - 1);
        if (n <= 0) break;
        used += (size_t)n;
        buf[used] = '\0';

        char *line_start = buf;
        char *nl;
        while ((nl = memchr(line_start, '\n', (buf + used) - line_start)) != NULL) {
            *nl = '\0';
            if (*line_start) {
                cJSON *req = cJSON_Parse(line_start);
                cJSON *resp = req ? handle(req) : err(NULL, "invalid JSON");
                char *out = cJSON_PrintUnformatted(resp);
                if (out) {
                    dprintf(fd, "%s\n", out);
                    free(out);
                }
                cJSON_Delete(resp);
                cJSON_Delete(req);
            }
            line_start = nl + 1;
        }
        /* keep the partial trailing line */
        size_t rem = (buf + used) - line_start;
        memmove(buf, line_start, rem);
        used = rem;
        if (used >= sizeof(buf) - 1) used = 0; /* overlong line: drop */
    }
}

int main(void) {
    signal(SIGINT, cleanup);
    signal(SIGTERM, cleanup);
    signal(SIGPIPE, SIG_IGN);

    const char *env_sock = getenv("DAEMON_SOCKET");
    if (env_sock && *env_sock) {
        strncpy(g_sock_path, env_sock, sizeof(g_sock_path) - 1);
        g_sock_path[sizeof(g_sock_path) - 1] = '\0';
    }
    if (getenv("MJQBE_RELAY_ACTIVE_HIGH")) g_relay_active_low = 0;

    g_stub = should_stub();

    g_server_fd = socket(AF_UNIX, SOCK_STREAM, 0);
    if (g_server_fd < 0) { perror("socket"); return 1; }

    unlink(g_sock_path);
    struct sockaddr_un addr;
    memset(&addr, 0, sizeof(addr));
    addr.sun_family = AF_UNIX;
    snprintf(addr.sun_path, sizeof(addr.sun_path), "%s", g_sock_path);

    if (bind(g_server_fd, (struct sockaddr *)&addr, sizeof(addr)) < 0) {
        perror("bind");
        return 1;
    }
    chmod(g_sock_path, 0660);
    if (listen(g_server_fd, 8) < 0) { perror("listen"); return 1; }

    av_init();

    printf("[daemon] listening on %s (%s mode)\n", g_sock_path, g_stub ? "stub" : "sysfs");
    fflush(stdout);

    for (;;) {
        int client = accept(g_server_fd, NULL, NULL);
        if (client < 0) {
            if (errno == EINTR) continue;
            break;
        }
        serve_client(client);
        close(client);
    }
    cleanup(0);
    return 0;
}
