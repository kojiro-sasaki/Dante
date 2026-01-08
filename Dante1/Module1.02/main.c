#include <stdio.h>
#include <stdlib.h>

int main() {
    int a, b;

    printf("Podaj dwie liczby calkowite oddzielone spacja: ");

    if (scanf("%d %d", &a, &b) != 2) {
        printf("Incorrect input");
        return 1;
    }

    printf("%d", a + b);
    return 0;
}
