pub async fn exit(exit_code: u32) -> ! {
    inner::exit(exit_code).await;
    unreachable!()
}

pub fn relaunch() {
    inner::relaunch();
}

mod inner {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen(module = "/src/process.js")]
    extern "C" {
        pub async fn exit(exitCode: u32);
        pub fn relaunch();
    }
}
