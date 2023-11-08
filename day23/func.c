#include <stdio.h>
#include <math.h>

int is_prime(int n) {
    int max = (int) sqrt( (double) n) + 1;
    for(int div = 2; div <= max; div++) {
        if((n / div) * div == n) {
            return 0;
        }
    }
    return 1;
}

int func() {
    int count = 0;
    for(int b = 108400; b <= 125400; b += 17)
        count += !is_prime(b);
    return count;
}

int main() {
    printf("%d\n", func());
}
