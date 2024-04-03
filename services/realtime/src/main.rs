use tonic::{transport::Server, Request, Response, Status};

pub mod realtime {
    tonic::include_proto!("realtime");
}

#[derive(Debug, Default)]
struct MyPing;

#[tonic::async_trait]
impl realtime::ping_server::Ping for MyPing {
    async fn say_ping(
        &self,
        request: Request<realtime::PingMessage>, // Accept request of type HelloRequest
    ) -> Result<Response<realtime::PongReply>, Status> { // Return an instance of type HelloReply
        println!("Got a request: {:?}", request);

        let reply = realtime::PongReply {
            msg: request.into_inner().msg
        };

        Ok(Response::new(reply)) // Send back our formatted greeting
    }

}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let pong = MyPing::default();

    Server::builder()
        .add_service(realtime::ping_server::PingServer::new(pong))
        .serve(addr)
        .await?;

    Ok(())
}