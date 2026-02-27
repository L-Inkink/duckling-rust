fn main() {
    // Only compile proto files when the "grpc" feature is enabled
    if std::env::var("CARGO_FEATURE_GRPC").is_ok() {
        tonic_build::configure()
            .build_server(true)
            .build_client(true)
            .compile_protos(&["proto/duckling.proto"], &["proto"])
            .expect("Failed to compile proto files");
    }
}
