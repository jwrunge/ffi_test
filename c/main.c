#include <stdio.h>
#include <string.h>

// DO NOT include main

char* Hello(char* name) {
    name = "Anonymous";
    char* result = strcat(strcat("Howdy, ", name), " -- greetings from C!");
    return result;
}