fn main() {
    println!("cargo:rustc-cfg=feature=\"build-time\"");
    prost_build::Config::new()
        .out_dir("src/pb")
        .compile_protos(&["abi.proto"], &["."])
        .unwrap();
}
