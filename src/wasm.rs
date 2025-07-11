use alloc::collections::btree_map::BTreeMap;
use core::ptr;
use riscv::register::mhartid;
use spin::Mutex;
use wasmtime::Config;

static WASMTIME_TLS_PTRS_MAP: Mutex<BTreeMap<usize, usize>> = Mutex::new(BTreeMap::new());

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wasmtime_tls_get() -> *mut u8 {
    WASMTIME_TLS_PTRS_MAP
        .lock()
        .get(&mhartid::read())
        .map_or_else(|| ptr::null_mut(), |&ptr| ptr as *mut u8)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wasmtime_tls_set(ptr: *mut u8) {
    let mut map = WASMTIME_TLS_PTRS_MAP.lock();
    map.insert(mhartid::read(), ptr as usize);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wasmtime_longjmp(jmp_buf: *const u8) -> ! {
    todo!();
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wasmtime_setjmp(
    jmp_buf: *const *mut u8,
    callback: unsafe extern "C" fn(_: *mut u8, _: *mut u8) -> bool,
    payload: *mut u8,
    callee: *mut u8,
) -> bool {
    todo!();
}

fn get_wasmtime_config() -> Config {
    let mut config = Config::new();
    config
        .target("pulley64")
        .expect("Failed to set wasmtime target to pulley64!");
    config
}
