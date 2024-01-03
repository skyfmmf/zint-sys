// Taken from: https://rust-lang.github.io/rust-bindgen/library-usage.html

#[cfg(feature = "vendored")]
use cmake::Config;
use std::env;
use std::path::PathBuf;

fn main() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    #[cfg(feature = "vendored")]
    {
        let zint_path = build_libzint();
        println!("cargo:rustc-link-search=native={}/lib", zint_path.display());
        println!("cargo:rustc-link-lib=static=zint");
    }

    #[cfg(not(feature = "vendored"))]
    {
        println!("cargo:rustc-link-lib=zint");
    }

    println!("cargo:rerun-if-changed=wrapper.h");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg(format!("-I{}/include", out_path.display()))
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

#[cfg(feature = "vendored")]
fn build_libzint() -> PathBuf {
    let dst = Config::new("src/zint")
        .define("ZINT_SHARED", "OFF")
        .define("ZINT_STATIC", "ON")
        .define("ZINT_USE_PNG", "ON")
        .define("ZINT_USE_QT", "OFF")
        .define("ZINT_UNINSTALL", "OFF")
        .define("ZINT_FRONTEND", "OFF")
        .build();

    return dst;
}
