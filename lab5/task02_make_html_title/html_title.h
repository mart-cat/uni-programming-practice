#ifndef HTML_TITLE_H
#define HTML_TITLE_H

#include <stddef.h>

int load_text_file(const char *filename, char *buffer, size_t buffer_size);
int extract_title(const char *html, char *title, size_t title_size);

#endif
