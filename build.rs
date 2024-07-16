use cmake::Config;

#[allow(unused_variables)]
fn main() {
    // #[cfg(not(target_family = "unix"))]
    // compile_error!("this only supports unix systems");

    // fix windows issues
    std::fs::write(
        "coordgenlibs/CoordgenConfig.hpp",
        "#define EXPORT_COORDGEN\n",
    )
    .unwrap();

    #[cfg(target_os = "macos")]
    let cpp_lib = "c++";
    #[cfg(not(target_os = "macos"))]
    let cpp_lib = "stdc++";

    let dst = Config::new(".")
        .define("COORDGEN_RIGOROUS_BUILD", "OFF")
        .define("COORDGEN_BUILD_TESTS", "OFF")
        .define("COORDGEN_BUILD_EXAMPLE", "OFF")
        .define("COORDGEN_BUILD_SHARED_LIBS", "OFF")
        .define("CMAKE_BUILD_TYPE", "Release")
        .define("CMAKE_INSTALL_LIBDIR", "lib")
        .uses_cxx11()
        .build();

    println!(
        "cargo:rustc-link-search=native={}",
        dst.join("lib").display()
    );
    println!("cargo:rustc-link-lib=static=coordgen");
    println!("cargo:rustc-link-lib=static=wrappedcoordgen");
    #[cfg(not(target_family = "windows"))]
    println!("cargo:rustc-link-lib={}", cpp_lib);
}
