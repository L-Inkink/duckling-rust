//! gRPC server binary for rustling
//!
//! Start with:
//!   cargo run --features grpc --bin grpc_server
//!   cargo run --features grpc --bin grpc_server -- --addr 0.0.0.0:50051
//!
//! Test with grpcurl:
//!   grpcurl -plaintext -d '{"text":"42","locale":"en"}' localhost:50051 duckling.Parser/Parse

use rustling::server::grpc::{GrpcAppState, ParserService};
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    // Parse address from args or use default
    let addr = std::env::args()
        .skip_while(|a| a != "--addr")
        .nth(1)
        .unwrap_or_else(|| "0.0.0.0:50051".to_string())
        .parse()?;

    log::info!("gRPC server listening on {}", addr);

    let state = GrpcAppState::new();
    let service = ParserService::new(state);

    Server::builder()
        .add_service(service.into_server())
        .serve(addr)
        .await?;

    Ok(())
}
