fn main() -> anyhow::Result<()> {
    wasm_opt_sys::init();

    wasm_opt_main()
}

mod c {
    use libc::{c_char, c_int};

    extern "C" {
        pub fn wasm_opt_main(argc: c_int, argv: *const *const c_char) -> c_int;
    }
}

pub fn wasm_opt_main() -> anyhow::Result<()> {
    use libc::{c_char, c_int};

    let args: Vec<String> = std::env::args().collect();
    let c_args: Result<Vec<std::ffi::CString>, _> = args
        .into_iter()
        .map(|s| std::ffi::CString::new(s))
        .collect();
    let c_args = c_args?;
    let c_ptrs: Vec<*const c_char> = c_args.iter().map(|s| s.as_ptr() as *const c_char).collect();

    let argc = c_ptrs.len() as c_int;
    let argv = c_ptrs.as_ptr();

    unsafe {
        c::wasm_opt_main(argc, argv);
    }

    drop(c_ptrs);
    drop(c_args);

    Ok(())
}
