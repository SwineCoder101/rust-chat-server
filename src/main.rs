use anyhow::Result;
use iroh::{Endpoint, SecretKey};

#[tokio::main]
async fn main() -> Result<()> {
    // Generate a secret key. This is the source of
    // identity for your endpoint. If you want to have
    // the same identity each time you open the app,
    // you would need to store and load it each time.
    let secret_key = SecretKey::generate(&mut rand::rng());

    // Create an endpoint.
    // By default we turn on our n0 discovery services.
    //  This allows you to
    // dial by `EndpointId`, and allows you to be
    // dialed by `EndpointId`.
    let endpoint = Endpoint::builder()
        // Pass in your secret key. If you don't pass
        // in a secret key a new one will be generated
        // for you each time.
        .secret_key(secret_key)
        // Bind the endpoint to the socket.
        .bind()
        .await?;

    println!("> our endpoint id: {}", endpoint.id());

    Ok(())
}
