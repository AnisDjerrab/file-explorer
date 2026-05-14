#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>
#include <string.h>

#ifdef _WIN32
    #define PLATFORM_WINDOWS
    // todo => win imports
    // todo => win struct
    typedef struct {
    } pipe_struct;
#else
    #define PLATFORM_UNIX
    #include <sys/stat.h>
    #include <sys/types.h>
    #include <unistd.h>
    #include <sched.h>
    typedef struct {
        FILE* pipe_out;
        FILE* pipe_in;
    } pipe_struct;
#endif

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


// this is the service pipe implementation.
#define MAX_IO_BUFFER_SIZE 4096

pipe_struct* establish_comms_with_service_unix(const char* pipe_dir_path) {
    #ifdef PLATFORM_UNIX
        // initialize the return value
        pipe_struct* output = malloc(sizeof(pipe_struct));
        if (output == NULL) {
            return NULL;
        }
        // the first pipe is the pipe from this main process to the service. the stdin equivalent.
        // assemble the hole path into a buffer.
        char pipe_in_path[strlen(pipe_dir_path) + strlen("pipe_core_to_service") + 1];
        memcpy(pipe_in_path, pipe_dir_path, strlen(pipe_dir_path));
        memcpy(pipe_in_path + strlen(pipe_dir_path), "pipe_core_to_service", strlen("pipe_core_to_service"));
        pipe_in_path[strlen(pipe_dir_path) + strlen("pipe_core_to_service")] = '\0';
        // open the path in RO
        FILE* fd_in;
        fd_in = fopen(pipe_in_path, "r");
        output->pipe_in = fd_in;
        // now, it's time to create the output pipe
        char pipe_out_path[strlen(pipe_dir_path) + strlen("pipe_service_to_core") + 1];
        memcpy(pipe_out_path, pipe_dir_path, strlen(pipe_dir_path));
        memcpy(pipe_out_path + strlen(pipe_dir_path), "pipe_service_to_core", strlen("pipe_service_to_core"));
        pipe_out_path[strlen(pipe_dir_path) + strlen("pipe_service_to_core")] = '\0';
        // create the pipe file & pipe itself
        FILE* fd_out;
        // check if it fails
        if (mkfifo(pipe_out_path, 0666) == -1) {
            free(output);
            return NULL;
        }
        // we are waiting for the other side to just open the file in RO
        while ((fd_out = fopen(pipe_out_path, "w")) == NULL) {
            sched_yield();
        }
        output->pipe_out = fd_out;
        return output;
    #elifdef PLATFORM_WINDOWS
        // todo => implement win pipes
        return NULL;
    #endif
}
