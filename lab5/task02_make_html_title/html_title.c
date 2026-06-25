#include "html_title.h"
#include <stdio.h>
#include <string.h>

int load_text_file(const char *filename, char *buffer, size_t buffer_size) {
    FILE *f;
    size_t n;

    f = fopen(filename, "r");
    if (!f) {
        return -1;
    }
    n = fread(buffer, 1, buffer_size - 1, f);
    buffer[n] = '\0';
    fclose(f);
    return 0;
}

int extract_title(const char *html, char *title, size_t title_size) {
    const char *open_tag = "<title>";
    const char *close_tag = "</title>";
    const char *start;
    const char *end;
    size_t len;

    start = strstr(html, open_tag);
    if (!start) {
        return -1;
    }
    start += strlen(open_tag);

    end = strstr(start, close_tag);
    if (!end) {
        return -1;
    }

    len = (size_t)(end - start);
    if (len >= title_size) {
        len = title_size - 1;
    }
    memcpy(title, start, len);
    title[len] = '\0';
    return 0;
}
