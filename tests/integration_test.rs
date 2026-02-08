extern crate libdeflater;

use std::fs::File;
use std::io::Read;
use std::vec::Vec;
use std::error::Error;
use std::thread;
use libdeflater::{Compressor, CompressionLvl, CompressionError, Decompressor, DecompressionError, CompressionLvlError};
use flate2;



// decompression

#[test]
fn test_can_create_new_decompressor() {
    Decompressor::new();
}

fn read_fixture(pth: &str) -> Vec<u8> {
    let mut f = File::open(pth).unwrap();
    let mut content = Vec::new();
    f.read_to_end(&mut content).unwrap();
    content
}

fn read_fixture_content() -> Vec<u8> {
    read_fixture("tests/hello")
}

fn fixture_content_size() -> usize {
    read_fixture_content().len()
}

fn read_fixture_gz() -> Vec<u8> {
    read_fixture("tests/hello.gz")
}

fn read_fixture_gz_with_bad_magic_num() -> Vec<u8> {
    let mut data = read_fixture_gz();

    // beats having to have an extra fixture file
    data[0] = 0x00;

    data
}

fn read_fixture_gz_with_bad_crc32() -> Vec<u8> {
    let mut data = read_fixture_gz();
    let crc32_start = data.len()-8;

    // beats having to have an extra fixture file
    data[crc32_start] = data[crc32_start] + 1;

    data
}

fn read_fixture_gz_with_bad_isize() -> Vec<u8> {
    let mut data = read_fixture_gz();
    let isize_start = data.len()-4;

    // beats having to have an extra fixture file
    data[isize_start] = data[isize_start] + 1;

    data
}

fn read_fixture_zlib() -> Vec<u8> {
    read_fixture("tests/hello.zz")
}

fn read_fixture_zlib_with_bad_cmf_field() -> Vec<u8> {
    const BAD_CM: u8 = 9;  // RFC1950: val == 8 for deflate
    const BAD_CINFO: u8 = 8;  // RFC1950: vals > 7 disallowed
    const BAD_CMF: u8 = ((BAD_CM << 4) & 0xf0) | (BAD_CINFO & 0x0f);

    let mut data = read_fixture_zlib();

    data[0] = BAD_CMF;

    data
}

fn read_fixture_zlib_with_bad_adler32_checksum() -> Vec<u8> {
    let mut data = read_fixture_zlib();
    let adler32_start = data.len() - 4;

    data[adler32_start] = data[adler32_start] + 1;

    data
}

fn read_fixture_deflate() -> Vec<u8> {
    read_fixture("tests/hello.deflate")
}

#[test]
fn test_decompression_error_derives_error() {
    let bd = DecompressionError::BadData;
    let _e = (&bd) as &dyn Error;
}

#[test]
fn test_can_send_decompressor_to_another_thread() {
    // note: this is a compile-time test: it just ensures that a
    // `Decompressor` can be sent between threads easily (i.e. that
    // `Decompressor` implements `Send`)

    let mut decompressor = Decompressor::new();
    let t = thread::spawn(move || {
        let content = read_fixture_gz();
        let mut decompressed = Vec::new();
        decompressed.resize(fixture_content_size(), 0);

        decompressor.gzip_decompress(&content, &mut decompressed).unwrap();
    });
    t.join().unwrap()
}

#[test]
fn test_can_send_compressor_to_another_thread() {
    // note: this is a compile-time test: it just ensures that a
    // Compressor can be sent between threads easily (i.e.  that
    // `Compressor` implements `Send`)

    let mut compressor = Compressor::new(CompressionLvl::default());
    let t = thread::spawn(move || {
        let in_data = read_fixture_content();
        let mut out_data = Vec::new();
        let out_max_sz = compressor.deflate_compress_bound(in_data.len());
        out_data.resize(out_max_sz, 0);

        compressor.deflate_compress(&in_data, &mut out_data).unwrap();
    });
    t.join().unwrap()
}

// gz decompression

