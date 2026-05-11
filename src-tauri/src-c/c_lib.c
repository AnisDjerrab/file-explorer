#include <stdbool.h>
#include <stdio.h>

#ifdef _WIN32
    #define PLATFORM_WINDOWS
#else
    #define PLATFORM_UNIX
    #include <sys/types.h>
    #include <unistd.h>
#endif

#define MAX_IO_BUFFER_SIZE 4096

typedef struct {
    bool success;
    FILE* stdin;
    FILE* stdout;
    FILE* stderr;
} process_infos;

process_infos* establish_comms_with_service_unix(const char* pipe_dir_path) {
    #ifdef PLATFORM_UNIX
    #endif
}
