use cmake;
// use std::env;
// use std::path::PathBuf;

fn main() {
    let dst = cmake::Config::new("peloader")
        .define("_GNU_SOURCE", "")
        .cflag("-ggdb3 -std=gnu99 -fshort-wchar -Wno-multichar -mstackrealign")
        .pic(false)
        .build();

    println!("cargo:rustc-link-search=native={}", dst.display());
    cmake::Config::new("intercept/libdisasm")
        .define("_GNU_SOURCE", "")
        .cflag("-ggdb3 -std=gnu99 -fshort-wchar -Wno-multichar -mstackrealign")
        .pic(false)
        .build();

    // println!("cargo:rustc-link-search=native={}", dst2.display());

    println!("cargo:rustc-link-lib=static:+whole-archive=peloader");
    println!("cargo:rustc-link-lib=static=disasm");
    println!("cargo:rustc-link-arg=m32");

    // let bindings = bindgen::Builder::default()
    //     .header("peloader/pe_linker.h")
    //     .parse_callbacks(Box::new(bindgen::CargoCallbacks))
    //     .generate()
    //     .expect("Unable to generate bindings");
    // let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    // bindings
    //     .write_to_file(out_path.join("bindings.rs"))
    //     .expect("Couldn't write bindings!");
}
