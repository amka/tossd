use log::info;
use sea_orm::Database;
use tonic::transport::Server;

// use agreements::agreements_service_server::AgreementsServiceServer;
use migration::{Migrator, MigratorTrait};
use services::agreements::Agreementer;

use crate::agreements::agreement_service_server::AgreementServiceServer;

mod services;
mod models;
mod repository;

pub mod agreements {
    tonic::include_proto!("agreement_service"); // The string specified here must match the proto package name
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // establish database connection
    let connection = Database::connect(&database_url).await?;
    Migrator::up(&connection, None).await?;

    let addr = std::env::var("APP_ADDRESS").unwrap_or("127.0.0.1:50051".to_string()).parse()?;
    let agreementer = Agreementer { connection };

    info!("Starting gRPC Server at {}", addr);

    Server::builder()
        .add_service(AgreementServiceServer::new(agreementer))
        .serve(addr)
        .await?;

    Ok(())
}