#[test]
fn test_calling_gzip_decompress_with_valid_args_works() {
    let mut decompressor = Decompressor::new();
    let content = read_fixture_gz();
    let mut decompressed = Vec::new();
    decompressed.resize(fixture_content_size(), 0);

    decompressor.gzip_decompress(&content, &mut decompressed).unwrap();
}

#[test]
fn test_calling_gzip_decompress_with_undersized_outbuf_returns_insufficient_space() {
    let result = {
        let mut decompressor = Decompressor::new();
        let content = read_fixture_gz();
        let mut decompressed = Vec::new();
        decompressor.gzip_decompress(&content, &mut decompressed)
    };

    assert_eq!(result.unwrap_err(), DecompressionError::InsufficientSpace);
}

#[test]
fn test_calling_gzip_decompress_with_valid_buf_fills_buf_with_expected_content() {
    let decompressed = {
        let mut decompressor = Decompressor::new();
        let content = read_fixture_gz();
        let mut decompressed = Vec::new();
        decompressed.resize(fixture_content_size(), 0);
        decompressor.gzip_decompress(&content, &mut decompressed).unwrap();
        decompressed
    };

    let expected_content = read_fixture_content();

    assert_eq!(decompressed, expected_content);
}

#[test]
fn test_calling_gzip_decompress_with_oversized_buf_returns_correct_size() {
    const OVERSIZED_FACTOR: usize = 2;

    let sz = {
        let mut decompressor = Decompressor::new();
        let content = read_fixture_gz();
        let mut decompressed = Vec::new();
        decompressed.resize(OVERSIZED_FACTOR*fixture_content_size(), 0);
        let sz = decompressor.gzip_decompress(&content, &mut decompressed).unwrap();
        sz
    };

    assert_eq!(sz, fixture_content_size());
}

#[test]
fn test_calling_gzip_decompress_with_bad_magic_num_returns_bad_data() {
    let mut decompressor = Decompressor::new();
    let content = read_fixture_gz_with_bad_magic_num();
    let mut decompressed = Vec::new();
    decompressed.resize(fixture_content_size(), 0);
    let result = decompressor.gzip_decompress(&content, &mut decompressed);

    assert_eq!(result.unwrap_err(), DecompressionError::BadData);
}

#[test]
fn test_calling_gzip_decompress_with_corrupted_crc32_returns_bad_data() {
    let mut decompressor = Decompressor::new();
    let content = read_fixture_gz_with_bad_crc32();
    let mut decompressed = Vec::new();
    decompressed.resize(fixture_content_size(), 0);
    let result = decompressor.gzip_decompress(&content, &mut decompressed);

    assert_eq!(result.unwrap_err(), DecompressionError::BadData);
}

#[test]
fn test_calling_gzip_decompress_with_corrupted_isize_returns_bad_data() {
    let mut decompressor = Decompressor::new();
    let content = read_fixture_gz_with_bad_isize();
    let mut decompressed = Vec::new();
    decompressed.resize(fixture_content_size(), 0);
    let result = decompressor.gzip_decompress(&content, &mut decompressed);

    assert_eq!(result.unwrap_err(), DecompressionError::BadData);
}


// zlib decompression

#[test]
fn test_calling_zlib_decompress_with_valid_args_works() {
    let mut decompressor = Decompressor::new();
    let content = read_fixture_zlib();
    let mut decompressed = Vec::new();
    decompressed.resize(fixture_content_size(), 0);

    decompressor.zlib_decompress(&content, &mut decompressed).unwrap();
}

#[test]
fn test_calling_zlib_decompress_with_undersized_outbuf_returns_insufficient_space() {
    let result = {
        let mut decompressor = Decompressor::new();
        let content = read_fixture_zlib();
        let mut decompressed = Vec::with_capacity(0);

        decompressor.zlib_decompress(&content, &mut decompressed)
    };

    assert_eq!(result.unwrap_err(), DecompressionError::InsufficientSpace);
}

