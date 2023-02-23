fn main() {
    prost_build::Config::new()
        .out_dir("src/pb")
        .compile_protos(&["abi.proto"], &["."])
        .unwrap();

    prost_build::Config::new()
        .out_dir("src/data")
        .compile_protos(&["type.proto"], &["."])
        .unwrap()
}
