pub fn exit(exit_code: u32) -> ! {
    inner::exit(exit_code);
    unreachable!()
}

pub fn relaunch() {
    inner::relaunch();
}

mod inner {
    use wasm_bindgen::prelude::wasm_bindgen;

    #[wasm_bindgen(module = "/dist/process.js")]
    extern "C" {
        pub fn exit(exitCode: u32);
        pub fn relaunch();
    }
}
