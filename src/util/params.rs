use std::collections::HashMap;
use std::sync::OnceLock;

pub static APP_PARAMS: OnceLock<HashMap<String, String>> = OnceLock::new();

#[cfg(target_arch = "wasm32")]
extern "C" {
    fn get_query_string(ptr: *mut u8, max_len: u32) -> u32;
}

/// Provides the query params or the command line arguments.
/// On web args are split by "&".
/// On standalone args are split by space.
pub fn get_params() -> HashMap<String, String> {
    #[cfg(target_arch = "wasm32")]
    {
        let mut buffer = vec![0u8; 256];

        let len = unsafe {
            get_query_string(buffer.as_mut_ptr(), buffer.len() as u32)
        };

        if len <= 0 {
            return HashMap::new();
        }

        buffer.truncate(len as usize);
        let raw_query = String::from_utf8_lossy(&buffer).into_owned();

        if raw_query.is_empty() {
            return HashMap::new();
        }

        let query_trimmed = raw_query.strip_prefix('?').unwrap_or(&raw_query);

        if query_trimmed.is_empty() {
            return HashMap::new();
        }

        let result = query_trimmed
            .split('&')
            .map(|pair| {
                if let Some((key, value)) = pair.split_once('=') {
                    (key.to_lowercase(), value.to_string())
                } else {
                    (pair.to_lowercase(), String::new())
                }
            })
            .collect::<HashMap<String, String>>();

        result
    }


    #[cfg(not(target_arch = "wasm32"))]
    {
        let result = std::env::args().skip(1)
        .map(|pair| {
            if let Some((key, value)) = pair.split_once('=') {
                (key.to_lowercase(), value.to_string())
            } else {
                (pair.to_lowercase(), String::new())
            }
        })
        .collect::<HashMap<String, String>>();

        result
    }
}

pub fn get_app_param_int(key: &str, default: i32) -> i32 {
    get_raw_param(key)
        .and_then(|val| val.parse::<i32>().ok())
        .unwrap_or(default)
}

pub fn get_app_param_string(key: &str, default: &str) -> String {
    get_raw_param(key)
        .cloned()
        .unwrap_or_else(|| default.to_string())
}

pub fn get_app_param_bool(key: &str, default: bool) -> bool {
    get_raw_param(key)
        .map(|val| {
            match val.to_lowercase().as_str() {
                "true" | "1" | "yes" | "on" | "" => true,
                "false" | "0" | "no" | "off" => false,
                _ => default,
            }
        })
        .unwrap_or(default)
}

fn get_raw_param(key: &str) -> Option<&String> {
    let search_key = key.to_lowercase();
    APP_PARAMS.get().and_then(|params| params.get(&search_key))
}