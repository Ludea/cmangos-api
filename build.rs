use cmake::Config;

fn main() {
    tonic_build::compile_protos("proto/helloworld.proto").unwrap();
    let dst = Config::new("../mangos-classic")
        .define("BOOST_DIR", "../boost_1_88_0")
        .build();
}
