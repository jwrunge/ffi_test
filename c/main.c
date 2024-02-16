#include <stdio.h>
#include <string.h>
#include <stdlib.h>

char* HelloC(char* name) {
    if(strlen(name) == 0) name = "Anonymous";
    char* greeting = "Howdy, ";
    char* end = " -- greetings from C!";
    char* result = malloc(strlen(greeting) + strlen(name) + strlen(end) + 1); // +1 for the null-terminator
    strcpy(result, greeting);
    strcat(result, name);
    strcat(result, end);
    free(result);
    return result;
}