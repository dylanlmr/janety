#include "janet.h"
#include <string.h>
#include <stdlib.h>
#include <emscripten.h>

static char result_buffer[8192];
static char print_buffer[65536];

EMSCRIPTEN_KEEPALIVE
int evaluate_janet_wasm(const char* code) {
    janet_init();
    JanetTable *env = janet_core_env(NULL);
    
    JanetBuffer *console_buf = janet_buffer(0);
    janet_table_put(env, janet_csymbolv("__out_buf__"), janet_wrap_buffer(console_buf));

    const char* inject = "(def __out_buf__ @\"\") (setdyn :out __out_buf__) (setdyn :err __out_buf__)\n";
    size_t total_len = strlen(inject) + strlen(code) + 1;
    char* full_code = malloc(total_len);
    strcpy(full_code, inject);
    strcat(full_code, code);

    Janet ret;
    int status = janet_dostring(env, full_code, "web_repl", &ret);
    free(full_code);

    Janet prints_val;
    if (janet_dostring(env, "__out_buf__", "get_prints", &prints_val) == 0) {
        const uint8_t *prints_str = janet_to_string(prints_val);
        strncpy(print_buffer, (const char*)prints_str, sizeof(print_buffer) - 1);
        print_buffer[sizeof(print_buffer) - 1] = '\0';
    } else {
        print_buffer[0] = '\0';
    }

    const uint8_t *jstr = janet_to_string(ret);
    strncpy(result_buffer, (const char*)jstr, sizeof(result_buffer) - 1);
    result_buffer[sizeof(result_buffer) - 1] = '\0';

    janet_deinit();
    return status;
}

EMSCRIPTEN_KEEPALIVE
const char* get_wasm_result() {
    return result_buffer;
}

EMSCRIPTEN_KEEPALIVE
const char* get_wasm_console() {
    return print_buffer;
}