#[test]
fn test_calling_zlib_decompress_with_valid_buf_fills_buf_with_expected_content() {
    let decompressed_content = {
        let mut decompressor = Decompressor::new();
        let content = read_fixture_zlib();
        let mut decompressed = Vec::new();
        decompressed.resize(fixture_content_size(), 0);
        decompressor.zlib_decompress(&content, &mut decompressed).unwrap();
        decompressed
    };

    let expected_content = read_fixture_content();

    assert_eq!(decompressed_content, expected_content);
}

#[test]
fn test_calling_zlib_decompress_with_oversized_buf_returns_correct_size() {
    const OVERSIZED_FACTOR: usize = 2;

    let sz = {
        let mut decompressor = Decompressor::new();
        let content = read_fixture_zlib();
        let mut decompressed = Vec::new();
        decompressed.resize(OVERSIZED_FACTOR*fixture_content_size(), 0);
        let sz = decompressor.zlib_decompress(&content, &mut decompressed).unwrap();
        sz
    };

    assert_eq!(sz, fixture_content_size());
}

#[test]
fn test_calling_zlib_decompress_with_bad_cmf_field_returns_bad_data() {
    let ret = {
        let mut decompressor = Decompressor::new();
        let content = read_fixture_zlib_with_bad_cmf_field();
        let mut decompressed = Vec::new();
        decompressed.resize(fixture_content_size(), 0);

        decompressor.zlib_decompress(&content, &mut decompressed)
    };

    assert_eq!(ret.unwrap_err(), DecompressionError::BadData);
}

#[test]
fn test_calling_zlib_decompress_with_bad_adler32_checksum_returns_bad_data() {
    let ret = {
        let mut decompressor = Decompressor::new();
        let content = read_fixture_zlib_with_bad_adler32_checksum();
        let mut decompressed = Vec::new();
        decompressed.resize(fixture_content_size(), 0);

        decompressor.zlib_decompress(&content, &mut decompressed)
    };

    assert_eq!(ret.unwrap_err(), DecompressionError::BadData);
}


// DEFLATE decompression

#[test]
fn test_calling_deflate_decompress_with_valid_args_works() {
    let mut decompressor = Decompressor::new();
    let content = read_fixture_deflate();
    let mut decompressed = Vec::new();
    decompressed.resize(fixture_content_size(), 0);

    decompressor.deflate_decompress(&content, &mut decompressed).unwrap();
}

#[test]
fn test_calling_deflate_decompress_with_undersized_outbuf_returns_insufficient_space() {
    let result = {
        let mut decompressor = Decompressor::new();
        let content = read_fixture_deflate();
        let mut decompressed = Vec::with_capacity(0);

        decompressor.deflate_decompress(&content, &mut decompressed)
    };

    assert_eq!(result.unwrap_err(), DecompressionError::InsufficientSpace);
}

#[test]
fn test_calling_deflate_decompress_with_valid_buf_fills_buf_with_expected_content() {
    let decompressed_content = {
        let mut decompressor = Decompressor::new();
        let content = read_fixture_deflate();
        let mut decompressed = Vec::new();
        decompressed.resize(fixture_content_size(), 0);
        decompressor.deflate_decompress(&content, &mut decompressed).unwrap();
        decompressed
    };

    let expected_content = read_fixture_content();

    assert_eq!(decompressed_content, expected_content);
}

#[test]
fn test_calling_deflate_decompress_with_oversized_buf_returns_correct_size() {
    const OVERSIZED_FACTOR: usize = 2;

    let sz = {
        let mut decompressor = Decompressor::new();
        let content = read_fixture_deflate();
        let mut decompressed = Vec::new();
        decompressed.resize(OVERSIZED_FACTOR*fixture_content_size(), 0);
        let sz = decompressor.deflate_decompress(&content, &mut decompressed).unwrap();
        sz
    };

    assert_eq!(sz, fixture_content_size());
}



// compression

#[test]
fn test_compression_lvl_new_returns_ok_for_0() {
    // see issuse #21 - some use-cases require uncompressed blocks
    CompressionLvl::new(0).unwrap();
}

