#![allow(clippy::expect_used)]
#![allow(clippy::panic)]
use advent_of_code::solve;
use jni::objects::{JClass, JString};
use jni::sys::jstring;
use jni::JNIEnv;

#[no_mangle]
pub extern "system" fn Java_net_fornwall_aoc_Solver_solve(
    env: JNIEnv,
    _class: JClass,
    year: i32,
    day: i32,
    part: i32,
    input: JString,
) -> jstring {
    let input_str: String = env
        .get_string(input)
        .expect("Unable to get input string")
        .into();

    let exception_message = match convert_params(year, day, part) {
        Ok((year, day, part)) => match solve(year, day, part, &input_str) {
            Ok(output) => {
                return env
                    .new_string(output)
                    .expect("Unable to create output string")
                    .into_inner();
            }
            Err(msg) => msg,
        },
        Err(message) => message,
    };

    env.throw_new("net/fornwall/aoc/SolverException", exception_message)
        .expect("Unable to throw exception");
    ::std::ptr::null_mut()
}

fn convert_params(year: i32, day: i32, part: i32) -> Result<(u16, u8, u8), String> {
    Ok((
        u16::try_from(year).map_err(|_| format!("Invalid year: {}", year))?,
        u8::try_from(day).map_err(|_| format!("Invalid day: {}", day))?,
        u8::try_from(part).map_err(|_| format!("Invalid part: {}", part))?,
    ))
}
