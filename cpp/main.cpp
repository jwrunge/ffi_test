#include <iostream>
#include <string>

std::string HelloCpp(std::string name) {
    if(name.length() == 0) name = "Anonymous";
    std::string greeting = "Howdy, ";
    std::string end = " -- greetings from C++!";
    std::string result = greeting + name + end;
    return result;
}