#[test]
fn test_compression_lvl_new_returns_ok_for_7() {
    CompressionLvl::new(7).unwrap();
}

#[test]
fn test_compression_lvl_new_returns_err_for_13() {
    // If libdeflate changes in the future this test will need to be
    // scrapped, but it's ok as a smoke test
    let ret = CompressionLvl::new(13).unwrap_err();

    assert_eq!(ret, CompressionLvlError::InvalidValue);
}

#[test]
fn test_compressor_with_fastest_compression_lvl_calls_with_no_panics() {
    Compressor::new(CompressionLvl::fastest());
}

#[test]
fn test_compressor_with_default_compression_lvl_calls_with_no_panics() {
    Compressor::new(CompressionLvl::default());
}

#[test]
fn test_compressor_with_best_compression_lvl_calls_with_no_panics() {
    Compressor::new(CompressionLvl::best());
}

#[test]
fn test_compression_iter_starts_at_compression_lvl_zero() {
    let val = CompressionLvl::iter().next().unwrap();
    assert_eq!(val, CompressionLvl::fastest());
    assert_eq!(val, CompressionLvl::new(0).unwrap());
}

#[test]
fn test_compressor_with_compression_lvl_can_be_called_with_all_lvls_with_no_panics() {
    for lvl in CompressionLvl::iter() {
        Compressor::new(lvl);
    }
}


// compress DEFLATE

#[test]
fn test_compressor_deflate_compress_bound_returns_nonzero_when_given_nonzero_input() {
    let returned_bound = {
        let mut compressor = Compressor::new(CompressionLvl::default());
        compressor.deflate_compress_bound(1)
    };

    assert_ne!(returned_bound, 0);
}

#[test]
fn test_compressor_deflate_compress_with_valid_args_does_not_panic() {
    let mut compressor = Compressor::new(CompressionLvl::default());
    let in_data = read_fixture_content();
    let mut out_data = Vec::new();
    let out_max_sz = compressor.deflate_compress_bound(in_data.len());
    out_data.resize(out_max_sz, 0);

    compressor.deflate_compress(&in_data, &mut out_data).unwrap();
}

#[test]
fn test_compressor_deflate_compress_with_undersized_buf_returns_insufficient_space() {
    let ret = {
        let mut compressor = Compressor::new(CompressionLvl::default());
        let in_data = read_fixture_content();
        let mut out_data = Vec::new();
        compressor.deflate_compress(&in_data, &mut out_data)
    };

    assert_eq!(ret.unwrap_err(), CompressionError::InsufficientSpace);
}

#[test]
fn test_compressor_deflate_compress_with_correct_args_returns_a_nonzero_size() {
    let ret = {
        let mut compressor = Compressor::new(CompressionLvl::default());
        let in_data = read_fixture_content();
        let mut out_data = Vec::new();
        let out_max_sz = compressor.deflate_compress_bound(in_data.len());
        out_data.resize(out_max_sz, 0);
        compressor.deflate_compress(&in_data, &mut out_data).unwrap()
    };

    assert_ne!(0, ret);
}

#[test]
fn test_compressor_deflate_compress_with_correct_args_resized_outbuf_to_return_val_produces_nonempty_nonzero_data() {
    // This is, give or take, essentially how users *should* use the
    // lib for raw DEFLATE compressions

    let compressed_data = {
        let mut compressor = Compressor::new(CompressionLvl::default());
        let in_data = read_fixture_content();
        let mut out_data = Vec::new();
        let out_max_sz = compressor.deflate_compress_bound(in_data.len());
        out_data.resize(out_max_sz, 0);
        let sz = compressor.deflate_compress(&in_data, &mut out_data).unwrap();
        out_data.resize(sz, 0);
        out_data
    };

    assert_ne!(compressed_data.len(), 0);
    assert!(compressed_data.iter().any(|byte| *byte != 0x00));
}


// compress zlib

