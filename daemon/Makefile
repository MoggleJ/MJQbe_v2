CC      = gcc
CFLAGS  = -Wall -Wextra -O2
TARGET  = mjqbe-daemon
SRC     = main.c

$(TARGET): $(SRC)
	$(CC) $(CFLAGS) -o $(TARGET) $(SRC) -lcjson

clean:
	rm -f $(TARGET)
