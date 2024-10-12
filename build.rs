use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-search=native=/usr/local/lib");
    println!("cargo:rustc-link-lib=dylib=crossdb");
    println!("cargo:rustc-link-arg=-Wl,-rpath=/usr/local/lib");
    println!("cargo:rustc-link-arg=-Wl,--no-as-needed");
    println!("cargo:rerun-if-changed=wrapper.h");

    // Explicitly link against libcrossdb.so
    println!("cargo:rustc-link-lib=crossdb");

    // Add this line to ensure the library is linked
    println!("cargo:rustc-flags=-L /usr/local/lib -l crossdb");

    // Debug prints
    println!("cargo:warning=Library search path: /usr/local/lib");
    println!("cargo:warning=Library name: crossdb");

    // Generate bindings
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .allowlist_file("/usr/local/include/crossdb.h")
        .blocklist_type("__mbstate_t")
        .blocklist_type("mbstate_t")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    // More debug prints
    println!("cargo:warning=Bindings generated at: {:?}", out_path.join("bindings.rs"));
}
