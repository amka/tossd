use std::net::SocketAddr;
use log::info;
use tonic::transport::Server;

use agreements::agreements_service_server::AgreementsServiceServer;
use services::agreements::Agreementer;

mod services;

pub mod agreements {
    tonic::include_proto!("agreements"); // The string specified here must match the proto package name
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let addr = std::env::var("APP_ADDRESS").unwrap_or("127.0.0.1:50051".to_string()).parse()?;
    let agreementer = Agreementer::default();

    info!("Starting gRPC Server at {}", addr);

    Server::builder()
        .add_service(AgreementsServiceServer::new(agreementer))
        .serve(addr)
        .await?;

    Ok(())
}