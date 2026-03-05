use std::env;
use std::path::PathBuf;

use libbpf_cargo::SkeletonBuilder;

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let bpf_dir = manifest_dir.join("src").join("bpf");
    let src = bpf_dir.join("blocker.bpf.c");
    let out = bpf_dir.join("blocker.skel.rs");

    println!("cargo:rerun-if-changed={}", src.display());

    SkeletonBuilder::new()
        .source(&src)
        .build_and_generate(&out)
        .expect("failed to build and generate BPF skeleton");
}

