import ctypes
import os

import numpy as np
from pelutils import OS, TT, array_ptr
from tqdm import tqdm


def load_library(name: str) -> ctypes.CDLL:
    print(f"Loading {name}")
    lib = ctypes.cdll.LoadLibrary(os.path.join("build", name + (".dll" if OS.is_windows else ".so")))
    lib.elementwise_mult.argtypes = (ctypes.c_size_t, ctypes.c_void_p, ctypes.c_void_p, ctypes.c_void_p)
    return lib

def benchmark(libs: dict[str, ctypes.CDLL]):
    reps = 1000
    array_size = 10 ** 6

    for i in tqdm(range(reps)):
        with TT.profile("Generate data"):
            arr0 = np.random.uniform(0, 1, array_size)
            arr1 = np.random.uniform(0, 1, array_size)
        with TT.profile("Numpy"):
            result_np = arr0 * arr1

        for libname, lib in libs.items():
            with TT.profile(libname):
                result = np.empty_like(arr0)
                lib.elementwise_mult(array_size, array_ptr(arr0), array_ptr(arr1), array_ptr(result))
            # Validate some of the results
            if i and i % (10 * len(libs)) == 0:
                with TT.profile("Validate"):
                    assert np.isclose(result, result_np).all(), f"Wrong result from {libname}"

    print(TT)

if __name__ == "__main__":
    libs: dict[str, ctypes.CDLL] = dict()
    for libname in "libc", "libcpp", "libcu", "librust":
        try:
            libs[libname] = load_library(libname)
        except OSError:
            print(f"Failed to load {libname}")

    benchmark(libs)

