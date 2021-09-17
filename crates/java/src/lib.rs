#![allow(clippy::expect_used)]
#![allow(clippy::panic)]
use advent_of_code::solve;
use jni::objects::{JClass, JObject, JString};
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
        .expect("Couldn't get java string!")
        .into();

    let result = match solve(year as u16, day as u8, part as u8, &input_str) {
        Ok(output) => env
            .new_string(output)
            .expect("Couldn't create java string!"),
        Err(msg) => {
            env.throw(("net/fornwall/aoc/SolverException", msg))
                .expect("Unable to throw exception");
            JObject::null().into()
        }
    };

    result.into_inner()
}
