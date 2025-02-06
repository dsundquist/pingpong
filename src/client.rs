use pingpong::ping_pong_client::PingPongClient;
use pingpong::Ping;
use tonic::Request;
use tokio::time::{sleep, Duration};
use tonic::transport::{ClientTlsConfig, Channel};
// use tokio_rustls::ClientTlsConfig;

pub mod pingpong {
    tonic::include_proto!("pingpong");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
   
    let tls_config = ClientTlsConfig::new()
        .domain_name("pingpong.gotestserver.com")
        .with_native_roots();

    let channel = Channel::from_static("https://pingpong.gotestserver.com")
        .tls_config(tls_config)?
        .connect()
        .await?;

    let mut client = PingPongClient::new(channel);
    
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