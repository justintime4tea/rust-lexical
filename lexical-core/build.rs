use rustc_version::version;

fn main() {
    // TARGET
    // ------

    // We need to optimize limb size for performance.
    // Only have optimized 64-bit instructions on certain architectures.
    // See `lexical-core/src/atof/algorithm/math.rs` for detailed
    // instructions of architecture instruction support for 64-bit
    // mathematical operations.

    // https://github.com/rust-lang/cargo/issues/4302#issuecomment-316482399
    let limb_64_archs = ["aarch64", "mips64", "powerpc64", "x86_64"];
    let limb_width_64 = match std::env::var("CARGO_CFG_TARGET_ARCH") {
        Ok(arch) => limb_64_archs.contains(&&*arch),
        _ => false,
    };
    if limb_width_64 {
        println!("cargo:rustc-cfg=limb_width_64");
    } else {
        println!("cargo:rustc-cfg=limb_width_32");
    }

    // We also need to know whether we can use const fn
    // in match or loop statements. Drop this when we
    // drop support for Rustc versions < 1.46.0.
    let rustc = version().unwrap();
    if (rustc.major, rustc.minor) >= (1, 46) {
        println!("cargo:rustc-cfg=has_const_if");
        println!("cargo:rustc-cfg=has_const_match");
    }
    if (rustc.major, rustc.minor) >= (1, 50) {
        println!("cargo:rustc-cfg=has_slice_fill");
    }
}
