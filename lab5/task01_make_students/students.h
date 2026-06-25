#ifndef STUDENTS_H
#define STUDENTS_H

typedef struct {
    char name[64];
    int score1;
    int score2;
    int score3;
    double avg;
} Student;

int load_students(const char *filename, Student arr[], int max_count);
int find_best_student(const Student arr[], int count);

#endif
