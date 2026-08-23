#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(bad_style)]

#[repr(C)]
pub struct libdeflate_compressor { _unused : [ u8 ; 0 ] , }
#[repr(C)]
pub struct libdeflate_decompressor { _unused : [ u8 ; 0 ] , }

#[repr(C)]
pub struct libdeflate_options {
    pub sizeof_options: usize,
    pub malloc_func: Option<unsafe extern "C" fn(arg1: usize) -> *mut ::std::os::raw::c_void>,
    pub free_func: Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void)>,
}

pub const libdeflate_result_LIBDEFLATE_SUCCESS: libdeflate_result = 0;
pub const libdeflate_result_LIBDEFLATE_BAD_DATA: libdeflate_result = 1;
pub const libdeflate_result_LIBDEFLATE_SHORT_OUTPUT: libdeflate_result = 2;
pub const libdeflate_result_LIBDEFLATE_INSUFFICIENT_SPACE: libdeflate_result = 3;
pub type libdeflate_result = ::std::os::raw::c_uint;

extern "C" {
    pub fn libdeflate_alloc_decompressor() -> *mut libdeflate_decompressor;
    pub fn libdeflate_alloc_decompressor_ex(options: *const libdeflate_options) -> *mut libdeflate_decompressor;
    pub fn libdeflate_free_decompressor(decompressor: *mut libdeflate_decompressor);

    pub fn libdeflate_gzip_decompress(decompressor: *mut libdeflate_decompressor,
                                      in_: *const ::std::os::raw::c_void,
                                      in_nbytes: usize ,
                                      out: *mut ::std::os::raw::c_void,
                                      out_nbytes_avail: usize,
                                      actual_out_nbytes_ret: *mut usize) -> libdeflate_result;

    pub fn libdeflate_gzip_decompress_ex(decompressor: *mut libdeflate_decompressor,
                                         in_: *const ::std::os::raw::c_void,
                                         in_nbytes: usize,
                                         out: *mut ::std::os::raw::c_void,
                                         out_nbytes_avail: usize,
                                         actual_in_nbytes_ret: *mut usize,
                                         actual_out_nbytes_ret: *mut usize) -> libdeflate_result;

    pub fn libdeflate_zlib_decompress(decompressor: *mut libdeflate_decompressor,
                                      in_: *const ::std::os::raw::c_void,
                                      in_nbytes: usize,
                                      out: *mut ::std::os::raw::c_void,
                                      out_nbytes_avail: usize,
                                      actual_out_nbytes_ret: *mut usize) -> libdeflate_result;

    pub fn libdeflate_zlib_decompress_ex(decompressor: *mut libdeflate_decompressor,
                                         in_: *const ::std::os::raw::c_void,
                                         in_nbytes: usize,
                                         out: *mut ::std::os::raw::c_void,
                                         out_nbytes_avail: usize,
                                         actual_in_nbytes_ret: *mut usize,
                                         actual_out_nbytes_ret: *mut usize) -> libdeflate_result;

    pub fn libdeflate_deflate_decompress(decompressor: *mut libdeflate_decompressor,
                                         in_: *const ::std::os::raw::c_void,
                                         in_nbytes: usize,
                                         out: *mut ::std::os::raw::c_void,
                                         out_nbytes_avail: usize,
                                         actual_out_nbytes_ret: *mut usize) -> libdeflate_result;
    
    pub fn libdeflate_deflate_decompress_ex(decompressor: *mut libdeflate_decompressor, 
                                            in_: *const ::std::os::raw::c_void,
                                            in_nbytes: usize,
                                            out: *mut ::std::os::raw::c_void,
                                            out_nbytes_avail: usize,
                                            actual_in_nbytes_ret: *mut usize,
                                            actual_out_nbytes_ret: *mut usize) -> libdeflate_result;

    pub fn libdeflate_alloc_compressor(compression_level: ::std::os::raw::c_int) -> *mut libdeflate_compressor;
    
    pub fn libdeflate_alloc_compressor_ex(compression_level: ::std::ffi::c_int,
                                          options: *const libdeflate_options) -> *mut libdeflate_compressor;

    pub fn libdeflate_deflate_compress_bound(compressor: *mut libdeflate_compressor,
                                             in_nbytes: usize) -> usize;


    pub fn libdeflate_deflate_compress(compressor: *mut libdeflate_compressor,
                                       in_: *const ::std::os::raw::c_void,
                                       in_nbytes: usize,
                                       out: *mut ::std::os::raw::c_void,
                                       out_nbytes_avail: usize) -> usize;

    pub fn libdeflate_zlib_compress_bound(compressor: *mut libdeflate_compressor,
                                          in_nbytes: usize) -> usize;

    pub fn libdeflate_zlib_compress(compressor: *mut libdeflate_compressor,
                                    in_: *const ::std::os::raw::c_void,
                                    in_nbytes: usize,
                                    out: *mut ::std::os::raw::c_void,
                                    out_nbytes_avail: usize) -> usize;

    pub fn libdeflate_gzip_compress_bound(compressor: *mut libdeflate_compressor,
                                          in_nbytes: usize) -> usize;

    pub fn libdeflate_gzip_compress(compressor: *mut libdeflate_compressor,
                                    in_: *const ::std::os::raw::c_void,
                                    in_nbytes: usize,
                                    out: *mut ::std::os::raw::c_void,
                                    out_nbytes_avail: usize ) -> usize;

    pub fn libdeflate_free_compressor (compressor : * mut libdeflate_compressor);

    pub fn libdeflate_crc32(crc32: u32,
                            buffer: *const ::std::os::raw::c_void,
                            len: usize) -> u32;

    pub fn libdeflate_adler32(adler32: u32,
                              buffer: *const ::std::os::raw::c_void,
                              len: usize) -> u32;

    pub fn libdeflate_set_memory_allocator(malloc_func: unsafe extern "C" fn(size: usize) -> *mut ::std::os::raw::c_void,
                                           free_func: unsafe extern "C" fn(ptr: *mut ::std::os::raw::c_void));
}

