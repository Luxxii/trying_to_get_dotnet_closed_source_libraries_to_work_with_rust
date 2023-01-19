use cc;
use pkg_config;


fn main() {

    // println!("cargo:rustc-link-lib=dylib=/usr/lib");
    // println!("cargo:rustc-link-lib=dylib=/usr/include/mono-2.0");
    // println!("cargo:rustc-link-lib=dylib=/usr/lib/libmonosgen-2.0.so.1");
    println!("cargo:rustc-flags=-L/usr/lib/pkgconfig/../../include/mono-2.0"); 
    println!("cargo:rustc-link-lib=/home/luxii/git/christians_solution/src/librawrrRcpp.a"); 
    pkg_config::probe_library("mono-2").unwrap();
    cc::Build::new()
        .file("src/rawrrRcpp.cpp")
        .include("/usr/lib/pkgconfig/../../include/mono-2.0") // retrieved via  pkg-config --cflags --libs mono-2 
        .opt_level(2)
        .flag("-fstack-protector-strong")
        .debug(true)
        .compile("rawrrRcpp");
    println!("cargo:rerun-if-changed=src/rawrrRcpp.cpp");
}
