//! Direct bindings to `wasm-opt`.
//!
//! These are bindings to `wasm-opt`,
//! as built by the [`wasm-opt-sys`] crate.
//! The bindings are created by the [`cxx`] crate,
//! and all go through a custom C++ shim layer
//! that provides a `cxx`-compatible C++ API.
//!
//! Most users will not want to use this crate directly,
//! but instead the [`wasm-opt`] crate.
//!
//! [`wasm-opt-sys`]: https://docs.rs/wasm-opt-sys
//! [`cxx`]: https://docs.rs/cxx
//! [`wasm-opt`]: https://docs.rs/wasm-opt
//!
//! The version of `cxx` used by these bindings is
//! reexported here.

pub use cxx;

#[cxx::bridge(namespace = "Colors")]
pub mod colors {
    unsafe extern "C++" {
        include!("shims.h");

        fn setEnabled(enabled: bool);
    }
}

#[cxx::bridge(namespace = "wasm_shims")]
pub mod wasm {
    unsafe extern "C++" {
        include!("shims.h");

        fn validateWasm(wasm: Pin<&mut Module>) -> bool;
    }

    unsafe extern "C++" {
        include!("shims.h");

        type Module;

        fn newModule() -> UniquePtr<Module>;
    }

    unsafe extern "C++" {
        include!("shims.h");

        type ModuleReader;

        fn newModuleReader() -> UniquePtr<ModuleReader>;

        fn setDebugInfo(self: Pin<&mut Self>, debug: bool);

        fn setDwarf(self: Pin<&mut Self>, dwarf: bool);

        fn readText(
            self: Pin<&mut Self>,
            filename: &CxxString,
            wasm: Pin<&mut Module>,
        ) -> Result<()>;

        fn readBinary(
            self: Pin<&mut Self>,
            filename: &CxxString,
            wasm: Pin<&mut Module>,
            sourceMapFilename: &CxxString,
        ) -> Result<()>;

        fn read(
            self: Pin<&mut Self>,
            filename: &CxxString,
            wasm: Pin<&mut Module>,
            sourceMapFilename: &CxxString,
        ) -> Result<()>;
    }

    unsafe extern "C++" {
        include!("shims.h");

        type ModuleWriter;

        fn newModuleWriter() -> UniquePtr<ModuleWriter>;

        fn setDebugInfo(self: Pin<&mut Self>, debug: bool);

        fn setSourceMapFilename(self: Pin<&mut Self>, source_map_filename: &CxxString);

        fn setSourceMapUrl(self: Pin<&mut Self>, source_map_url: &CxxString);

        fn writeText(
            self: Pin<&mut Self>,
            wasm: Pin<&mut Module>,
            filename: &CxxString,
        ) -> Result<()>;

        fn writeBinary(
            self: Pin<&mut Self>,
            wasm: Pin<&mut Module>,
            filename: &CxxString,
        ) -> Result<()>;
    }

    unsafe extern "C++" {
        include!("shims.h");

        fn getRegisteredNames() -> UniquePtr<CxxVector<CxxString>>;

        fn getPassDescription(name: &CxxString) -> UniquePtr<CxxString>;

        fn isPassHidden(name: &CxxString) -> bool;
    }

    unsafe extern "C++" {
        include!("shims.h");

        type InliningOptions;

        fn newInliningOptions() -> UniquePtr<InliningOptions>;

        fn setAlwaysInlineMaxSize(self: Pin<&mut Self>, size: u32);

        fn setOneCallerInlineMaxSize(self: Pin<&mut Self>, size: u32);

        fn setFlexibleInlineMaxSize(self: Pin<&mut Self>, size: u32);

        fn setAllowFunctionsWithLoops(self: Pin<&mut Self>, allow: bool);

        fn setPartialInliningIfs(self: Pin<&mut Self>, number: u32);
    }

    unsafe extern "C++" {
        include!("shims.h");

        type PassOptions;

        fn newPassOptions() -> UniquePtr<PassOptions>;

        fn setValidate(self: Pin<&mut Self>, validate: bool);

        fn setValidateGlobally(self: Pin<&mut Self>, validate: bool);

        fn setOptimizeLevel(self: Pin<&mut Self>, level: i32);

        fn setShrinkLevel(self: Pin<&mut Self>, level: i32);

        fn setInliningOptions(self: Pin<&mut Self>, inlining: UniquePtr<InliningOptions>);

        fn setTrapsNeverHappen(self: Pin<&mut Self>, ignoreTraps: bool);

        fn setLowMemoryUnused(self: Pin<&mut Self>, memoryUnused: bool);

        fn setFastMath(self: Pin<&mut Self>, fastMath: bool);

        fn setZeroFilledMemory(self: Pin<&mut Self>, zeroFilledMemory: bool);

        fn setDebugInfo(self: Pin<&mut Self>, debugInfo: bool);
    }

    unsafe extern "C++" {
        include!("shims.h");

        type PassRunner<'wasm>;

        fn newPassRunner<'wasm>(wasm: Pin<&'wasm mut Module>) -> UniquePtr<PassRunner<'wasm>>;

        fn newPassRunnerWithOptions<'wasm>(
            wasm: Pin<&'wasm mut Module>,
            options: UniquePtr<PassOptions>,
        ) -> UniquePtr<PassRunner<'wasm>>;

        fn add(self: Pin<&mut Self>, pass_name: &CxxString);

        fn addDefaultOptimizationPasses(self: Pin<&mut Self>);

        fn run(self: Pin<&mut Self>);

        fn passRemovesDebugInfo(name: &CxxString) -> bool;
    }

    unsafe extern "C++" {
        include!("shims.h");

        fn checkInliningOptionsDefaults(inlining_options: UniquePtr<InliningOptions>) -> bool;
    }
}

/// Hack to establish linkage to wasm-opt-sys.
///
/// See docs for `wasm_opt_sys::init`.
#[doc(hidden)]
pub fn init() -> anyhow::Result<()> {
    wasm_opt_sys::init();

    Ok(())
}
