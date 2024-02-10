use crate::grpc::health::proto::{HealthCheckRequest, HealthCheckResponse};
use std::pin::Pin;
use tokio_stream::{Stream, StreamExt};
use tonic::{transport::Server, Request, Response, Status};

pub mod proto {
    tonic::include_proto!("grpc.health.v1");
}

type HealthCheckResponseStream =
    Pin<Box<dyn Stream<Item = Result<proto::HealthCheckResponse, Status>> + Send>>;

#[derive(Default)]
pub(crate) struct HealthService {}

#[tonic::async_trait]
impl proto::health_server::Health for HealthService {
    async fn check(
        &self,
        request: Request<HealthCheckRequest>,
    ) -> Result<Response<HealthCheckResponse>, Status> {
        todo!()
    }

    type WatchStream = HealthCheckResponseStream;

    async fn watch(
        &self,
        request: Request<HealthCheckRequest>,
    ) -> Result<Response<Self::WatchStream>, Status> {
        todo!()
    }
}
