use tonic::{transport::Server, Request, Response, Status};
use tonic::codegen::tokio_stream::wrappers::ReceiverStream;

use agreements::agreements_service_server::{AgreementsService, AgreementsServiceServer};
use agreements::{Agreement, AgreementAcceptance, AgreementAcceptanceResponse, AgreementStatus};

pub mod agreements {
    tonic::include_proto!("agreements"); // The string specified here must match the proto package name
}

#[derive(Default)]
pub struct Agreementer;

#[tonic::async_trait]
impl AgreementsService for Agreementer {
    async fn create_agreement(&self, request: Request<Agreement>) -> Result<Response<Agreement>, Status> {
        todo!()
    }

    async fn update_agreement(&self, request: Request<Agreement>) -> Result<Response<Agreement>, Status> {
        todo!()
    }

    async fn delete_agreement(&self, request: Request<Agreement>) -> Result<Response<Agreement>, Status> {
        todo!()
    }

    async fn get_agreement(&self, request: Request<Agreement>) -> Result<Response<Agreement>, Status> {
        todo!()
    }

    type GetAgreementAcceptancesStream = ReceiverStream<Result<AgreementAcceptance, Status>>;

    async fn get_agreement_acceptances(&self, request: Request<Agreement>) -> Result<Response<Self::GetAgreementAcceptancesStream>, Status> {
        todo!()
    }

    async fn set_agreement_acceptance(&self, request: Request<AgreementAcceptance>) -> Result<Response<AgreementAcceptanceResponse>, Status> {
        todo!()
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:50051".parse()?;
    let greeter = Agreementer::default();

    Server::builder()
        .add_service(AgreementsServiceServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}