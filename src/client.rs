use pingpong::ping_pong_client::PingPongClient;
use pingpong::Ping;
use tonic::Request;
use tokio::time::{sleep, Duration};

pub mod pingpong {
    tonic::include_proto!("pingpong");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = PingPongClient::connect("http://[::1]:50051").await?;
    
    let mut count = 0;
    
    loop {
        count += 1;
        let request = Request::new(Ping {
            message: format!("Ping #{}", count),
        });

        let response = client.send_ping(request).await?;
        println!("Received: {}", response.into_inner().message);

        sleep(Duration::from_secs(1)).await;
    }
}