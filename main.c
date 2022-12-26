#include <stdio.h>

void finding1 () {
    double quotient = 1L;

    for (int i = 0; i < 7; i++) {
        quotient /= 10;
        printf("%d %0.20f\n", i, quotient);
    }
}

int main (int argc, char **argv) {
    finding1();
    return 0;
}