use std::env;
use std::path::PathBuf;

fn main() {
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg("-I./ffmpeg-win64/include")
        .allowlist_function("av_.*") // All functions starting with 'av_'
        .allowlist_type("AV.*") // All types starting with 'AV'
        .allowlist_var("AV_.*") // All variables starting with 'AV_'
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}