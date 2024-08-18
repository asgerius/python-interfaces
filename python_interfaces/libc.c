#include <stdlib.h>


int elementwise_mult(size_t n, double *arr0, double *arr1, double *result) {
    for (size_t i = 0; i < n; ++ i) {
        result[i] = arr0[i] * arr1[i];
    }
    return 0;
}
