#include "students.h"
#include <stdio.h>

int main(int argc, char *argv[]) {
    Student students[100];
    const char *path = "students.txt";
    int count;
    int best;

    if (argc > 1) {
        path = argv[1];
    }

    count = load_students(path, students, 100);
    if (count <= 0) {
        printf("Cannot read students file: %s\n", path);
        return 1;
    }

    best = find_best_student(students, count);
    printf("Count: %d\n", count);
    printf("Best: %s (avg=%.2f)\n", students[best].name, students[best].avg);

    return 0;
}
