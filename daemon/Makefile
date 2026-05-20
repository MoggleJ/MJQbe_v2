CC = gcc
CFLAGS = -Wall -Wextra -O2
TARGET = mjqbe-daemon
SRCS = main.c

$(TARGET): $(SRCS)
	$(CC) $(CFLAGS) -o $(TARGET) $(SRCS)

clean:
	rm -f $(TARGET)

.PHONY: clean
