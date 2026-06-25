#include "log_stats.h"
#include <stdio.h>
#include <string.h>

int analyze_log_file(const char *filename, LogStats *stats) {
    FILE *f;
    char line[512];

    f = fopen(filename, "r");
    if (!f) {
        return -1;
    }

    stats->info_count = 0;
    stats->warn_count = 0;
    stats->error_count = 0;

    while (fgets(line, sizeof(line), f)) {
        if (strstr(line, "INFO") != NULL) {
            stats->info_count++;
        } else if (strstr(line, "WARN") != NULL) {
            stats->warn_count++;
        } else if (strstr(line, "ERROR") != NULL) {
            stats->error_count++;
        }
    }

    fclose(f);
    return 0;
}
