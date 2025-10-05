// GENERATED CODE
// https://github.com/uben0/tree-sitter-typst-utils

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

struct unicode_range {
    uint32_t min;
    uint32_t max;
};

static bool unicode_classify(struct unicode_range t[], size_t min, size_t max, uint32_t c) {
    while (max - min > 1) {
        size_t mid = (min + max) / 2;
        if (c < t[mid].min) {
            max = mid;
        }
        else {
            min = mid;
        }
    }
    return t[min].min <= c && c <= t[min].max;
}
