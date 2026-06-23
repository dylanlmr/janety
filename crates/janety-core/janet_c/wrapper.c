#include "janet.h"
#include <string.h>

static char result_buffer[8192];
static char print_buffer[65536];

const char* evaluate_janet_code(const char* code, const char* source_name, int* out_status, const char** out_prints) {
    janet_init();
    JanetTable *env = janet_core_env(NULL);
    
    JanetBuffer *console_buf = janet_buffer(0);
    
    janet_table_put(env, janet_ckeywordv("out"), janet_wrap_buffer(console_buf));
    janet_table_put(env, janet_ckeywordv("err"), janet_wrap_buffer(console_buf));

    Janet ret;
    *out_status = janet_dostring(env, code, source_name, &ret);

    size_t len = console_buf->count;
    if (len >= sizeof(print_buffer)) len = sizeof(print_buffer) - 1;
    memcpy(print_buffer, console_buf->data, len);
    print_buffer[len] = '\0';
    *out_prints = print_buffer;

    const uint8_t *jstr = janet_to_string(ret);
    strncpy(result_buffer, (const char*)jstr, sizeof(result_buffer) - 1);
    result_buffer[sizeof(result_buffer) - 1] = '\0';

    janet_deinit();
    
    return result_buffer;
}