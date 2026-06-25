CC = gcc
CFLAGS = -std=c11 -Wall -Wextra -pedantic
TARGET = task02_title
SRC = main.c html_title.c

all: $(TARGET)

$(TARGET): $(SRC)
	$(CC) $(CFLAGS) $(SRC) -o $(TARGET)

debug:
	$(CC) $(CFLAGS) -g $(SRC) -o $(TARGET)

clean:
	rm -f $(TARGET)
