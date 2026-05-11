#include <stdio.h>
#include <stdlib.h>


#define MAX_INPUT_SIZE 4096

// this is a low level function to read directly from stdin without any interferences
char* get_stdin_pipe_input() {
    char* buffer = (char*)malloc(MAX_INPUT_SIZE);
    char* ptr_or_err = fgets(buffer, MAX_INPUT_SIZE, stdin);
    if (ptr_or_err == NULL) {
        free((void*)buffer);
        return NULL;
    } else {
        return buffer;
    }
}