// Basic tests for Rust-to-C bindings. These tests are just for quick
// internal checks to make sure that the bindgen build script built
// something sane-looking. User-facing tests are in `tests/`
#[cfg(test)]
mod tests {
    use super::*;

    const MIN_COMP_LVL: i32 = 0;
    const MAX_COMP_LVL: i32 = 12;

    #[test]
    fn can_make_decompressor_at_each_compression_lvl() {
        unsafe {
            for lvl in MIN_COMP_LVL..MAX_COMP_LVL+1 {
                let ptr = libdeflate_alloc_compressor(lvl);

                assert!(!ptr.is_null());

                libdeflate_free_compressor(ptr);
            }
        }
    }

    #[test]
    fn can_make_decompressor_with_null_malloc_free() {
        unsafe {
            let options = libdeflate_options {
                sizeof_options: std::mem::size_of::<libdeflate_options>(),
                malloc_func: None,
                free_func: None,
            };
            let ptr = libdeflate_alloc_decompressor_ex(&options);

            assert!(!ptr.is_null());

            libdeflate_free_decompressor(ptr);
        }
    }

    #[test]
    fn making_compressor_with_negative_compression_lvl_fails() {
        unsafe {
            let ptr = libdeflate_alloc_compressor(-1);
            assert!(ptr.is_null());
        }
    }

    #[test]
    fn making_compressor_with_too_large_compression_lvl_fails() {
        unsafe {
            let ptr = libdeflate_alloc_compressor(MAX_COMP_LVL+1);

            assert!(ptr.is_null());
        }
    }

    #[test]
    fn can_use_compressor_to_compress_trivially_compressable_data() {
        unsafe {
            let in_data: [u8; 1<<16] = [0; 1<<16];
            let mut out_data: [u8; 1<<16] = [0; 1<<16];
            let compressor = libdeflate_alloc_compressor(MAX_COMP_LVL);
            let sz = libdeflate_deflate_compress(compressor,
                                                 in_data.as_ptr() as *const core::ffi::c_void,
                                                 in_data.len(),
                                                 out_data.as_mut_ptr() as *mut core::ffi::c_void,
                                                 out_data.len());
            assert_ne!(sz, 0);
            assert!(sz < 100);
            libdeflate_free_compressor(compressor);
        }
    }

    #[test]
    fn can_call_crc32() {
        let init = 0;
        let buf: [u8; 4] = [0x31, 0x33, 0x33, 0x37];

        unsafe {
            let ret = libdeflate_crc32(init,
                                       buf.as_ptr() as *const core::ffi::c_void,
                                       buf.len());

            assert_ne!(ret, init);
        }
    }

    #[test]
    fn can_call_adler32() {
        let init = 0;
        let buf: [u8; 4] = [0x31, 0x33, 0x33, 0x37];

        unsafe {
            let ret = libdeflate_adler32(init,
                                         buf.as_ptr() as *const core::ffi::c_void,
                                         buf.len());

            assert_ne!(ret, init);
        }
    }
}
