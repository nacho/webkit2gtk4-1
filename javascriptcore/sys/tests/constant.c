// Generated by gir (https://github.com/gtk-rs/gir @ 425f84d5af7f)
// from gir-files (https://github.com/gtk-rs/gir-files.git @ 4eaad6a722bf)
// from webkit2gtk-gir-files
// DO NOT EDIT

#include "manual.h"
#include <stdio.h>

#define PRINT_CONSTANT(CONSTANT_NAME) \
    printf("%s;", #CONSTANT_NAME); \
    printf(_Generic((CONSTANT_NAME), \
                    char *: "%s", \
                    const char *: "%s", \
                    char: "%c", \
                    signed char: "%hhd", \
                    unsigned char: "%hhu", \
                    short int: "%hd", \
                    unsigned short int: "%hu", \
                    int: "%d", \
                    unsigned int: "%u", \
                    long: "%ld", \
                    unsigned long: "%lu", \
                    long long: "%lld", \
                    unsigned long long: "%llu", \
                    float: "%f", \
                    double: "%f", \
                    long double: "%ld"), \
           CONSTANT_NAME); \
    printf("\n");

int main() {
    PRINT_CONSTANT((gint) JSC_CHECK_SYNTAX_MODE_MODULE);
    PRINT_CONSTANT((gint) JSC_CHECK_SYNTAX_MODE_SCRIPT);
    PRINT_CONSTANT((gint) JSC_CHECK_SYNTAX_RESULT_IRRECOVERABLE_ERROR);
    PRINT_CONSTANT((gint) JSC_CHECK_SYNTAX_RESULT_OUT_OF_MEMORY_ERROR);
    PRINT_CONSTANT((gint) JSC_CHECK_SYNTAX_RESULT_RECOVERABLE_ERROR);
    PRINT_CONSTANT((gint) JSC_CHECK_SYNTAX_RESULT_STACK_OVERFLOW_ERROR);
    PRINT_CONSTANT((gint) JSC_CHECK_SYNTAX_RESULT_SUCCESS);
    PRINT_CONSTANT((gint) JSC_CHECK_SYNTAX_RESULT_UNTERMINATED_LITERAL_ERROR);
    PRINT_CONSTANT(JSC_MAJOR_VERSION);
    PRINT_CONSTANT(JSC_MICRO_VERSION);
    PRINT_CONSTANT(JSC_MINOR_VERSION);
    PRINT_CONSTANT(JSC_OPTIONS_USE_DFG);
    PRINT_CONSTANT(JSC_OPTIONS_USE_FTL);
    PRINT_CONSTANT(JSC_OPTIONS_USE_JIT);
    PRINT_CONSTANT(JSC_OPTIONS_USE_LLINT);
    PRINT_CONSTANT((gint) JSC_OPTION_BOOLEAN);
    PRINT_CONSTANT((gint) JSC_OPTION_DOUBLE);
    PRINT_CONSTANT((gint) JSC_OPTION_INT);
    PRINT_CONSTANT((gint) JSC_OPTION_RANGE_STRING);
    PRINT_CONSTANT((gint) JSC_OPTION_SIZE);
    PRINT_CONSTANT((gint) JSC_OPTION_STRING);
    PRINT_CONSTANT((gint) JSC_OPTION_UINT);
    PRINT_CONSTANT((gint) JSC_TYPED_ARRAY_FLOAT32);
    PRINT_CONSTANT((gint) JSC_TYPED_ARRAY_FLOAT64);
    PRINT_CONSTANT((gint) JSC_TYPED_ARRAY_INT16);
    PRINT_CONSTANT((gint) JSC_TYPED_ARRAY_INT32);
    PRINT_CONSTANT((gint) JSC_TYPED_ARRAY_INT64);
    PRINT_CONSTANT((gint) JSC_TYPED_ARRAY_INT8);
    PRINT_CONSTANT((gint) JSC_TYPED_ARRAY_NONE);
    PRINT_CONSTANT((gint) JSC_TYPED_ARRAY_UINT16);
    PRINT_CONSTANT((gint) JSC_TYPED_ARRAY_UINT32);
    PRINT_CONSTANT((gint) JSC_TYPED_ARRAY_UINT64);
    PRINT_CONSTANT((gint) JSC_TYPED_ARRAY_UINT8);
    PRINT_CONSTANT((gint) JSC_TYPED_ARRAY_UINT8_CLAMPED);
    PRINT_CONSTANT((guint) JSC_VALUE_PROPERTY_CONFIGURABLE);
    PRINT_CONSTANT((guint) JSC_VALUE_PROPERTY_ENUMERABLE);
    PRINT_CONSTANT((guint) JSC_VALUE_PROPERTY_WRITABLE);
    return 0;
}
