#include <stdio.h>
#include <stdlib.h>
#include <errno.h>
#include <fcntl.h>
#include <string.h>
#include <unistd.h>

#define BUFFER_LENGTH 256
static char receive[BUFFER_LENGTH];

int main(){
    int ret, fd;
    char stringToSend[BUFFER_LENGTH];
    char echoCmd[300];
    printf("Starting device test code example...\n");
    fd = open("/dev/ebbchar", O_RDWR);             // Open the device with read/write access
    if (fd < 0){
        perror("Failed to open the device...");
        return errno;
    }

    while(1){
        printf("Type in a short string to send to the kernel module:\n");
        scanf("%[^\n]%*c", stringToSend);                // Read in a string (with spaces)
        printf("Writing message to the device [%s].\n", stringToSend);
        sprintf(echoCmd, "echo \"%s\" > /dev/ebbchar", stringToSend);
        ret = system(echoCmd); // Send the string to the LKM
        if (ret < 0){
            perror("Failed to write the message to the device.");
            return errno;
        }

        //printf("Press ENTER to read back from the device...\n");
        //getchar();
      
        ret = system("cat /dev/ebbchar");

        //printf("Reading from the device...\n");
        //ret = read(fd, receive, BUFFER_LENGTH);        // Read the response from the LKM
        if (ret < 0){
            perror("Failed to read the message fm the device.");
            return errno;
        }
        //printf("The received message is: [%s]\n", receive);
    }

    printf("End of the program\n");
    close(fd);
    return 0;
}
