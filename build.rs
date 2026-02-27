fn main() {
    // Only compile proto files when the "grpc" feature is enabled
    if std::env::var("CARGO_FEATURE_GRPC").is_ok() {
        println!("cargo:rerun-if-changed=proto/duckling.proto");
        println!("cargo:rerun-if-changed=proto");

        tonic_build::configure()
            .build_server(true)
            .build_client(true)
            .compile_protos(&["proto/duckling.proto"], &["proto"])
            .unwrap_or_else(|e| {
                panic!(
                    "Failed to compile proto files: {}\n\
                     Hint: install protoc — on Debian/Ubuntu:\n  \
                       apt-get install protobuf-compiler\n\
                     Or download from https://github.com/protocolbuffers/protobuf/releases",
                    e
                )
            });
    }
}
