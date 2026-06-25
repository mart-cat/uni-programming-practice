#include "html_title.h"
#include <stdio.h>

int main(int argc, char *argv[]) {
    char html[8192];
    char title[256];
    const char *path = "page.html";

    if (argc > 1) {
        path = argv[1];
    }

    if (load_text_file(path, html, sizeof(html)) != 0) {
        printf("Cannot read html file: %s\n", path);
        return 1;
    }

    if (extract_title(html, title, sizeof(title)) != 0) {
        printf("Tag <title> not found\n");
        return 1;
    }

    printf("Title: %s\n", title);
    return 0;
}
