use cmake::Config;

#[allow(unused_variables)]
fn main() {
    // #[cfg(not(target_family = "unix"))]
    // compile_error!("this only supports unix systems");

    // fix windows issues
    // std::fs::write(
    //     "coordgenlibs/CoordgenConfig.hpp",
    //     "#define EXPORT_COORDGEN\n",
    // )
    // .unwrap();
    std::fs::copy("modified-config.hpp", "coordgenlibs/CoordgenConfig.hpp").unwrap();

    #[cfg(target_os = "macos")]
    let cpp_lib = "c++";
    #[cfg(not(target_os = "macos"))]
    let cpp_lib = "stdc++";

    let mut cfg = Config::new(".");
    cfg.define("COORDGEN_RIGOROUS_BUILD", "OFF")
        .define("COORDGEN_BUILD_TESTS", "OFF")
        .define("COORDGEN_BUILD_EXAMPLE", "OFF")
        .define("CMAKE_BUILD_TYPE", "Release")
        .define("CMAKE_INSTALL_LIBDIR", "lib")
        .define("CMAKE_INSTALL_BINDIR", "lib")
        .uses_cxx11();

    #[cfg(not(target_env = "msvc"))]
    cfg.define("COORDGEN_BUILD_SHARED_LIBS", "OFF");

    let dst = cfg.build();

    println!(
        "cargo:rustc-link-search=native={}",
        dst.join("lib").display()
    );
    #[cfg(target_env = "msvc")]
    println!("cargo:rustc-link-lib=dylib=coordgen");
    #[cfg(not(target_env = "msvc"))]
    println!("cargo:rustc-link-lib=static=coordgen");
    println!("cargo:rustc-link-lib=static=wrappedcoordgen");
    #[cfg(not(target_env = "msvc"))]
    println!("cargo:rustc-link-lib={}", cpp_lib);
}
