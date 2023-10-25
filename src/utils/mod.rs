pub mod scoped_perf;
pub mod to_any;

// SAFETY: You must guarantee lifetime for string. Pointer must be c string in UTF-8 or ASCII encoding
pub unsafe fn c_string_to_str<T: num::Num>(string: *const T) -> &'static str {
    let mut cur = string as *const u8;
    let size = unsafe {
        while (*cur != 0u8) {
            cur = cur.add(1);
        }
        cur.offset_from(string as *const u8) as usize
    };
    let byte_array = std::slice::from_raw_parts(string as *const u8, size);
    std::str::from_utf8(byte_array).unwrap()
}
