#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <sys/socket.h>
#include <sys/un.h>
#include <signal.h>

#define SOCKET_PATH_ENV "DAEMON_SOCKET"
#define SOCKET_PATH_DEFAULT "/run/mjqbe/daemon.sock"
#define BUFFER_SIZE 4096

static volatile int running = 1;

static void handle_signal(int sig) {
    (void)sig;
    running = 0;
}

int main(void) {
    const char *socket_path = getenv(SOCKET_PATH_ENV);
    if (!socket_path)
        socket_path = SOCKET_PATH_DEFAULT;

    signal(SIGINT, handle_signal);
    signal(SIGTERM, handle_signal);

    int server_fd = socket(AF_UNIX, SOCK_STREAM, 0);
    if (server_fd < 0) {
        perror("socket");
        return EXIT_FAILURE;
    }

    struct sockaddr_un addr = {0};
    addr.sun_family = AF_UNIX;
    strncpy(addr.sun_path, socket_path, sizeof(addr.sun_path) - 1);

    unlink(socket_path);
    if (bind(server_fd, (struct sockaddr *)&addr, sizeof(addr)) < 0) {
        perror("bind");
        close(server_fd);
        return EXIT_FAILURE;
    }

    if (listen(server_fd, 5) < 0) {
        perror("listen");
        close(server_fd);
        return EXIT_FAILURE;
    }

    printf("mjqbe-daemon listening on %s\n", socket_path);

    while (running) {
        int client_fd = accept(server_fd, NULL, NULL);
        if (client_fd < 0) {
            if (running)
                perror("accept");
            continue;
        }

        char buf[BUFFER_SIZE] = {0};
        ssize_t n = read(client_fd, buf, sizeof(buf) - 1);
        if (n > 0) {
            printf("Received: %s\n", buf);
            const char *resp = "{\"status\":\"ok\"}\n";
            write(client_fd, resp, strlen(resp));
        }
        close(client_fd);
    }

    close(server_fd);
    unlink(socket_path);
    return EXIT_SUCCESS;
}
