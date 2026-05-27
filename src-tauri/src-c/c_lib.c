#include <stdbool.h>
#include <stdio.h>
#include <string.h>
#include <stdlib.h>

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

#define MAX_IO_BUFFER_SIZE 4096

pipe_struct* establish_comms_with_service_unix(const char* pipe_dir_path) {
    #ifdef PLATFORM_UNIX
        fprintf(stderr, "entered func");
        // initialize the return value
        pipe_struct* output = malloc(sizeof(pipe_struct));
        if (output == NULL) {
            return NULL;
        }
        // the first pipe is the pipe from this main process to the service. the stdout equivalent.
        // assemble the hole path into a buffer.
        char pipe_out_path[strlen(pipe_dir_path) + strlen("pipe_core_to_service") + 1];
        memcpy(pipe_out_path, pipe_dir_path, strlen(pipe_dir_path));
        memcpy(pipe_out_path + strlen(pipe_dir_path), "pipe_core_to_service", strlen("pipe_core_to_service"));
        pipe_out_path[strlen(pipe_dir_path) + strlen("pipe_core_to_service")] = '\0';
        // create the pipe file & pipe itself
        FILE* fd_out;
        // check if it fails
        fprintf(stderr, "chkpt 1");
        if (mkfifo(pipe_out_path, 0777) == -1) {
            free(output);
            return NULL;
        }
        fprintf(stderr, "chkpt 2");
        // we are waiting for the other side to just open the file in RO
        while ((fd_out = fopen(pipe_out_path, "w")) == NULL) {
            sched_yield();
        }
        fprintf(stderr, "chkpt 3");
        output->pipe_out = fd_out;
        // now, it's time to get the input pipe
        // in order to do that, we need to wait for the file to be created
        char pipe_in_path[strlen(pipe_dir_path) + strlen("pipe_service_to_core") + 1];
        memcpy(pipe_in_path, pipe_dir_path, strlen(pipe_dir_path));
        memcpy(pipe_in_path + strlen(pipe_dir_path), "pipe_service_to_core", strlen("pipe_service_to_core"));
        pipe_in_path[strlen(pipe_dir_path) + strlen("pipe_service_to_core")] = '\0';
        // loop where we test whether the file exists
        struct stat st;
        fprintf(stderr, "chkpt 4");
        while (stat(pipe_in_path, &st) != 0) {
            sched_yield();
        };
        fprintf(stderr, "chkpt 5");
        // now, we know the file exists
        // open it in RO
        FILE* fd_in = fopen(pipe_in_path, "r");
        output->pipe_in = fd_in;
        fprintf(stderr, "chkpt 6");
        return output;
    #elifdef PLATFORM_WINDOWS
        // todo => implement win pipes
        return NULL;
    #endif
}
