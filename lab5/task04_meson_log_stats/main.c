#include "log_stats.h"
#include <stdio.h>

int main(int argc, char *argv[]) {
    LogStats stats;
    const char *path = "app.log";

    if (argc > 1) {
        path = argv[1];
    }

    if (analyze_log_file(path, &stats) != 0) {
        printf("Cannot read log file: %s\n", path);
        return 1;
    }

    printf("INFO: %d\n", stats.info_count);
    printf("WARN: %d\n", stats.warn_count);
    printf("ERROR: %d\n", stats.error_count);
    return 0;
}
