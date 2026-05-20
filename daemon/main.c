#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <sys/socket.h>
#include <sys/un.h>
#include <signal.h>

#define SOCK_PATH "/run/mjqbe/daemon.sock"
#define BUF_SIZE  4096

static int server_fd = -1;

static void cleanup(int sig) {
    (void)sig;
    if (server_fd >= 0) close(server_fd);
    unlink(SOCK_PATH);
    exit(0);
}

int main(void) {
    struct sockaddr_un addr;
    signal(SIGINT, cleanup);
    signal(SIGTERM, cleanup);

    server_fd = socket(AF_UNIX, SOCK_STREAM, 0);
    if (server_fd < 0) { perror("socket"); return 1; }

    unlink(SOCK_PATH);
    memset(&addr, 0, sizeof(addr));
    addr.sun_family = AF_UNIX;
    strncpy(addr.sun_path, SOCK_PATH, sizeof(addr.sun_path) - 1);

    if (bind(server_fd, (struct sockaddr *)&addr, sizeof(addr)) < 0) { perror("bind"); return 1; }
    if (listen(server_fd, 5) < 0) { perror("listen"); return 1; }

    printf("[daemon] listening on %s\n", SOCK_PATH);
    fflush(stdout);

    for (;;) {
        int client = accept(server_fd, NULL, NULL);
        if (client < 0) continue;
        char buf[BUF_SIZE];
        ssize_t n = read(client, buf, sizeof(buf) - 1);
        if (n > 0) {
            buf[n] = '\0';
            const char *resp = "{\"status\":\"ok\"}\n";
            write(client, resp, strlen(resp));
        }
        close(client);
    }
    return 0;
}
