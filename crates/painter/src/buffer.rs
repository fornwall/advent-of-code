use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

const HEADER_ELEMENT_LENGTH: usize = 3;
const HEADER_BYTE_LENGTH: usize = HEADER_ELEMENT_LENGTH * 4;
const HEADER_READER_WANT_MORE_OFFSET: usize = 0;
const HEADER_READ_OFFSET: usize = 1;
const HEADER_WRITE_OFFSET: usize = 2;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

pub struct CircularOutputBuffer {
    shared_buffer: Vec<i32>,
    non_flushed_writes: i32,
}

impl CircularOutputBuffer {
    pub fn data_len(&self) -> usize {
        return self.shared_buffer.len() - HEADER_ELEMENT_LENGTH;
    }

    pub fn new() -> Self {
        let mut result = Self {
            shared_buffer: vec![0; 64 * 65536],
            non_flushed_writes: 0,
        };

        let data_buffer_offset = unsafe { result.shared_buffer.as_mut_ptr() as u32 };
        let data_buffer_byte_length = result.shared_buffer.len() as u32 * 4;
        let memory_buffer = wasm_bindgen::memory()
            .unchecked_into::<js_sys::WebAssembly::Memory>()
            .buffer();

        let message = js_sys::Object::new();
        js_sys::Reflect::set(&message, &"type".into(), &"buffer".into());
        js_sys::Reflect::set(&message, &"buffer".into(), &memory_buffer);
        js_sys::Reflect::set(&message, &"offset".into(), &data_buffer_offset.into());
        js_sys::Reflect::set(&message, &"length".into(), &data_buffer_byte_length.into());

        let worker_scope: web_sys::DedicatedWorkerGlobalScope = js_sys::global().unchecked_into();
        worker_scope.post_message(&message);

        result
    }

    pub fn write(&mut self, value: i32) {
        let mut write_offset = ((self.shared_buffer[HEADER_WRITE_OFFSET] + self.non_flushed_writes)
            as usize
            % self.data_len())
            + HEADER_ELEMENT_LENGTH;
        self.shared_buffer[write_offset] = value;
        self.non_flushed_writes += 1;
    }

    pub fn write_float(&mut self, value: f32) {
        self.write(value.to_bits() as i32);
    }

    pub fn write_float3(&mut self, a: f32, b: f32, c: f32) {
        self.write_float(a);
        self.write_float(b);
        self.write_float(c);
    }

    pub fn write_float4(&mut self, a: f32, b: f32, c: f32, d: f32) {
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

    pub fn write4(&mut self, a: i32, b: i32, c: i32, d: i32) {
        self.write(a);
        self.write(b);
        self.write(c);
        self.write(d);
    }

    pub fn wait(&mut self) {
        self.shared_buffer[HEADER_READER_WANT_MORE_OFFSET] = 0;

        unsafe {
            // Firefox issue? i64::MAX makes memory_atomic_wait32() return 2 directly:
            let timeout_ns = 100_000_000_000_000_000;

            let raw_pointer: *mut i32 = self.shared_buffer.as_mut_ptr();
            // Block while there is no more writes desired: while header[HEADER_READER_WANT_MORE_OFFSET] == 0.
            // https://docs.rs/core_arch/0.1.5/core_arch/wasm32/fn.i32_atomic_wait.html
            core::arch::wasm32::memory_atomic_wait32(raw_pointer, 0, timeout_ns);
        }
    }
}
