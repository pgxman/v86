use cpu2::cpu::{
    safe_read16, safe_read32s, safe_write16, safe_write32, translate_address_read,
    translate_address_system_read, translate_address_system_write, writable_or_pagefault,
};
use cpu2::misc_instr::{pop16, pop32s, push16, push32};

#[no_mangle]
pub unsafe fn safe_read16_js(addr: i32) -> i32 { safe_read16(addr).unwrap() }
#[no_mangle]
pub unsafe fn safe_read32s_js(addr: i32) -> i32 { safe_read32s(addr).unwrap() }

#[no_mangle]
pub unsafe fn safe_write16_js(addr: i32, value: i32) { safe_write16(addr, value).unwrap() }
#[no_mangle]
pub unsafe fn safe_write32_js(addr: i32, value: i32) { safe_write32(addr, value).unwrap() }

#[no_mangle]
pub unsafe fn translate_address_read_js(addr: i32) -> u32 { translate_address_read(addr).unwrap() }
#[no_mangle]
pub unsafe fn translate_address_system_read_js(addr: i32) -> u32 {
    translate_address_system_read(addr).unwrap()
}
#[no_mangle]
pub unsafe fn translate_address_system_write_js(addr: i32) -> u32 {
    translate_address_system_write(addr).unwrap()
}

#[no_mangle]
pub unsafe fn writable_or_pagefault_js(addr: i32, size: i32) -> bool {
    writable_or_pagefault(addr, size).is_ok()
}

#[no_mangle]
pub unsafe fn push16_js(value: i32) { push16(value).unwrap() }

#[no_mangle]
pub unsafe fn push32_js(value: i32) { push32(value).unwrap() }

#[no_mangle]
pub unsafe fn pop16_js() -> i32 { pop16().unwrap() }

#[no_mangle]
pub unsafe fn pop32s_js() -> i32 { pop32s().unwrap() }
