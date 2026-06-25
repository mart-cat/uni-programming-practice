#include "matrix_stats.h"
#include <stdio.h>

int read_matrix_2x2(const char *filename, int m[2][2]) {
    FILE *f;

    f = fopen(filename, "r");
    if (!f) {
        return -1;
    }
    if (fscanf(f, "%d %d %d %d", &m[0][0], &m[0][1], &m[1][0], &m[1][1]) != 4) {
        fclose(f);
        return -1;
    }
    fclose(f);
    return 0;
}

int matrix_sum_2x2(const int m[2][2]) {
    return m[0][0] + m[0][1] + m[1][0] + m[1][1];
}

int matrix_trace_2x2(const int m[2][2]) {
    return m[0][0] + m[1][1];
}

int matrix_det_2x2(const int m[2][2]) {
    return m[0][0] * m[1][1] - m[0][1] * m[1][0];
}
