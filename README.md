# libdeflater

> Rust bindings to [libdeflate](https://github.com/ebiggers/libdeflate), a high-performance
> library for working with gzip/zlib/deflate data.

[![Crates.io](https://img.shields.io/crates/v/libdeflater.svg?maxAge=2592000)](https://crates.io/crates/libdeflater)
[![Documentation](https://docs.rs/libdeflater/badge.svg)](https://docs.rs/libdeflater)

```
libdeflater = "1.25.2"
```

`libdeflater` is a thin wrapper library around [libdeflate](https://github.com/ebiggers/libdeflate). Libdeflate 
is optimal in applications that have the input data up-front, or when (large) input datasets can be split 
into smaller chunks (e.g. genomic [bam](https://samtools.github.io/hts-specs/SAMv1.pdf) files, some object stores,
specialized backends, game netcode packets).

This is a thin library around libdeflate that:

- Builds [libdeflate](https://github.com/ebiggers/libdeflate) from source (see 
  [libdeflate-sys](libdeflate-sys)'s [build.rs](libdeflate-sys/build.rs) file)

- Binds to libdeflate's C API with rust bindings (see [lib.rs](src/lib.rs))

- Contains high-level integration tests to ensure the bindings work (see [integration_test.rs](tests/integration_test.rs))

- Contains [usage examples](examples) and a [benchmark suite](benches). The benchmark
  suite indicates a 2-3x speedup accross the Calgary and Canterbury corpuses (see [Benchmarks](#benchmarks) 
  section)

⚠️ **Warning**: [libdeflate](https://github.com/ebiggers/libdeflate) is best-suited for *specialized*
                use-cases where you know the rough size range of your input up-front. You should use
                something like [flate2](https://github.com/alexcrichton/flate2-rs) if you want a
                general-purpose deflate library that supports streaming.

# Examples

Example source [here](examples). To run the examples:

```bash
cargo run --example gz_compress.rs
cargo run --example gz_decompress.rs
```


# Benchmarks

Benchmark data is from both the [Calgary Corpus](https://en.wikipedia.org/wiki/Calgary_corpus), and the
[Canterbury Corpus](http://corpus.canterbury.ac.nz/resources/cantrbry.zip). The
benchmark tables below were made with this set of steps:

```bash
wget http://www.data-compression.info/files/corpora/largecalgarycorpus.zip
unzip -d bench_data largecalgarycorpus.zip
wget http://corpus.canterbury.ac.nz/resources/cantrbry.zip
unzip -d bench_data cantrbry.zip

# runs benchmarks against all files in `bench_data`
cargo bench
scripts/process-bench.rb encode
scripts/process-bench.rb decode
```

### Compression

Avg. speedup (on this corpus) is around 2-3x

```
bench           size [KB]    speedup    flate2 [us]    libdeflate [us]
alice29.txt     152          3.1        5636           1821
asyoulik.txt    125          3.1        4911           1584
bib             111          2.9        3278           1133
book1           768          3.2        32697          10377
book2           610          2.8        19780          6975
cp.html         24           2.3        394            170
fields.c        11           2.4        155            65
geo             102          7.1        7717           1082
grammar.lsp     3            2.0        38             19
kennedy.xls     1029         7.3        46598          6427
lcet10.txt      426          3.0        14924          4931
news            377          2.5        10160          4052
obj1            21           2.6        385            149
obj2            246          3.5        7771           2218
paper1          53           2.4        1312           543
paper2          82           2.7        2608           955
paper3          46           2.5        1303           513
paper4          13           2.2        226            102
paper5          11           2.1        182            88
paper6          38           2.3        848            367
pic             513          3.8        7508           1990
plrabn12.txt    481          3.4        22527          6698
progc           39           2.4        882            361
progl           71           2.8        1553           559
progp           49           2.6        904            346
ptt5            513          3.8        7389           1964
sum             38           3.8        1124           297
trans           93           2.5        1595           650
xargs.1         4            1.9        40             21
```

### Decompression

Avg. speedup (on this corpus) is around 2x.

```
bench           size [KB]    speedup    flate2 [us]    libdeflate [us]
alice29.txt     152          3.0        338            114
asyoulik.txt    125          2.8        300            106
bib             111          3.2        240            76
book1           768          2.5        1906           768
book2           610          2.7        1376           501
cp.html         24           2.0        31             16
fields.c        11           2.1        15             7
geo             102          2.3        359            160
grammar.lsp     3            1.8        7              4
kennedy.xls     1029         1.4        1241           911
lcet10.txt      426          2.8        919            325
news            377          2.4        969            400
obj1            21           1.9        41             21
obj2            246          2.5        558            220
paper1          53           3.2        109            34
paper2          82           3.1        182            58
paper3          46           3.0        100            34
paper4          13           1.9        22             12
paper5          11           1.9        21             11
paper6          38           3.0        75             25
pic             513          3.1        617            198
plrabn12.txt    481          2.5        1183           472
progc           39           3.1        76             25
progl           71           3.5        103            30
progp           49           3.0        65             22
ptt5            513          3.1        616            197
sum             38           2.5        75             31
trans           93           3.7        131            36
xargs.1         4            1.8        9              5

```

### Benchmark Notes

- All benchmarks are single-threaded

- IO/streaming overhead is not considered. The decompressed data is
  read into memory before performing the comparison

- Comparison made against `flate2` with no feature flags (i.e. `miniz`
  implementation). `flate2` was chosen because it's the most
  popular.

- Comparisons with other `flate2` backends are available on the
  `bench-flate2-miniz-oxide` and `bench-flate2-zlib` branches. The
  `zlib` backend is ~8 % faster on some of the corpus entries.

- Compression performed with default compression setting in both cases

- Corpus entries were compressed with `flate2` at default compression
  level

### Compile-time features

You can enable the following features to customise the build:
 - `use_rust_alloc`: Makes libdeflate use Rust's allocator instead of the libc one.
   This is useful when Rust is preconfigured to use a
   [custom global allocator](https://doc.rust-lang.org/edition-guide/rust-2018/platform-and-target-support/global-allocators.html)
   (e.g. pool-based, or a tracking one, or something else entirely).
 - `freestanding`: Builds libdeflate in a freestanding mode (no reliance on libc).
   This is useful for targets that don't have a C stdlib (e.g. `wasm32-unknown-unknown`)
   as otherwise they would fail to compile. Implies `use_rust_alloc`.
