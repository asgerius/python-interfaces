// https://stackoverflow.com/questions/63165778/numpy-array-to-rust-by-ndpointer-fails-in-windows-works-on-linux

#[no_mangle]
pub extern "C" fn elementwise_mult(n: usize, arr0: *mut f64, arr1: *mut f64, result: *mut f64) -> i32 {
    let arr0_data: &mut[f64] = unsafe {
        assert!(!arr0.is_null());
        std::slice::from_raw_parts_mut(arr0, n)
    };
    let arr1_data: &mut[f64] = unsafe {
        assert!(!arr1.is_null());
        std::slice::from_raw_parts_mut(arr1, n)
    };
    let result_data: &mut[f64] = unsafe {
        assert!(!result.is_null());
        std::slice::from_raw_parts_mut(result, n)
    };
    for i in 0..n {
        result_data[i] = arr0_data[i] * arr1_data[i];
    }
    return 0;
}
