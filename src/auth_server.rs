use tonic::{Request, Response, Status, transport::Server};

pub mod tonic_auth {
    tonic::include_proto!("tonic_auth");
}

use tonic_auth::greeter_server::{Greeter, GreeterServer};
use tonic_auth::{HelloReply, HelloRequest};
use tracing_subscriber::fmt::format::FmtSpan;

#[derive(Debug, Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    #[tracing::instrument]
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
    // Setup tracing
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        // Use a more compact, abbreviated log format
        .compact()
        // Display source code file paths
        .with_file(true)
        // Display source code line numbers
        .with_line_number(true)
        // Display the thread ID an event was recorded on
        .with_thread_ids(true)
        // Don't display the event's target (module path)
        .with_target(false)
        // Include per-span timings
        .with_span_events(FmtSpan::CLOSE)
        // Build the subscriber
        .finish();

    // Set the subscriber as the default
    tracing::subscriber::set_global_default(subscriber).unwrap();

    let addr = "[::1]:50051".parse()?;
    let greeter = MyGreeter::default();

    let svc = GreeterServer::with_interceptor(greeter, check_auth);

    Server::builder()
        .trace_fn(|_| tracing::info_span!("auth_server"))
        .add_service(svc)
        .serve(addr)
        .await?;

    Ok(())
}
