use tonic::{Request, Response, Status, transport::Server};

pub mod tonic_auth {
    tonic::include_proto!("tonic_auth");
}

use tonic_auth::greeter_server::{Greeter, GreeterServer};
use tonic_auth::{HelloReply, HelloRequest};

#[derive(Debug, Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        println!("Got a request: {:?}", request);

        let reply = tonic_auth::HelloReply {
            message: format!("Hello {}!", request.into_inner().name),
        };

        Ok(Response::new(reply))
    }
}

fn check_auth(req: Request<()>) -> Result<Request<()>, Status> {
    use tonic::metadata::MetadataValue;
    let token: MetadataValue<_> = "Bearer some-secret-token".parse().unwrap();

    match req.metadata().get("authorization") {
        Some(t) if token == t => Ok(req),
        _ => Err(Status::unauthenticated("No valid auth token")),
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let greeter = MyGreeter::default();

    let svc = GreeterServer::with_interceptor(greeter, check_auth);

    Server::builder().add_service(svc).serve(addr).await?;

    Ok(())
}
