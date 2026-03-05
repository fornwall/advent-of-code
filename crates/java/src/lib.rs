#![allow(clippy::expect_used)]
#![allow(clippy::panic)]
use advent_of_code::solve;
use jni::EnvUnowned;
use jni::errors::ThrowRuntimeExAndDefault;
use jni::objects::{JClass, JString};
use jni::strings::JNIString;
use jni::sys::jstring;

#[unsafe(no_mangle)]
pub extern "system" fn Java_net_fornwall_aoc_Solver_solve(
    mut unowned_env: EnvUnowned,
    _class: JClass,
    year: i32,
    day: i32,
    part: i32,
    input: JString,
) -> jstring {
    unowned_env
        .with_env(|env| -> jni::errors::Result<_> {
            let input_str: String = input.to_string();

            let exception_message = match convert_params(year, day, part) {
                Ok((year, day, part)) => match solve(year, day, part, &input_str) {
                    Ok(output) => {
                        return Ok(env
                            .new_string(output)
                            .expect("Unable to create output string")
                            .into_raw());
                        //.new_string(output) .expect("Unable to create output string") .into_raw();
                    }
                    Err(msg) => msg,
                },
                Err(message) => message,
            };

            let class_name = JNIString::from("net/fornwall/aoc/SolverException");
            let s = JNIString::from(exception_message);
            env.throw_new(class_name, &s)
                .expect("Unable to throw exception");
            Ok(::std::ptr::null_mut())
            //Ok(JObject::null().into())
        })
        .resolve::<ThrowRuntimeExAndDefault>()
}

fn convert_params(year: i32, day: i32, part: i32) -> Result<(u16, u8, u8), String> {
    Ok((
        u16::try_from(year).map_err(|_| format!("Invalid year: {year}"))?,
        u8::try_from(day).map_err(|_| format!("Invalid day: {day}"))?,
        u8::try_from(part).map_err(|_| format!("Invalid part: {part}"))?,
    ))
}