#[test]
fn test_compressor_zlib_compress_bound_returns_nonzero_when_given_nonzero_input() {
    let returned_bound = {
        let mut compressor = Compressor::new(CompressionLvl::default());
        compressor.zlib_compress_bound(1)
    };

    assert_ne!(returned_bound, 0);
}

#[test]
fn test_compressor_compresss_zlib_with_valid_args_does_not_panic() {
    let mut compressor = Compressor::new(CompressionLvl::default());
    let in_data = read_fixture_content();
    let mut out_data = Vec::new();
    let out_max_sz = compressor.zlib_compress_bound(in_data.len());
    out_data.resize(out_max_sz, 0);

    compressor.zlib_compress(&in_data, &mut out_data).unwrap();
}

#[test]
fn test_compressor_zlib_compress_with_undersized_outbuf_returns_insufficient_space() {
    let ret = {
        let mut compressor = Compressor::new(CompressionLvl::default());
        let in_data = read_fixture_content();
        let mut out_data = Vec::new();
        compressor.zlib_compress(&in_data, &mut out_data)
    };

    assert_eq!(ret.unwrap_err(), CompressionError::InsufficientSpace);
}

#[test]
fn test_compressor_zlib_compress_with_correct_args_returns_nonzero_size() {
    let ret = {
        let mut compressor = Compressor::new(CompressionLvl::default());
        let in_data = read_fixture_content();
        let mut out_data = Vec::new();
        let out_max_sz = compressor.zlib_compress_bound(in_data.len());
        out_data.resize(out_max_sz, 0);

        compressor.zlib_compress(&in_data, &mut out_data).unwrap()
    };

    assert_ne!(ret, 0);
}

#[test]
fn test_compressor_zlib_compress_with_correct_args_resized_outbuf_to_return_val_produces_nonempty_nonzero_data() {
    // This is, give or take, essentially how users *should* use the
    // lib for zlib compressions

    let compressed_data = {
        let mut compressor = Compressor::new(CompressionLvl::default());
        let in_data = read_fixture_content();
        let mut out_data = Vec::new();
        let out_max_sz = compressor.zlib_compress_bound(in_data.len());
        out_data.resize(out_max_sz, 0);
        let sz = compressor.zlib_compress(&in_data, &mut out_data).unwrap();
        out_data.resize(sz, 0);
        out_data
    };

    assert_ne!(compressed_data.len(), 0);
    assert!(compressed_data.iter().any(|byte| *byte != 0x00));
}


// compress gzip

#[test]
fn test_compressor_gzip_compress_bound_returns_nonzero_when_given_nonzero_input() {
    let returned_bound = {
        let mut compressor = Compressor::new(CompressionLvl::default());
        compressor.gzip_compress_bound(1)
    };

    assert_ne!(returned_bound, 0);
}

#[test]
fn test_compressor_compresss_gzip_with_valid_args_does_not_panic() {
    let mut compressor = Compressor::new(CompressionLvl::default());
    let in_data = read_fixture_content();
    let mut out_data = Vec::new();
    let out_max_sz = compressor.gzip_compress_bound(in_data.len());
    out_data.resize(out_max_sz, 0);

    compressor.gzip_compress(&in_data, &mut out_data).unwrap();
}

#[test]
fn test_compressor_gzip_compress_with_undersized_outbuf_returns_insufficient_space() {
    let ret = {
        let mut compressor = Compressor::new(CompressionLvl::default());
        let in_data = read_fixture_content();
        let mut out_data = Vec::new();
        compressor.gzip_compress(&in_data, &mut out_data)
    };

    assert_eq!(ret.unwrap_err(), CompressionError::InsufficientSpace);
}

#[test]
fn test_compressor_gzip_compress_with_correct_args_returns_nonzero_size() {
    let ret = {
        let mut compressor = Compressor::new(CompressionLvl::default());
        let in_data = read_fixture_content();
        let mut out_data = Vec::new();
        let out_max_sz = compressor.gzip_compress_bound(in_data.len());
        out_data.resize(out_max_sz, 0);

        compressor.gzip_compress(&in_data, &mut out_data).unwrap()
    };

    assert_ne!(ret, 0);
}

