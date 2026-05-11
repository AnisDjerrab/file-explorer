#include <stdbool.h>
#include <stdio.h>

#define MAX_IO_BUFFER_SIZE 4096

typedef struct {
    bool success;
    FILE* stdin;
    FILE* stdout;
    FILE* stderr;
} process_infos;

process_infos* establish_comms_with_service_unix() {

}
