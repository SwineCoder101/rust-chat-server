use anyhow::Result;
use iroh::protocol::Router;
use iroh::Endpoint;
use iroh_gossip::{net::Gossip, proto::TopicId};

#[tokio::main]
async fn main() -> Result<()> {
    let endpoint = Endpoint::bind().await?;

    println!("> our endpoint id: {}", endpoint.id());
    let gossip = Gossip::builder().spawn(endpoint.clone());

    let router = Router::builder(endpoint.clone())
        .accept(iroh_gossip::ALPN, gossip.clone())
        .spawn();

    // Create a new topic.
    let id = TopicId::from_bytes(rand::random());
    let endpoint_ids = vec![];


    let topic = gossip.subscribe(id, endpoint_ids).await?;


    let (sender, _receiver) = topic.split();


    sender.broadcast("sup".into()).await?;

    router.shutdown().await?;

    Ok(())
}
