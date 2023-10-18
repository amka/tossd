use tonic::transport::Server;

use agreements::agreements_service_server::AgreementsServiceServer;
use services::agreements::Agreementer;

mod services;

pub mod agreements {
    tonic::include_proto!("agreements"); // The string specified here must match the proto package name
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:50051".parse()?;
    let agreementer = Agreementer::default();

    Server::builder()
        .add_service(AgreementsServiceServer::new(agreementer))
        .serve(addr)
        .await?;

    Ok(())
}