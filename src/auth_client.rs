pub mod tonic_auth {
    tonic::include_proto!("tonic_auth");
}

use tonic::Request;
use tonic::metadata::MetadataValue;
use tonic::transport::Channel;
use tonic_auth::HelloRequest;
use tonic_auth::greeter_client::GreeterClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Build the client as a channel with a token and interceptor
    let channel = Channel::from_static("http://[::1]:50051").connect().await?;
    let token: MetadataValue<_> = "Bearer some-secret-token".parse()?;
    let mut client = GreeterClient::with_interceptor(channel, move |mut req: Request<()>| {
        req.metadata_mut().insert("authorization", token.clone());
        Ok(req)
    });

    let request = tonic::Request::new(HelloRequest {
        name: "Tonic".into(),
    });

    let response = client.say_hello(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
