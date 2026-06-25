#include "matrix_stats.h"
#include <stdio.h>

int main(int argc, char *argv[]) {
    int m[2][2];
    const char *path = "matrix.txt";

    if (argc > 1) {
        path = argv[1];
    }

    if (read_matrix_2x2(path, m) != 0) {
        printf("Cannot read matrix file: %s\n", path);
        return 1;
    }

    printf("Sum: %d\n", matrix_sum_2x2(m));
    printf("Trace: %d\n", matrix_trace_2x2(m));
    printf("Det: %d\n", matrix_det_2x2(m));
    return 0;
}
