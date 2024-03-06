#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#[allow(warnings)]

mod ffi {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

#[cfg(test)]
mod tests {
    use super::ffi;

    #[test]
    fn test_ffmpeg_bindings() {
        // Perform your test assertions here
        // For example, you can check if a specific function or constant is defined
        assert!(ffi::AVCodecID_AV_CODEC_ID_H264  != ffi::AVCodecID_AV_CODEC_ID_AAC);
    }
}
