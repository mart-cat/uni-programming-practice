#ifndef LOG_STATS_H
#define LOG_STATS_H

typedef struct {
    int info_count;
    int warn_count;
    int error_count;
} LogStats;

int analyze_log_file(const char *filename, LogStats *stats);

#endif
