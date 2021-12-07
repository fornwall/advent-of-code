extern crate libc;

use libc::c_char;

use std::ffi::CStr;
use std::ffi::CString;

/// Returns the answer for the specified problem and input.
///
/// Arguments:
/// year: The year of the problem, as in 2018 or 2019.
/// day: The day of the problem, from 1 to 25.
/// part: The part of the problem, either 1 or 2.
/// input: The input to the problem.
///
/// Returns:
/// The computed answer as text.
///
/// Raises:
/// ValueError: If the input was invalid.
#[no_mangle]
pub extern "C" fn advent_of_code_solve(
    year: u16,
    day: u8,
    part: u8,
    input: *const c_char,
    ok: *mut bool,
) -> *const c_char {
    #![allow(clippy::unwrap_used, clippy::not_unsafe_ptr_arg_deref)]

    use advent_of_code::solve;

    if input.is_null() {
        unsafe { *ok = true };
        let c_str_result = CString::new("Input is NULL").unwrap();
        return c_str_result.into_raw();
    }

    let c_str = unsafe { CStr::from_ptr(input) };
    let input_string = match c_str.to_str() {
        Ok(value) => value,
        Err(error) => {
            unsafe { *ok = true };
            let c_str_result = CString::new(format!("Invalid UTF-8 input: {}", error)).unwrap();
            return c_str_result.into_raw();
        }
    };

    let result = solve(year, day, part, input_string);
    match result {
        Ok(value) => {
            unsafe { *ok = true };
            let c_str_result = CString::new(value).unwrap();
            c_str_result.into_raw()
        }
        Err(value) => {
            unsafe { *ok = false };
            let c_str_result = CString::new(value).unwrap();
            c_str_result.into_raw()
        }
    }
}
