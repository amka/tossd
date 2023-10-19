use log::debug;
use sea_orm::{DatabaseConnection, TryIntoModel};
use tonic::{Request, Response, Status};
use tonic::codegen::tokio_stream::wrappers::ReceiverStream;

use crate::agreements::{Agreement, AgreementAcceptance, AgreementAcceptanceResponse, GetAgreementRequest};
use crate::agreements::agreements_service_server::AgreementsService;
use crate::repository::agreements::AgreementsRepository;

pub struct Agreementer {
    pub connection: DatabaseConnection,
}

#[tonic::async_trait]
impl AgreementsService for Agreementer {
    async fn create_agreement(&self, request: Request<Agreement>) -> Result<Response<Agreement>, Status> {
        debug!("Called create_agreement");
        todo!();
        // let conn = &self.connection;
        //
        // if let Some(inserted) = AgreementsRepository::add(conn, request.into_inner())
        //     .await
        //     .ok() {
        //     let agreement = Agreement::from_model(inserted.try_into_model().unwrap());
        //
        //     Ok(Response::new(agreement))
        // } else {
        //     Err(Status::new(
        //         tonic::Code::Aborted,
        //         "Could not insert Agreement",
        //     ))
        // }
    }

    async fn update_agreement(&self, request: Request<Agreement>) -> Result<Response<Agreement>, Status> {
        debug!("Called update_agreement");
        todo!()
    }

    async fn delete_agreement(&self, request: Request<Agreement>) -> Result<Response<Agreement>, Status> {
        debug!("Called delete_agreement");
        todo!()
    }

    async fn get_agreement(&self, request: Request<GetAgreementRequest>) -> Result<Response<Agreement>, Status> {
        debug!("Called get_agreement");
        todo!();
        // let conn = &self.connection;
        // let id = request.into_inner().id;
        //
        // if let Some(model) = AgreementsRepository::find_by_id(conn, id)
        //     .await
        //     .ok().unwrap() {
        //     let agreement = Agreement::from_model(model);
        //
        //     Ok(Response::new(agreement))
        // } else {
        //     Err(Status::new(
        //         tonic::Code::Aborted,
        //         "Could not find Agreement with id ".to_owned() + &id.to_string(),
        //     ))
        // }
    }

    type GetAgreementAcceptancesStream = ReceiverStream<Result<AgreementAcceptance, Status>>;

    async fn get_agreement_acceptances(&self, request: Request<Agreement>) -> Result<Response<Self::GetAgreementAcceptancesStream>, Status> {
        debug!("Called get_agreement_acceptances");
        todo!()
    }

    async fn set_agreement_acceptance(&self, request: Request<AgreementAcceptance>) -> Result<Response<AgreementAcceptanceResponse>, Status> {
        debug!("Called set_agreement_acceptance");
        todo!()
    }
}
