fn main() -> Result<(), Box<dyn std::error::Error>> {
    let protoc_path = protoc_bin_vendored::protoc_bin_path()
        .expect("protoc-bin-vendored: failed to locate bundled protoc binary");

    std::env::set_var("PROTOC", protoc_path);

    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .out_dir("src/proto_gen")
        .compile_protos(&["proto/edisondb.proto"], &["proto/"])
        .expect("tonic_build: failed to compile proto/edisondb.proto");

    println!("cargo:rerun-if-changed=proto/edisondb.proto");
    println!("cargo:rerun-if-changed=build.rs");

    Ok(())
}
