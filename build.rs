fn main() {
    #[cfg(target_os = "linux")]
    {
        use cmake;
        let dst = cmake::Config::new("peloader")
            .define("_GNU_SOURCE", "")
            .cflag("-ggdb3 -std=gnu99 -fshort-wchar -Wno-multichar -mstackrealign")
            .pic(false)
            .build();

        println!("cargo:rustc-link-search=native={}", dst.display());
        let dst2 = cmake::Config::new("intercept/libdisasm")
            .define("_GNU_SOURCE", "")
            .cflag("-ggdb3 -std=gnu99 -fshort-wchar -Wno-multichar -mstackrealign")
            .pic(false)
            .build();

        println!("cargo:rustc-link-search=native={}", dst2.display());

        println!("cargo:rustc-link-lib=static:+whole-archive=peloader");
        println!("cargo:rustc-link-lib=static:+whole-archive=disasm");
        println!("cargo:rustc-link-arg=-m32");
    }
}
