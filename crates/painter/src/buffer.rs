#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
use wasm_bindgen::prelude::*;
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
use wasm_bindgen::JsCast;

const HEADER_ELEMENT_LENGTH: usize = 4;
// const HEADER_BYTE_LENGTH: usize = HEADER_ELEMENT_LENGTH * 4;
const HEADER_READER_WANT_MORE_OFFSET: usize = 0;
const HEADER_READ_OFFSET: usize = 1;
const HEADER_WRITE_OFFSET: usize = 2;
const HEADER_OK_TO_EXIT_OFFSET: usize = 3;

#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);

    #[wasm_bindgen]
    fn do_wait(offset: u32, value: i32);
}

#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
#[macro_export]
macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

pub struct CircularOutputBuffer {
    shared_buffer: Vec<i32>,
    non_flushed_writes: i32,
}

impl CircularOutputBuffer {
    pub fn new() -> Self {
        let mut result = Self {
            shared_buffer: vec![0; 16 * 65536],
            non_flushed_writes: 0,
        };

        #[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
        {
            let data_buffer_offset = result.shared_buffer.as_mut_ptr() as u32;
            let data_buffer_byte_length = result.shared_buffer.len() as u32 * 4;
            let memory_buffer = wasm_bindgen::memory()
                .unchecked_into::<js_sys::WebAssembly::Memory>()
                .buffer();

            let message = js_sys::Object::new();
            js_sys::Reflect::set(&message, &"type".into(), &"buffer".into()).unwrap();
            js_sys::Reflect::set(&message, &"buffer".into(), &memory_buffer).unwrap();
            js_sys::Reflect::set(&message, &"offset".into(), &data_buffer_offset.into()).unwrap();
            js_sys::Reflect::set(&message, &"length".into(), &data_buffer_byte_length.into())
                .unwrap();

            let worker_scope: web_sys::DedicatedWorkerGlobalScope =
                js_sys::global().unchecked_into();
            worker_scope.post_message(&message).unwrap();
        }

        result
    }

    pub fn data_len(&self) -> usize {
        self.shared_buffer.len() - HEADER_ELEMENT_LENGTH
    }

    fn writer_offset(&self) -> usize {
        (self.shared_buffer[HEADER_WRITE_OFFSET] + self.non_flushed_writes) as usize
            % self.data_len()
            + HEADER_ELEMENT_LENGTH
    }

    pub fn write(&mut self, value: i32) {
        let write_offset = self.writer_offset();
        self.shared_buffer[write_offset] = value;
        self.non_flushed_writes += 1;
    }

    pub fn write_float(&mut self, value: f64) {
        self.write((value as f32).to_bits() as i32);
    }

    pub fn write_float3(&mut self, a: f64, b: f64, c: f64) {
        self.write_float(a);
        self.write_float(b);
        self.write_float(c);
    }

    pub fn write_float4(&mut self, a: f64, b: f64, c: f64, d: f64) {
        self.write_float(a);
        self.write_float(b);
        self.write_float(c);
        self.write_float(d);
    }

    pub fn flush(&mut self) {
        if self.non_flushed_writes > 0 {
            self.shared_buffer[HEADER_WRITE_OFFSET] =
                (self.shared_buffer[HEADER_WRITE_OFFSET] + self.non_flushed_writes) as i32
                    % (self.data_len() as i32);
            self.non_flushed_writes = 0;
        }
    }

    pub fn write_text(&mut self, text: &str) {
        self.write(text.len() as i32);

        unsafe {
            let byte_pointer = self.shared_buffer.as_mut_ptr() as *mut u8;
            let buffer_start = self.writer_offset() * 4;
            for i in 0..text.len() {
                byte_pointer
                    .offset((buffer_start + i) as isize)
                    .write(text.as_bytes()[i]);
            }
            self.non_flushed_writes +=
                (text.len() / 4 + if text.len() % 4 == 0 { 0 } else { 1 }) as i32;
        }
    }

    pub fn perhaps_wait(&mut self) {
        self.shared_buffer[HEADER_READER_WANT_MORE_OFFSET] = 0;

        let read_offset = self.shared_buffer[HEADER_READ_OFFSET];
        let write_offset = self.shared_buffer[HEADER_WRITE_OFFSET];
        let used = if read_offset > write_offset {
            write_offset - read_offset + self.data_len() as i32
        } else {
            write_offset - read_offset
        };
        if used < (self.data_len() as i32 / 3) {
            return;
        } else {
            /*
            #[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
            console_log!(
                "[rust] Waiting, use ratio {}",
                used as f32 / self.data_len() as f32
            );
             */
        }

        #[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
        unsafe {
            let timeout_ns = -1;
            let raw_pointer: *mut i32 = self.shared_buffer.as_mut_ptr();
            if raw_pointer as usize == 1 {
                self.log(
                    "This check of raw_pointer is necessary for (wasm-opt|compiler)? to keep it",
                );
            }
            core::arch::wasm32::memory_atomic_wait32(raw_pointer, 0, timeout_ns);

            // A variant calling out to javascript, requires lines to be uncommented in
            // worker-visualiser.js. Still needs nightly build with atomics feature to
            // make wasm-bindgen create the wasm memory with the shared flag:
            // let data_buffer_offset = unsafe { self.shared_buffer.as_mut_ptr() as u32 } / 4;
            // do_wait(data_buffer_offset, 0);
        }
    }

    /// Avoid exiting program while the worker javascript has not finished reading the output buffer.
    pub fn wait_forever(&mut self) {
        #[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
        unsafe {
            let timeout_ns = -1;
            let raw_pointer: *mut i32 = self
                .shared_buffer
                .as_mut_ptr()
                .offset(HEADER_OK_TO_EXIT_OFFSET as isize);
            if raw_pointer as usize == 1 {
                self.log(
                    "This check of raw_pointer is necessary for (wasm-opt|compiler)? to keep it",
                );
            }
            self.log("Done - asking for permission to exit...");
            core::arch::wasm32::memory_atomic_wait32(raw_pointer, 0, timeout_ns);
            self.log("Got ok to exit!");
        }
    }

    pub fn log(&mut self, text: &str) {
        console_log!("[rust] {}", text);
    }
}

#[test]
fn basic_buffer() {
    let mut buffer = CircularOutputBuffer::new();
    //assert_eq!(buffer.shared_buffer[HEADER_READ_OFFSET], 1);

    buffer.write(12345);

    // Ok.
}