#[test]
fn test_compressor_gzip_compress_with_correct_args_resized_outbuf_to_return_val_produces_nonempty_nonzero_data() {
    // This is, give or take, essentially how users *should* use the
    // lib for gzip compressions

    let compressed_data = {
        let mut compressor = Compressor::new(CompressionLvl::default());
        let in_data = read_fixture_content();
        let mut out_data = Vec::new();
        let out_max_sz = compressor.gzip_compress_bound(in_data.len());
        out_data.resize(out_max_sz, 0);
        let sz = compressor.gzip_compress(&in_data, &mut out_data).unwrap();
        out_data.resize(sz, 0);
        out_data
    };

    assert_ne!(compressed_data.len(), 0);
    assert!(compressed_data.iter().any(|byte| *byte != 0x00));
}


// compress + decompress (full-cycle tests)

#[test]
fn test_zlib_compress_then_zlib_decompress_works_and_produces_the_same_input_data() {
    let input_data = read_fixture_content();
    
    let compression_buf = {
        let mut compressor = Compressor::new(CompressionLvl::default());
        let mut compression_buf = Vec::new();
        let compressed_max_sz = compressor.zlib_compress_bound(input_data.len());
        compression_buf.resize(compressed_max_sz, 0);
        let compressed_sz = compressor.zlib_compress(&input_data, &mut compression_buf).unwrap();
        compression_buf.resize(compressed_sz, 0);
        compression_buf
    };

    let decompressed_buf = {
        let mut decompressor = Decompressor::new();
        let mut decompressed_buf = Vec::new();
        decompressed_buf.resize(input_data.len(), 0);
        let decompressed_sz = decompressor.zlib_decompress(&compression_buf, &mut decompressed_buf).unwrap();

        assert_eq!(decompressed_sz, input_data.len());
        decompressed_buf
    };

    assert_eq!(input_data, decompressed_buf);
}

#[test]
fn test_deflate_compress_then_deflate_decompress_works_and_produces_the_same_input_data() {
    let input_data = read_fixture_content();
    
    let compression_buf = {
        let mut compressor = Compressor::new(CompressionLvl::default());
        let mut compression_buf = Vec::new();
        let compressed_max_sz = compressor.deflate_compress_bound(input_data.len());
        compression_buf.resize(compressed_max_sz, 0);
        let compressed_sz = compressor.deflate_compress(&input_data, &mut compression_buf).unwrap();
        compression_buf.resize(compressed_sz, 0);
        compression_buf
    };

    let decompressed_buf = {
        let mut decompressor = Decompressor::new();
        let mut decompressed_buf = Vec::new();
        decompressed_buf.resize(input_data.len(), 0);
        let decompressed_sz = decompressor.deflate_decompress(&compression_buf, &mut decompressed_buf).unwrap();

        assert_eq!(decompressed_sz, input_data.len());
        decompressed_buf
    };

    assert_eq!(input_data, decompressed_buf);
}

#[test]
fn test_gzip_compress_then_gzip_decompress_works_and_produces_the_same_input_data() {
    let input_data = read_fixture_content();
    
    let compression_buf = {
        let mut compressor = Compressor::new(CompressionLvl::default());
        let mut compression_buf = Vec::new();
        let compressed_max_sz = compressor.gzip_compress_bound(input_data.len());
        compression_buf.resize(compressed_max_sz, 0);
        let compressed_sz = compressor.gzip_compress(&input_data, &mut compression_buf).unwrap();
        compression_buf.resize(compressed_sz, 0);
        compression_buf
    };

    let decompressed_buf = {
        let mut decompressor = Decompressor::new();
        let mut decompressed_buf = Vec::new();
        decompressed_buf.resize(input_data.len(), 0);
        let decompressed_sz = decompressor.gzip_decompress(&compression_buf, &mut decompressed_buf).unwrap();

        assert_eq!(decompressed_sz, input_data.len());
        decompressed_buf
    };

    assert_eq!(input_data, decompressed_buf);
}


