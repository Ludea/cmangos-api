fn main() {
    cxx_build::bridge("src/main.rs")
        .file("src/cmangos.cc")
        .compile("cmangos-api");

    println!("cargo:rerun-if-changed=src/main.rs");
    println!("cargo:rerun-if-changed=src/cmangos.cc");
}
