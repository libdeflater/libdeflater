use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    let dst = PathBuf::from(env::var_os("OUT_DIR").unwrap());

    #[cfg(feature = "dynamic")]
    if libdeflate_dynamic() { return; }
    else {
        println!("cargo:warning=Dynamic linking failed; falling back to static build.");
    }

    let mut build = cc::Build::new();

    build
        .files(&[
            "libdeflate/lib/arm/cpu_features.c",
            "libdeflate/lib/x86/cpu_features.c",
            "libdeflate/lib/adler32.c",
            "libdeflate/lib/crc32.c",
            "libdeflate/lib/deflate_compress.c",
            "libdeflate/lib/deflate_decompress.c",
            "libdeflate/lib/gzip_compress.c",
            "libdeflate/lib/gzip_decompress.c",
            "libdeflate/lib/utils.c",
            "libdeflate/lib/zlib_compress.c",
            "libdeflate/lib/zlib_decompress.c",
        ])
        .include("libdeflate")
        .warnings(false)
        .out_dir(dst.join("lib"));

    if cfg!(feature = "freestanding") && !build.get_compiler().is_like_msvc() {
        build
            .flag("-ffreestanding")
            .flag("-nostdlib")
            .define("FREESTANDING", None);
    }

    build.compile("deflate");

    let src = Path::new("libdeflate");
    let include = dst.join("include");
    fs::create_dir_all(&include).unwrap();
    fs::copy(src.join("libdeflate.h"), include.join("libdeflate.h")).unwrap();
    println!("cargo:root={}", dst.display());
    println!("cargo:include={}", include.display());
}

#[cfg(feature = "dynamic")]
/// # Link to System Copy?
///
/// Returns `true` if `pkg-config` was able to link against the system's
/// copy of `libdeflate`, mooting the need to build it from source.
fn libdeflate_dynamic() -> bool {
    /// # Strip Prefix and Trailing Whitespace.
    ///
    /// This method strips the `prefix` _and_ any whitespace that follows,
    /// returning the remainder if both `prefix` and whitespace were present.
    ///
    /// (`libdeflate` doesn't use whitespace consistently; we shouldn't assume
    /// spaces will always be spaces, tabs always tabs.)
    fn strip_prefix_and_ws<'a>(line: &'a str, prefix: &'static str) -> Option<&'a str> {
        // First strip the prefix.
        let line = line.strip_prefix(prefix)?;

        // Grab the current length, then trim.
        let len_pretrim = line.len();
        let line = line.trim_start();

        // Return the remainder if anything was trimmed.
        if line.len() == len_pretrim { None }
        else { Some(line) }
    }

    /// # Parse Vendored Libdeflate Version.
    ///
    /// Parse and return the `MAJOR.MINOR` version string from `libdeflate.h`.
    fn libdeflate_version() -> Option<String> {
        let raw = fs::read_to_string("libdeflate/libdeflate.h").ok()?;
        let mut major = Option::<u16>::None;
        let mut minor = Option::<u16>::None;

        // Focus on lines beginning `#define`.
        for line in raw.lines().filter_map(|line| strip_prefix_and_ws(line, "#define")) {
            if let Some(rest) = strip_prefix_and_ws(line, "LIBDEFLATE_VERSION_MAJOR") {
                // What remains should be a number.
                let num = rest.parse::<u16>().ok()?;
                major.replace(num);
                if minor.is_some() { break; }
                continue;
            }

            if let Some(rest) = strip_prefix_and_ws(line, "LIBDEFLATE_VERSION_MINOR") {
                // What remains should be a number.
                let num = rest.parse::<u16>().ok()?;
                minor.replace(num);
                if major.is_some() { break; }
            }
        }

        let major = major?;
        let minor = minor?;
        Some(format!("{major}.{minor}"))
    }

    // Determine the version we'd otherwise be linking against.
    if let Some(version) = libdeflate_version() {
        // The crate version should begin the same way; if not pop an alert so
        // the maintainer can fix that.
        if ! env!("CARGO_PKG_VERSION").starts_with(&format!("{version}.")) {
            println!(
                "cargo:warning=Crate ({}) and vendor ({version}) versions mismatch.",
                env!("CARGO_PKG_VERSION"),
            );
        }

        // Look for a matching system copy!
        pkg_config::Config::new()
            .print_system_libs(false)
            .cargo_metadata(true)
            .exactly_version(&version)
            .probe("libdeflate")
            .is_ok()
    }
    else {
        // Version parsing shouldn't ever fail; pop a warning to alert the
        // maintainer if it does!
        println!("cargo:warning=Failed to parse vendored MAJOR/MINOR versions.");
        false
    }
}
