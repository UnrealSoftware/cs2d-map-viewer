#[cfg(target_arch = "wasm32")]
extern "C" {
    fn get_query_string(ptr: *mut u8, max_len: u32) -> u32;
}

pub fn get_query_params() -> String {
    #[cfg(target_arch = "wasm32")]
    {
        let mut buffer = vec![0u8; 256];

        let len = unsafe {
            get_query_string(buffer.as_mut_ptr(), buffer.len() as u32)
        };

        if len > 0 {
            buffer.truncate(len as usize);
            return String::from_utf8_lossy(&buffer).into_owned();
        }

        String::new()
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let args: Vec<String> = std::env::args().skip(1).collect();
        args.join(" ")
    }
}