// crc32

#[test]
fn test_use_crc32_reader_to_compute_crc32_of_fixture_returns_same_crc32_as_flate2() {
    // This assumes that flate2's crc32 implementation returns a
    // correct value, which is a pretty safe assumption.

    let input_data = read_fixture_content();

    let flate2_crc32 = {
        let mut crc = flate2::Crc::new();
        crc.update(&input_data);
        crc.sum()
    };

    let libdeflate_crc32 = {
        let mut crc = libdeflater::Crc::new();
        crc.update(&input_data);
        crc.sum()
    };

    assert_eq!(flate2_crc32, libdeflate_crc32);
}

#[test]
fn test_use_crc32_convenience_method_returns_same_crc32_as_flate2() {
    // This assumes that flate2's crc32 implementation returns a
    // correct value, which is a pretty safe assumption.

    let input_data = read_fixture_content();

    let flate2_crc32 = {
        let mut crc = flate2::Crc::new();
        crc.update(&input_data);
        crc.sum()
    };

    let libdeflate_crc32 = libdeflater::crc32(&input_data);

    assert_eq!(flate2_crc32, libdeflate_crc32);
}

#[test]
fn test_crc_with_initial_can_be_used_to_store_and_combine_crc_checksums() {
    let input_data = read_fixture_content();
    let first_half_slice = &input_data[0..input_data.len()/2];
    let second_half_slice = &input_data[input_data.len()/2..];

    let first_half_checksum = libdeflater::crc32(&first_half_slice);
    let pieced_together_checksum = {
        let mut checksummer = libdeflater::Crc::with_initial(first_half_checksum);
        checksummer.update(&second_half_slice);
        checksummer.sum()
    };

    let end_to_end_checksum = libdeflater::crc32(&input_data);

    assert_eq!(pieced_together_checksum, end_to_end_checksum);
}

// adler32
#[test]
fn test_use_adler32_reader_to_compute_adler32_of_fixture_returns_same_adler32_as_adler32_crate(
) {
    // This assumes that adler32 crate implementation returns a
    // correct value, which is a pretty safe assumption.

    let input_data = read_fixture_content();

    let crate_adler32 = {
        let mut adler32 = adler32::RollingAdler32::new();
        adler32.update_buffer(&input_data);
        adler32.update_buffer(&input_data);
        adler32.hash()
    };

    let libdeflate_adler32 = {
        let mut adler32 = libdeflater::Adler32::new();
        adler32.update(&input_data);
        adler32.update(&input_data);
        adler32.sum()
    };

    assert_eq!(crate_adler32, libdeflate_adler32);
}

#[test]
fn test_use_adler32_convenience_method_returns_same_adler32_as_adler32_crate() {
    // This assumes that adler32 crate implementation returns a
    // correct value, which is a pretty safe assumption.

    let input_data = read_fixture_content();

    let crate_adler32 = {
        let mut adler32 = adler32::RollingAdler32::new();
        adler32.update_buffer(&input_data);
        adler32.hash()
    };

    let libdeflate_adler32 = libdeflater::adler32(&input_data);

    assert_eq!(crate_adler32, libdeflate_adler32);
}

#[test]
fn test_adler32_with_initial_can_be_used_to_store_and_combine_adler_checksums() {
    let input_data = read_fixture_content();
    let first_half_slice = &input_data[0..input_data.len()/2];
    let second_half_slice = &input_data[input_data.len()/2..];

    let first_half_checksum = libdeflater::adler32(&first_half_slice);
    let pieced_together_checksum = {
        let mut checksummer = libdeflater::Adler32::with_initial(first_half_checksum);
        checksummer.update(&second_half_slice);
        checksummer.sum()
    };

    let end_to_end_checksum = libdeflater::adler32(&input_data);

    assert_eq!(pieced_together_checksum, end_to_end_checksum);
}

