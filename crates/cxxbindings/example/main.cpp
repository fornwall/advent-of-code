#include <advent-of-code.hpp>
#include <exception>
#include <iostream>
#include <string>

void test(uint16_t year, uint8_t day, uint8_t part, std::string input) {
    std::string message;
    std::string ok;
    try {
        message = std::string(aoc::solve(year, day, part, input));
        ok = "true";
    } catch (std::exception& e) {
        message = e.what();
        ok = "false";
    }

    std::cout << year << '-' << (int) day << '-' << (int) part << ": Input='" << input << "' -> ok=" << ok << ", output='" << message << "'" << std::endl;
    //printf("%d-%d-%d: Input='%s' -> ok=%s, output='%s'\n", year, day, part, input, ok ? "true" : "false", message);
}

int main() {
    test(2019, 1, 1, "14");
    test(2019, 1, 1, "hej");
    test(2019, 1, 1, "");
    test(2019, 1, 1, "ö");
    // test(2019, 1, 1, NULL);
    test(2019, 1, 1, "\xc3\x28");
    test(2021, 7, 2, "16,1,2,0,4,2,7,1,2,14");
    return 0;
}
