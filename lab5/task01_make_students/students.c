#include "students.h"
#include <stdio.h>

int load_students(const char *filename, Student arr[], int max_count) {
    FILE *f = fopen(filename, "r");
    int count = 0;
    if (!f) {
        return -1;
    }
    while (count < max_count) {
        Student s;
        int n = fscanf(f, " %63[^,],%d,%d,%d", s.name, &s.score1, &s.score2, &s.score3);
        if (n != 4) {
            break;
        }
        s.avg = (s.score1 + s.score2 + s.score3) / 3.0;
        arr[count] = s;
        count++;
    }
    fclose(f);
    return count;
}

int find_best_student(const Student arr[], int count) {
    int best = 0;
    int i;
    for (i = 1; i < count; i++) {
        if (arr[i].avg > arr[best].avg) {
            best = i;
        }
    }
    return best;
}
