CC = gcc
CFLAGS = -std=c11 -Wall -Wextra -pedantic
TARGET = task01_students
SRC = main.c students.c

all: $(TARGET)

$(TARGET): $(SRC)
	$(CC) $(CFLAGS) $(SRC) -o $(TARGET)

run: $(TARGET)
	./$(TARGET) students.txt

clean:
	rm -f $(TARGET)
