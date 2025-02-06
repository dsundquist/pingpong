use tonic::{transport::Server, Request, Response, Status};
use pingpong::ping_pong_server::{PingPong, PingPongServer};
use pingpong::{Ping, Pong};

pub mod pingpong {
    tonic::include_proto!("pingpong");
}

#[derive(Default)]
pub struct MyPingPong;

#[tonic::async_trait]
impl PingPong for MyPingPong {
    async fn send_ping(&self, request: Request<Ping>) -> Result<Response<Pong>, Status> {
        let ping = request.into_inner();
        println!("Received Ping: {}", ping.message);

        let reply = Pong {
            message: format!("Pong: {}", ping.message),
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let service = MyPingPong::default();

    println!("Starting gRPC server on {}", addr);

    Server::builder()
        .add_service(PingPongServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}