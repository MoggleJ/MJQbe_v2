CC      = gcc
CFLAGS  = -Wall -Wextra -O2
LDLIBS  = -lcjson -lpthread
TARGET  = mjqbe-daemon
SRC     = main.c av.c

$(TARGET): $(SRC) av.h
	$(CC) $(CFLAGS) -o $(TARGET) $(SRC) $(LDLIBS)

clean:
	rm -f $(TARGET)

.PHONY: clean
