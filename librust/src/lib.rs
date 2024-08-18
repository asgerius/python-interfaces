// https://stackoverflow.com/questions/63165778/numpy-array-to-rust-by-ndpointer-fails-in-windows-works-on-linux

fn array_from_raw<T>(n: usize, ptr: *mut T) -> &'static mut[T] {
    unsafe {
        assert!(!ptr.is_null());
        std::slice::from_raw_parts_mut(ptr, n)
    }
}

#[no_mangle]
pub extern "C" fn elementwise_mult(n: usize, arr0: *mut f64, arr1: *mut f64, result: *mut f64) -> i32 {
    let arr0_data = array_from_raw(n, arr0);
    let arr1_data = array_from_raw(n, arr1);
    let result_data = array_from_raw(n, result);
    for i in 0..n {
        result_data[i] = arr0_data[i] * arr1_data[i];
    }
    return 0;
}
