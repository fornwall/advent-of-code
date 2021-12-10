#include <advent-of-code.h>
#include <stdio.h>

void test(uint16_t year, uint8_t day, uint8_t part, const char* input) {
    bool ok;

    char* result = advent_of_code_solve(year, day, part, input, &ok);

    printf("%d-%d-%d: Input='%s' -> ok=%s, output='%s'\n", year, day, part, input, ok ? "true" : "false", result);

    free(result);
}

int main() {
    test(2019, 1, 1, "14");
    test(2019, 1, 1, "hej");
    test(2019, 1, 1, "");
    test(2019, 1, 1, "ö");
    test(2019, 1, 1, NULL);
    test(2019, 1, 1, "\xc3\x28");
    test(2021, 7, 2, "16,1,2,0,4,2,7,1,2,14");
    return 0;
}
